SCRIPTS = $(wildcard day*.apl)
PKG = https://ftp.gnu.org/gnu/apl/apl_1.9-1_amd64.deb

test:
	@for f in $(SCRIPTS); do echo $${f%.apl}; /bin/time -f '%Us\n' ./$$f -t; done
.PHONY: test

run_all:
	@for f in $(SCRIPTS); do echo $${f%.apl}; /bin/time -f '%Us\n' ./$$f; done
.PHONY: run_all

dev%: day%.apl
	while true; do \
		apl --LX ')more auto on' --LX ']boxing 8' -L ./lib.apl -q -s -f ./$< -- -t; \
		inotifywait -e close_write $<; \
		done
.PHONY: dev%

ubuntu-deps:
	# broken. build from source
	set -e; \
	tmp_pkg=$(shell mktemp); \
	wget -O $$tmp_pkg $(PKG); \
	sudo dpkg -i $$tmp_pkg
	sudo apt-get install inotify-tools
.PHONY: ubuntu-deps
