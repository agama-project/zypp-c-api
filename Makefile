all:
	$(MAKE) -C c-layer $@
	$(MAKE) -C c-example $@
	(cd rust; cargo build)
	doxygen

check:
	git ls-files | grep '\.[ch]' | \
	  xargs --verbose clang-format --style="{BasedOnStyle: llvm, ColumnLimit: 120}" --dry-run
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
	  xargs --verbose clang-format --style="{BasedOnStyle: llvm, ColumnLimit: 120}" -i
	(cd rust; cargo fmt)
