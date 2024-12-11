.PHONY: clean

all: serve

serve:
	@bundle exec jekyll serve

clean:
	@$(RM) -frv _site/
