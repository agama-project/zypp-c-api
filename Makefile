all:
	$(MAKE) -C c-layer $@
	$(MAKE) -C c-example $@
	(cd rust; cargo build)
	doxygen

check:
	$(MAKE) -C c-layer $@
	$(MAKE) -C c-example $@
# These tests cannot be run in multiple threads
# because libzypp is not thread safe, and we do not mutex it (yet)
	(cd rust; cargo test -- --test-threads=1)

clean:
	$(MAKE) -C c-layer $@
	$(MAKE) -C c-example $@
	(cd rust; cargo clean)
