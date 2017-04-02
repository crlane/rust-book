
env:
	@docker run --rm -it -v`pwd`/src:/opt/src -w /opt/src scorpil/rust:1.16 bash
