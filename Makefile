subdirs := kernel bootloader

.PHONY: all
all: clean build run

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

debug: make-img
	sh qemu_debug.sh

