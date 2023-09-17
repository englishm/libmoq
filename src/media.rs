use anyhow::Context;
use moq_transport::model::{broadcast, segment, track};
use moq_transport::VarInt;
use mp4::{self, ReadBox};
use serde_json::json;
use std::collections::HashMap;
use std::io::Cursor;
use std::io::Read;
use std::os::raw::c_int;
use std::time;

use crate::MoqContext;

// Read a full MP4 atom into a vector.
pub fn read_atom<R: Read>(reader: &mut R) -> anyhow::Result<Vec<u8>> {
    // Read the 8 bytes for the size + type
    let mut buf = [0u8; 8];
    let bytes_read = reader.read_exact(&mut buf)?;

    // Convert the first 4 bytes into the size.
    let size = u32::from_be_bytes(buf[0..4].try_into()?) as u64;

    let mut raw = buf.to_vec();

    let mut limit = match size {
        // Runs until the end of the file.
        0 => reader.take(u64::MAX),

        // The next 8 bytes are the extended size to be used instead.
        1 => {
            reader.read_exact(&mut buf)?;
            let size_large = u64::from_be_bytes(buf);
            anyhow::ensure!(
                size_large >= 16,
                "impossible extended box size: {}",
                size_large
            );

            reader.take(size_large - 16)
        }

        2..=7 => {
            anyhow::bail!("impossible box size: {}", size)
        }

        size => reader.take(size - 8),
    };

    // check max bytes limit will read
    let target_size = limit.limit();
    println!("target_size: {}", target_size);

    // Append to the vector and return it.
    let read_bytes = limit.read_to_end(&mut raw)?;
    println!("read_bytes: {}", read_bytes);

    Ok(raw)
}

// TODO: Set up catalog and init tracks
pub fn init_tracks(moq_ctx: &mut MoqContext) -> Result<c_int, anyhow::Error> {
    let mut broadcast = moq_ctx.publisher.clone().unwrap();

    let mut buf = &moq_ctx.unread as &[u8];
    let size = buf.len() as c_int;
    dbg!(buf, size);

    let ftyp = read_atom(&mut buf)?;
    anyhow::ensure!(&ftyp[4..8] == b"ftyp", "expected ftyp atom");

    if (size - ftyp.len() as c_int) < 8 {
        // Don't try to read the moov atom if we don't have enough bytes
        return Ok(0);
    }

    let moov = read_atom(&mut buf)?;
    anyhow::ensure!(&moov[4..8] == b"moov", "expected moov atom");
    dbg!(moov.len());

    let mut init = ftyp;
    init.extend(&moov);

    let bytes_read = init.len();

    // We're going to parse the moov box.
    // We have to read the moov box header to correctly advance the cursor for the mp4 crate.
    let mut moov_reader = Cursor::new(&moov);
    let moov_header = mp4::BoxHeader::read(&mut moov_reader)?;

    // Parse the moov box so we can detect the timescales for each track.
    let moov = mp4::MoovBox::read_box(&mut moov_reader, moov_header.size)?;

    // Create the catalog track with a single segment.
    let mut init_track = broadcast.create_track("1.mp4")?;
    let mut init_segment = init_track.create_segment(segment::Info {
        sequence: VarInt::ZERO,
        priority: i32::MAX,
        expires: None,
    })?;

    init_segment.write_chunk(init.into())?;

    let mut tracks = HashMap::new();

    for trak in &moov.traks {
        let id = trak.tkhd.track_id;
        let name = id.to_string();

        let timescale = track_timescale(&moov, id);

        // Store the track publisher in a map so we can update it later.
        let track = broadcast.create_track(&name)?;
        let track = Track::new(track, timescale);
        tracks.insert(name, track);
    }

    let mut catalog = broadcast.create_track(".catalog")?;

    // Create the catalog track
    serve_catalog(&mut catalog, init_track.name.to_string(), &moov, &tracks)?;

    moq_ctx.tracks = tracks;

    Ok(bytes_read as c_int)
}

