subdirs := kernel bootloader

.PHONY: all
all: clean build run

.PHONY: debug
debug: clean build run_debug

.PHONY: build $(subdirs)
build: $(subdirs)

$(subdirs):
	make build -C $@

clean:
	cargo clean

make-img:
	sh make_img.sh

run: make-img
	sh qemu.sh

run_debug: make-img
	sh qemu_debug.sh

