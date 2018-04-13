BUILDDIR := build
release := $(BUILDDIR)/release
debug := $(BUILDDIR)/debug
degraded := $(BUILDDIR)/degraded
hardened := $(BUILDDIR)/hardened
asan := $(BUILDDIR)/asan
msan := $(BUILDDIR)/msan
name := race

all: release debug

release: main.c
	mkdir -p $(release)
	clang main.c -pthread -O2 -o $(release)/$(name)

debug: main.c
	mkdir -p $(debug)
	clang -g main.c -pthread -o $(debug)/$(name)

clean:
	rm -Rf $(BUILDDIR)
