.ty/token.o:
	ty obj src/token.ty

../target/release/librepr.rlib: ../repr/src/lib.rs
	cd .. && cargo build -p repr --release
	cd .ty && ar x ../../target/release/librepr.rlib

.ty/test: .ty/main.o
	cc .ty/main.o .ty/repr-37b621cda8f778ea.repr.c0ded12167d2f276-cgu.0.rcgu.o -o .ty/test

.ty/main.o: src/main.c
	cc -c src/main.c -o .ty/main.o

.PHONY: clean
clean:
	rm -rf .ty
	mkdir .ty
