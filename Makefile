all: build

build:
	cargo build

serve:
	cargo run

clean:
	cargo clean

docker-build:
	docker build -t sunnybeta .

docker-serve:
	docker run --rm -p 127.0.0.1:2382:2382 --name sunnybeta sunnybeta