#[derive(Debug)]
pub struct Track {
    // The track we're producing
    track: track::Publisher,

    // The current segment
    segment: Option<segment::Publisher>,

    // The number of units per second.
    timescale: u64,

    // The number of segments produced.
    sequence: u64,
}

impl Track {
    fn new(track: track::Publisher, timescale: u64) -> Self {
        Self {
            track,
            sequence: 0,
            segment: None,
            timescale,
        }
    }

    pub fn header(&mut self, raw: Vec<u8>, fragment: Fragment) -> anyhow::Result<()> {
        if let Some(segment) = self.segment.as_mut() {
            if !fragment.keyframe {
                // Use the existing segment
                segment.write_chunk(raw.into())?;
                return Ok(());
            }
        }

        // Otherwise make a new segment

        // Compute the timestamp in milliseconds.
        // Overflows after 583 million years, so we're fine.
        let _timestamp: i32 = fragment
            .timestamp(self.timescale)
            .as_millis()
            .try_into()
            .context("timestamp too large")?;

        // Create a new segment.
        let mut segment = self.track.create_segment(segment::Info {
            sequence: VarInt::try_from(self.sequence).context("sequence too large")?,
            priority: i32::MAX, // TODO

            // Delete segments after 10s.
            expires: Some(time::Duration::from_secs(10)),
        })?;

        self.sequence += 1;

        // Insert the raw atom into the segment.
        segment.write_chunk(raw.into())?;

        // Save for the next iteration
        self.segment = Some(segment);

        Ok(())
    }

    pub fn data(&mut self, raw: Vec<u8>) -> anyhow::Result<()> {
        let segment = self.segment.as_mut().context("missing segment")?;
        segment.write_chunk(raw.into())?;

        Ok(())
    }
}

pub struct Fragment {
    // The track for this fragment.
    track: u32,

    // The timestamp of the first sample in this fragment, in timescale units.
    timestamp: u64,

    // True if this fragment is a keyframe.
    keyframe: bool,
}

impl Fragment {
    fn new(moof: mp4::MoofBox) -> anyhow::Result<Self> {
        // We can't split the mdat atom, so this is impossible to support
        anyhow::ensure!(moof.trafs.len() == 1, "multiple tracks per moof atom");
        let track = moof.trafs[0].tfhd.track_id;

        // Parse the moof to get some timing information to sleep.
        let timestamp = sample_timestamp(&moof).expect("couldn't find timestamp");

        // Detect if we should start a new segment.
        let keyframe = sample_keyframe(&moof);

        Ok(Self {
            track,
            timestamp,
            keyframe,
        })
    }

    // Convert from timescale units to a duration.
    fn timestamp(&self, timescale: u64) -> time::Duration {
        time::Duration::from_millis(1000 * self.timestamp / timescale)
    }
}

fn sample_timestamp(moof: &mp4::MoofBox) -> Option<u64> {
    Some(moof.trafs.first()?.tfdt.as_ref()?.base_media_decode_time)
}

fn sample_keyframe(moof: &mp4::MoofBox) -> bool {
    for traf in &moof.trafs {
        // TODO trak default flags if this is None
        let default_flags = traf.tfhd.default_sample_flags.unwrap_or_default();
        let trun = match &traf.trun {
            Some(t) => t,
            None => return false,
        };

        for i in 0..trun.sample_count {
            let mut flags = match trun.sample_flags.get(i as usize) {
                Some(f) => *f,
                None => default_flags,
            };

            if i == 0 && trun.first_sample_flags.is_some() {
                flags = trun.first_sample_flags.unwrap();
            }

            // https://chromium.googlesource.com/chromium/src/media/+/master/formats/mp4/track_run_iterator.cc#177
            let keyframe = (flags >> 24) & 0x3 == 0x2; // kSampleDependsOnNoOther
            let non_sync = (flags >> 16) & 0x1 == 0x1; // kSampleIsNonSyncSample

            if keyframe && !non_sync {
                return true;
            }
        }
    }

    false
}

