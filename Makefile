SERVICE_NAME = hlhsinfo_backend_server
SERVICE_PATH = /etc/systemd/system/$(SERVICE_NAME).service

run:
	cargo run

build:
	cargo build --release

install:
	make build

	sudo cp target/release/$(SERVICE_NAME) /usr/local/bin/
	sudo cp tools/$(SERVICE_NAME).service $(SERVICE_PATH)
	sudo systemctl enable $(SERVICE_NAME)
	sudo systemctl start $(SERVICE_NAME)

uninstall:
	sudo systemctl stop $(SERVICE_NAME)
	sudo systemctl disable $(SERVICE_NAME)
	sudo rm $(SERVICE_PATH)
	sudo rm /usr/local/bin/$(SERVICE_NAME)