SHELL := /bin/sh

CARGO := cargo
CARGO_OPTS :=

.POSIX:
.PHONY: all build clean test bench ci doc

all: build

build:
	${CARGO} ${CARGO_OPTS} build

release:
	${CARGO} ${CARGO_OPTS} build --release

clean:
	${CARGO} ${CARGO_OPTS} clean

test:
	${CARGO} ${CARGO_OPTS} test

bench:
	${CARGO} ${CARGO_OPTS} bench

ci: clean build test

doc:
	+${MAKE} -C docs/diagrams all
	${CARGO} ${CARGO_OPTS} doc