// Find the timescale for the given track.
fn track_timescale(moov: &mp4::MoovBox, track_id: u32) -> u64 {
    let trak = moov
        .traks
        .iter()
        .find(|trak| trak.tkhd.track_id == track_id)
        .expect("failed to find trak");

    trak.mdia.mdhd.timescale as u64
}

fn serve_catalog(
    track: &mut track::Publisher,
    init_track_name: String,
    moov: &mp4::MoovBox,
    _tracks: &HashMap<String, Track>,
) -> Result<(), anyhow::Error> {
    let mut segment = track.create_segment(segment::Info {
        sequence: VarInt::ZERO,
        priority: i32::MAX,
        expires: None,
    })?;

    // avc1[.PPCCLL]
    //
    // let profile = 0x64;
    // let constraints = 0x00;
    // let level = 0x1f;

    // TODO: do build multi-track catalog by looping through moov.traks
    let trak = moov.traks[0].clone();
    let avc1 = trak
        .mdia
        .minf
        .stbl
        .stsd
        .avc1
        .ok_or(anyhow::anyhow!("avc1 atom not found"))?;

    let profile = avc1.avcc.avc_profile_indication;
    let constraints = avc1.avcc.profile_compatibility; // Not 100% certain here, but it's 0x00 on my current test video
    let level = avc1.avcc.avc_level_indication;

    let width = avc1.width;
    let height = avc1.height;

    let codec = rfc6381_codec::Codec::avc1(profile, constraints, level);
    let codec_str = codec.to_string();

    let catalog = json!({
        "tracks": [
            {
            "container": "mp4",
            "kind": "video",
            "init_track": init_track_name,
            "data_track": "1", // assume just one track for now
            "codec": codec_str,
            "width": width,
            "height": height,
            "frame_rate": 30, //TODO: get from moov
            "bit_rate": 15000, //TODO: get from moov
            }
        ]
    });
    let catalog_str = serde_json::to_string_pretty(&catalog)?;
    println!("catalog: {}", catalog_str);

    // Add the segment and add the fragment.
    segment.write_chunk(catalog_str.into())?;

    Ok(())
}

pub fn handle_atom(moq_ctx: &mut MoqContext) -> Result<c_int, anyhow::Error> {
    let mut reader = Cursor::new(&moq_ctx.unread);
    let atom = read_atom(&mut reader)?;
    let atom_len = atom.len() as c_int;

    let mut reader = Cursor::new(&atom);
    let header = mp4::BoxHeader::read(&mut reader)?;
    dbg!(header.name, header.size, atom.len());

    match header.name {
        mp4::BoxType::MoofBox => {
            let moof =
                mp4::MoofBox::read_box(&mut reader, header.size).context("failed to read MP4")?;

            // Process the moof.
            let fragment = Fragment::new(moof)?;
            let name = fragment.track.to_string();

            // Get the track for this moof.
            let track = moq_ctx
                .tracks
                .get_mut(&name)
                .context("failed to find track")?;

            // Save the track ID for the next iteration, which must be a mdat.
            anyhow::ensure!(moq_ctx.track_name.is_none(), "multiple moof atoms");
            moq_ctx.track_name.replace(name);

            // Publish the moof header, creating a new segment if it's a keyframe.
            track
                .header(atom, fragment)
                .context("failed to publish moof")?;
        }
        mp4::BoxType::MdatBox => {
            // Get the track ID from the previous moof.
            let name = moq_ctx.track_name.take().context("missing moof")?;
            let track = moq_ctx
                .tracks
                .get_mut(&name)
                .context("failed to find track")?;

            // Publish the mdat atom.
            track.data(atom).context("failed to publish mdat")?;
        }

        _ => {
            // Skip unknown atoms
        }
    };
    Ok(atom_len)
}
