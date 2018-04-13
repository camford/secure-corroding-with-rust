BUILDDIR := build
release := $(BUILDDIR)/release
debug := $(BUILDDIR)/debug
name := bo

all: release debug

release: main.c
	mkdir -p $(release)
	clang -Xclang -fcolor-diagnostics -Werror main.c -O2 -o $(release)/$(name)

debug: main.c
	mkdir -p $(debug)
	clang -Xclang -fcolor-diagnostics -g -Werror main.c -o $(debug)/$(name)

clean:
	rm -Rf $(BUILDDIR)
