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
	make clean
	make test-build
	make run KERNEL="test"

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
	sh make_img.sh $(KERNEL)

.PHONY:run
run:
	make make-img KERNEL=$(KERNEL)
	sh qemu.sh

run-debug:
	make make-img KERNEL=$(KERNEL)
	sh qemu.sh "debug"

