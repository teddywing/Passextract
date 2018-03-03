TRANSFORMED_MAN_PAGE := doc/passextract.1.transformed

$(TRANSFORMED_MAN_PAGE): doc/passextract.1.txt
	sed 's/`/*/g' $< > $@

clean_transformed:
	rm $(TRANSFORMED_MAN_PAGE)

doc/passextract.1: $(TRANSFORMED_MAN_PAGE)
	a2x --no-xmllint --format manpage $<

doc: doc/passextract.1 clean_transformed

.PHONY: clean_transformed doc
