IMAGE ?= rust
TAG ?= 1.30.0

run:
	@docker run --rm -it --user $(shell id -u):$(shell id -g) -e USER=${USER} -v`pwd`:/opt/src -w /opt/src ${IMAGE}:${TAG} bash
