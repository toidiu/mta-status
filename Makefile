#TODO make this dymamic
VERSION=0.1.0
NAME=mta_status
REGISTRY=toidiu

watch:
	watchexec --exts rs rustup run nightly "cargo rustc --features clippy -- -Z no-trans"

build:
	docker run --rm -it -v `pwd`:/home/rust/src ekidd/rust-musl-builder:stable cargo build --release && \
	docker build -t $(NAME):$(VERSION) . && \
		docker tag $(NAME):$(VERSION) $(REGISTRY)/$(NAME):$(VERSION)

run:
	docker run -p4000:4000 --rm $(NAME):$(VERSION) 

push:
	docker push $(REGISTRY)/$(NAME):$(VERSION)

.PHONY: watch build run
