SHELL := /bin/sh

.POSIX:
.PHONY: all

# TODO: Dyanmic discovery of diagrams.

all: diagrams

diagrams:
	+${MAKE} all -C main_event_loop

clean:
	+${MAKE} clean -C main_event_loop
