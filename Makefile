subdirs :=  kernel bootloader

.PHONY: all
all:
	make build
	make run KERNEL="target/kernel/debug/kernel.elf"

.PHONY: debug
debug:
	make build
	make run-debug KERNEL="target/kernel/debug/kernel.elf"

.PHONY: test
test:
	make test-build
	make run KERNEL=$(shell find target/kernel/debug/deps/ -name '*.elf')

.PHONY: test-build $(subdirs)
test-build: $(subdirs)


.PHONY: build $(subdirs)
build: $(subdirs)

.PHONY: clean $(subdirs)
clean: $(subdirs)
	cargo clean

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
	sh qemu_debug.sh "debug"

