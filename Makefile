all:
	$(MAKE) -C c-layer $@
	$(MAKE) -C c-example $@
# TODO: put bindgen to build.rs
	(cd rust/zypp-agama-sys; bindgen --merge-extern-blocks headers.h -o src/bindings.rs -- -I../../c-layer/include)
	(cd rust; cargo build)
	doxygen

check:
	git ls-files | grep '\.[ch]' | \
	  xargs --verbose clang-format --style=llvm --dry-run
	$(MAKE) -C c-layer $@
	$(MAKE) -C c-example $@
	(cd rust; cargo fmt -- --check)
	(cd rust; cargo test)

clean:
	$(MAKE) -C c-layer $@
	$(MAKE) -C c-example $@
	(cd rust; cargo clean)

format:
	git ls-files | grep '\.[ch]' | \
	  xargs --verbose clang-format --style=llvm -i
	(cd rust; cargo fmt)
