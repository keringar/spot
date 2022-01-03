all: install build

build: doc.tex
	latexmk -c doc.tex

watch: doc.tex
	latexmk -pvc doc.tex

DEPENDENCIES = luatex $\
	fontspec $\
	lm $\
	xcolor $\
	xcolor-solarized $\
	fontawesome $\
	xifthen $\
	ifmtarg $\
	pgf $\
	pgf-blur $\
	ec $\
	etoolbox $\
	xkeyval $\
	biblatex $\
	biber

install:
	tlmgr update --self
	tlmgr update texlive-scripts
	tlmgr install $(DEPENDENCIES)

clean:
	latexmk -C