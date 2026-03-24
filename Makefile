CC ?= gcc
CFLAGS := -Wall -c
BINARY := regvm
OBJDIR := obj
SRCDIR := src
SRCS := $(wildcard $(SRCDIR)/*.c)

.PHONY: build

$(OBJDIR)/%.o: $(SRCS)
	mkdir -p $(OBJDIR)
	$(CC) $(CFLAGS) $^

build: $(OBJDIR)/*.o
	mkdir -p bin
	$(CC) -o regvm $^
	mv regvm bin

clean:
	rm -rf obj/ bin/ src/*.pch

bear:
	bear -- make
