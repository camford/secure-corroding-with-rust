BUILDDIR = build
release  = $(BUILDDIR)/release
debug    = $(BUILDDIR)/debug
name     = unqiue_ptr
CXX      = clang++
CXXFLAGS = -std=c++14 -stdlib=libc++ -Wall -pedantic

all: release debug

release: main.cpp
	mkdir -p $(release)
	$(CXX) main.cpp $(CXXFLAGS) -O2 -o $(release)/$(name)

debug: main.cpp
	mkdir -p $(debug)
	$(CXX) -g main.cpp $(CXXFLAGS) -o $(debug)/$(name)

clean:
	rm -Rf $(BUILDDIR)
