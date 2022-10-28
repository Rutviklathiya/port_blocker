BPF_SRCS = $(shell find src/bpf/ -type f -regex ".*\.bpf.c")
BPF_OBJS = $(patsubst src/bpf/%.bpf.c,target/bpf/%.bpf.o,$(BPF_SRCS))

.PHONY: build unload load clean chk bpf

build:
	cargo build

unload:
	cargo run -- -u

load:
	cargo run -- -a "9000,22,9090"

clean:
	cargo clean
	-rm -rf src/bpf/.out/

chk:
	$(MAKE) clean
	cargo check
	cargo fmt -- --check
	cargo clippy
	$(MAKE) bpf

target/bpf/%.bpf.o: src/bpf/%.bpf.c
	@mkdir -p target/bpf/
	@bpftool btf dump file /sys/kernel/btf/vmlinux format c > target/bpf/vmlinux.h
	clang -g -O2 -target bpf \
		-D__TARGET_ARCH_x86 \
		-I target/bpf/ \
		-c $< -o $@

bpf: $(BPF_OBJS)
