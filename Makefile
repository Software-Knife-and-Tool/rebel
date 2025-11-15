
# mu project dev makefile
#
.PHONY: help emacs world

help:
	@echo "mu project dev makefile -----------------"
	@echo "    emacs - emacs adjuncts"
	@echo "    world - build rebel"

emacs:
	@echo '((nil . ((compile-command . "cd ~/projects/rebel ; make world"))))' > .dir-locals.el
	@find src -name "*.rs" -print | etags -

world:
	@cargo build
