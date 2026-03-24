BINARY := regvm
OBJECT := main.o
SOURCE := main.c
CFLAGS := -Wall
CC := gcc

.PHONY: build

build: obj/main.o
	mkdir -p bin
	cc -o regvm $^
	mv regvm bin

obj/main.o: src/main.c
	mkdir -p obj
	${CC} ${CFLAGS} -c $^ -o obj/main.o

clean:
	rm -rf obj/ bin/ src/*.pch

bear:
	bear -- make
