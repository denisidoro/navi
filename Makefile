BIN ?= navi
PREFIX ?= /usr/local

install:
	scripts/symlink

uninstall:
	rm -f $(PREFIX)/bin/$(BIN)
