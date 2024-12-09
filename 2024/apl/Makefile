SCRIPTS = $(wildcard *.apl)
PKG = https://ftp.gnu.org/gnu/apl/apl_1.9-1_amd64.deb

test:
	for f in $(SCRIPTS); do echo $${f%.apl}; ./$$f -t; done
.PHONY: test

run_all:
	for f in $(SCRIPTS); do echo $${f%.apl}; ./$$f; done
.PHONY: run_all

ubuntu-deps:
	set -e; \
	tmp_pkg=$(shell mktemp); \
	wget -O $$tmp_pkg $(PKG); \
	sudo dpkg -i $$tmp_pkg
.PHONY: ubuntu-deps
