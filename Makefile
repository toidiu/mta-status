#TODO make this dymamic
VERSION=0.1.1
NAME=mta-status
REGISTRY=toidiu

watch:
	watchexec --exts rs rustup run nightly "cargo rustc --features clippy -- -Z no-trans"

build:
	docker run --rm -it -v `pwd`:/home/rust/src ekidd/rust-musl-builder:stable cargo build --release && \
	docker build -t $(NAME):$(VERSION) . && \
	docker tag $(NAME):$(VERSION) $(REGISTRY)/$(NAME):$(VERSION)

docker-build:
	docker build -t $(NAME):$(VERSION) . && \
	docker tag $(NAME):$(VERSION) $(REGISTRY)/$(NAME):$(VERSION)

#	docker tag $(NAME):$(VERSION) gcr.io/trygke/$(NAME):$(VERSION)

docker-push:
	docker push $(REGISTRY)/$(NAME):$(VERSION)

raspi:
	docker run \
		--volume $PWD:/home/cross/project \
		--volume ~/.cargo/registry:/home/cross/.cargo/registry \
		ragnaroek/rust-raspberry:1.23.0 build --release

.PHONY: watch build raspi docker-build docker-push
