-include ../tools.mk

all: $(call STATICLIB,c1) $(call STATICLIB,c2)
	$(RUSTC) foo.rs
	$(RUSTC) bar.rs
	$(call RUN,bar)
