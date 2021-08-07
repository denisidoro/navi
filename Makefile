export PATH := /usr/local/opt/bash/bin/:$(PATH)

install:
	scripts/make install

uninstall:
	scripts/make uninstall

fix:
	scripts/make fix

test:
	scripts/test

build:
	cargo build