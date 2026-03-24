BINARY := regvm
OBJECT := main.o
SOURCE := main.c
CFLAGS := -Wall
CC := gcc

.PHONY: build

build: main.o
	mkdir -p bin
	cc -o regvm $^
	mv regvm bin

main.o: src/main.c src/main.h
	${CC} ${CFLAGS} -c $^

clean: main.o src/*.pch bin/
	rm -r $^

bear:
	bear -- make
