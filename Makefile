CFLAGS ?= -Wall
CC ?= gcc

build: main.o
	mkdir -p bin
	cc -o regvm $^
	mv regvm bin

main.o: src/main.c src/main.h
	${CC} ${CFLAGS} -c $^

clean: main.o bin/
	rm -r $^

bear:
	bear -- make
