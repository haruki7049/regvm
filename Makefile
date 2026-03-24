CFLAGS ?= -Wall

build: main.o
	mkdir -p bin
	cc -o bin/regvm $^

main.o: src/main.c
	${CC} -c $^

clean: main.o
	rm $^
