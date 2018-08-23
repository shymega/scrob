.PHONY: all clean

all: dests

dests: $(patsubst %.dot,%.png,$(wildcard *.dot)) \
	   $(patsubst %.dot,%.svg,$(wildcard *.dot))

%.png: %.dot
	dot -Tpng $< -o $@

%.svg: %.dot
	dot -Tsvg $< -o $@

clean:
	rm -f *.{svg,png}
