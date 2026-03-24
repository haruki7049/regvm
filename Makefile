CFLAGS ?= -Wall

build: main.o
	mkdir -p bin
	cc -o regvm $^
	mv regvm bin

main.o: src/main.c
	${CC} -c $^

clean: main.o bin/
	rm -r $^
