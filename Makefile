all:
	$(MAKE) -C c-layer $@
	$(MAKE) -C c-example $@
	(cd rust; cargo build)
	doxygen

check:
	$(MAKE) -C c-layer $@
	$(MAKE) -C c-example $@
	(cd rust; cargo test)

clean:
	$(MAKE) -C c-layer $@
	$(MAKE) -C c-example $@
	(cd rust; cargo clean)
