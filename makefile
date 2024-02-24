build:
	make build_server

build_preview_client:
	cd snap-client && npm install && npm run build

build_server:
	cd snap-server && cargo build --release

make_static_files:
	cp -r snap-client/build snap-server/public
