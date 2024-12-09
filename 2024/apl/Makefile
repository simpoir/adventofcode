SCRIPTS = $(wildcard day*.apl)
PKG = https://ftp.gnu.org/gnu/apl/apl_1.9-1_amd64.deb

test:
	@for f in $(SCRIPTS); do echo $${f%.apl}; ./$$f -t; done
.PHONY: test

run_all:
	@for f in $(SCRIPTS); do echo $${f%.apl}; ./$$f; done
.PHONY: run_all

dev%: day%.apl
	while true; do \
		inotifywait -e close_write $<; \
		apl --LX ')more auto on' --LX ']boxing 4' -L ./lib.apl -q -s -f ./$< -- -t; \
		done

ubuntu-deps:
	# broken. build from source
	set -e; \
	tmp_pkg=$(shell mktemp); \
	wget -O $$tmp_pkg $(PKG); \
	sudo dpkg -i $$tmp_pkg
.PHONY: ubuntu-deps
