subdirs :=  kernel bootloader

.PHONY: all
all:
	make build
	make run

.PHONY: debug
debug:
	make build
	make run-debug

.PHONY: test
test:
	make test -C kernel-lib
	make test -C pci
	rm -r -f target/kernel/
	make test-build
	make run-test KERNEL="test"

.PHONY: test-build $(subdirs)
test-build: $(subdirs)


.PHONY: build $(subdirs)
build: $(subdirs)

.PHONY: clean $(subdirs)
clean: $(subdirs)
	cargo clean
	rm -r -f target

$(subdirs):
	make $(MAKECMDGOALS) -C $@

.PHONY:make-img
make-img:
	sh ./scripts/make_img.sh $(KERNEL)

.PHONY:run
run:
	make make-img KERNEL=$(KERNEL)
	sh ./scripts/qemu.sh

.PHONY:run-test
run-test:
	make make-img KERNEL=$(KERNEL)
	sh ./scripts/qemu.sh "test"

run-debug:
	make make-img KERNEL=$(KERNEL)
	sh ./scripts/qemu.sh "debug"

