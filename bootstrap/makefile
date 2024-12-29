SRCS := $(wildcard src/*.ty)
OBJS := $(patsubst src/%.ty,.ty/%.o,$(SRCS))

all: $(OBJS)
	echo $(OBJS)
	echo $(SRCS)
	touch .ty/all

$(OBJS): $(SRCS)
	ty obj $<

.PHONY: clean
clean:
	rm -rf .ty
	mkdir .ty
