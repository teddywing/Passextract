MAN_PAGE := doc/passextract.1.txt
TRANSFORMED_MAN_PAGE := $(patsubst %,%.tmp,$(MAN_PAGE))

doc/passextract.1: doc/passextract.1.txt
	sed 's/`/*/g' $< > $(TRANSFORMED_MAN_PAGE) && \
	a2x --no-xmllint --format manpage $(TRANSFORMED_MAN_PAGE) && \
	rm $(TRANSFORMED_MAN_PAGE)
