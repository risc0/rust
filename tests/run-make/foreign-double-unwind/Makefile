# ignore-cross-compile
include ../tools.mk

all: foo
	$(call RUN,foo) | $(CGREP) -v unreachable

foo: foo.rs $(call NATIVE_STATICLIB,foo)
	$(RUSTC) $< -lfoo $(EXTRARSCXXFLAGS)

$(TMPDIR)/libfoo.o: foo.cpp
	$(call COMPILE_OBJ_CXX,$@,$<)
