TRANSFORMED_MAN_PAGE := doc/passextract.1.transformed
VERSION := 0.5.0
RELEASE_DIR := passextract-$(VERSION)

$(TRANSFORMED_MAN_PAGE): doc/passextract.1.txt
	sed 's/`/*/g' $< > $@

clean_transformed:
	rm $(TRANSFORMED_MAN_PAGE)

doc/passextract.1: $(TRANSFORMED_MAN_PAGE)
	a2x --no-xmllint --format manpage $<

doc: doc/passextract.1 clean_transformed

release:
	cargo build --release
	mkdir -p $(RELEASE_DIR)
	cp target/release/passextract \
		passextract.bash-completion \
		doc/passextract.1 \
		$(RELEASE_DIR)
	tar cjvf passextract-$(VERSION)_osx_amd64.tar.bz2 \
		$(RELEASE_DIR)
	rm -rf $(RELEASE_DIR)

.PHONY: clean_transformed doc release
