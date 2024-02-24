build:
	make build_preview_client
	make build_server
	make make_static_files

build_preview_client:
	cd snap-client && npm install && npm run build

build_server:
	cd snap-server && cargo build --release

make_static_files:
	cp -r snap-client/build snap-server/public
	cp -r snap-client/node_modules/highlight.js/styles/ snap-server/public/
