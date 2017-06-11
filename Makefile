
VERSION=0.1.2
NAME=mta_status
REGISTRY=bla.dkr.ecr.us-east-1.amazonaws.com

watch:
	watchexec --exts rs rustup run nightly "cargo rustc --features clippy -- -Z no-trans"

build:
	docker run --rm -it -v `pwd`:/home/rust/src ekidd/rust-musl-builder:stable cargo build --release && \
	docker build -t $(NAME):$(VERSION) .
#	docker build -t $(NAME):$(VERSION) . && \
#		docker tag $(NAME):$(VERSION) $(REGISTRY)/$(NAME):$(VERSION)

run:
	docker run -p4000:4000 --rm $(NAME):$(VERSION)

push:
	`aws ecr get-login --region us-east-1` && \
		docker push $(REGISTRY)/$(NAME):$(VERSION)

.PHONY: watch build run



alias rust-musl-builder='docker run --rm -it -v "$(pwd)":/home/rust/src ekidd/rust-musl-builder'
rust-musl-builder cargo build --release