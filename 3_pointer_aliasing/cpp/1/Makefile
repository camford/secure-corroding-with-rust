BUILDDIR = build
release  = $(BUILDDIR)/release
debug    = $(BUILDDIR)/debug
degraded = $(BUILDDIR)/degraded
hardened = $(BUILDDIR)/hardened
asan     = $(BUILDDIR)/asan
msan     = $(BUILDDIR)/msan
name     = unique_ptr
CXX      = clang++
CXXFLAGS = -std=c++14 -stdlib=libc++ -Wall -pedantic

all: release debug degraded hardened asan msan

release: main.cpp
	mkdir -p $(release)
	$(CXX) main.cpp $(CXXFLAGS) -O2 -o $(release)/$(name)

debug: main.cpp
	mkdir -p $(debug)
	$(CXX) -g main.cpp $(CXXFLAGS) -o $(debug)/$(name)

degraded: main.cpp
	mkdir -p $(degraded)
	$(CXX) -g main.cpp $(CXXFLAGS) -Wl,-z,execstack -o $(degraded)/$(name)

# Stack cookies on most functions -fstack-protector-strong
# DEP - -z,noexecstack
# Full RELRO - -z,relro,-z,now
# ASLR - -fpie -pie
# https://blog.quarkslab.com/clang-hardening-cheat-sheet.html
hardened: main.cpp
	mkdir -p $(hardened)
	$(CXX) -g main.cpp $(CXXFLAGS) -O2 -fstack-protector-strong -fpie -pie -D_FORTIFY_SOURCE=2 -Wl,-z,noexecstack,-z,relro,-z,now -o $(hardened)/$(name)

# https://clang.llvm.org/docs/AddressSanitizer.html
asan: main.cpp
	mkdir -p $(asan)
	$(CXX) -g main.cpp $(CXXFLAGS) -O1 -fsanitize=address -fno-omit-frame-pointer -fno-optimize-sibling-calls -o $(asan)/$(name)

# https://clang.llvm.org/docs/MemorySanitizer.html
msan: main.cpp
	mkdir -p $(msan)
	$(CXX) -g main.cpp $(CXXFLAGS) -O1 -fsanitize=memory -fsanitize-memory-track-origins -fno-omit-frame-pointer -fno-optimize-sibling-calls -o $(msan)/$(name)

clean:
	rm -Rf $(BUILDDIR)
