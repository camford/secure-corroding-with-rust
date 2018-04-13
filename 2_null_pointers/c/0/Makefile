BUILDDIR := build
release := $(BUILDDIR)/release
debug := $(BUILDDIR)/debug
name := null

all: release debug

release: main.c
	mkdir -p $(release)
	clang -Xclang -fcolor-diagnostics main.c -O2 -o $(release)/$(name)

debug: main.c
	mkdir -p $(debug)
	clang -Xclang -fcolor-diagnostics -g main.c -o $(debug)/$(name)

clean:
	rm -Rf $(BUILDDIR)
