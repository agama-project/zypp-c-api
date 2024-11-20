all:
	$(MAKE) -C c-layer $@
	$(MAKE) -C c-example $@
	(cd rust; cargo build)
	doxygen

clean:
	$(MAKE) -C c-layer $@
	$(MAKE) -C c-example $@
	(cd rust; cargo clean)
