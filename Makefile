subdirs :=  kernel bootloader

.PHONY: all
all:
	make build
	make run

.PHONY: debug
debug:
	make build
	make run-debug

.PHONY: build $(subdirs)
build: $(subdirs)

.PHONY: clean $(subdirs)
clean: $(subdirs)
	cargo clean

$(subdirs):
	make build -C $@

make-img:
	sh make_img.sh

run: make-img
	sh qemu.sh

run-debug: make-img
	sh qemu_debug.sh "debug"

