SRCS := $(src/%.ty)
OBJS := $(patsubst src/%.ty,.ty/%.o,$(SRCS))

.ty/lexer.o:
	ty obj src/lexer.ty

.ty/token.o:
	ty obj src/token.ty

.PHONY: clean
clean:
	rm -rf .ty
	mkdir .ty
