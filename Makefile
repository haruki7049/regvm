CC ?= gcc
CFLAGS := -Wall -c
BINARY := regvm
OBJDIR := obj
SRCDIR := src

.PHONY: build

$(OBJDIR)/%.o: $(SRCDIR)/%.c
	mkdir -p $(OBJDIR)
	$(CC) $(CFLAGS) $< -o $@

build: $(OBJDIR)/main.o
	mkdir -p bin
	cc -o regvm $^
	mv regvm bin

clean:
	rm -rf obj/ bin/ src/*.pch

bear:
	bear -- make
