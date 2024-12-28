all: .ty/main.o .ty/integrationc.o .ty/integration.o .ty/repr-37b621cda8f778ea.repr.c0ded12167d2f276-cgu.0.rcgu.o
	cd .ty && objdump -D integrationc.o > integrationc.o.txt
	cd .ty && objdump -D integration.o > integrationty.o.txt
	cd .ty && objdump -D repr-37b621cda8f778ea.repr.c0ded12167d2f276-cgu.0.rcgu.o > integrationrs.o.txt
	touch .ty/all

.ty/main.o: main.c
	cc -c main.c -o .ty/main.o -O3

.ty/integrationc.o: integration.c
	cc -c integration.c -o .ty/integrationc.o -O3

.ty/integration.o: integration.ty
	ty obj integration.ty

.ty/repr-37b621cda8f778ea.repr.c0ded12167d2f276-cgu.0.rcgu.o: ../../target/release/librepr.rlib
	cd .ty && ar x ../../../target/release/librepr.rlib

../../target/release/librepr.rlib: ../../repr/src/lib.rs
	cd ../.. && cargo build -p repr --release

.PHONY: clean
clean:
	rm -rf .ty
	mkdir .ty
