all: clone-and-init install-deps build-spdk build-isal build-rocksdb build-spfresh

clone-and-init:
	./scripts/clone-and-init.sh

install-deps:
	./scripts/install-deps.sh

build-spdk:
	./scripts/build-spdk.sh

build-isal:
	./scripts/build-isal.sh

build-rocksdb:
	./scripts/build-rocksdb.sh

build-spfresh:
	./scripts/build-spfresh.sh

clean:
	rm -rf build/*

.PHONY: all clone-and-init install-deps build-spdk build-isal build-rocksdb build-spfresh clean