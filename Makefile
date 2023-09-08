install:
	cargo build --release
	mkdir -p /usr/local/include/libmoq
	cp target/release/libmoq.pc /usr/local/lib/pkgconfig/
	cp target/release/libmoq.dylib /usr/local/lib/
	cp target/release/libmoq.a /usr/local/lib/
	cp target/release/moq.h /usr/local/include/libmoq/

uninstall:
	rm -f /usr/local/lib/pkgconfig/libmoq.pc
	rm -f /usr/local/lib/libmoq.dylib
	rm -f /usr/local/lib/libmoq.a
	rm -f /usr/local/include/libmoq/moq.h
	rmdir /usr/local/include/libmoq
