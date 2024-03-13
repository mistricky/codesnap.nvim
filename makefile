build_generator:
	cd generator && cargo build --release
	cp generator/target/release/libgenerator.dylib lua/generator.so

build_mac_x86_64_target:
	rustup target install x86_64-apple-darwin
	cd generator && cargo build --target x86_64-apple-darwin --release
	cp generator/target/release/libgenerator.dylib lua/generator.so

build_mac_arm64_target:
	rustup target install aarch64-apple-darwin
	cd generator && cargo build --target aarch64-apple-darwin --release
	cp generator/target/release/libgenerator.dylib lua/generator.so

build_linux_x86_64_target:
	cd generator && cargo build --release
	cp generator/target/release/libgenerator.so lua/generator.so
