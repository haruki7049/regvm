CC ?= gcc
CFLAGS := -Wall
BINARY := regvm
OBJDIR := obj
SRCDIR := src
BINDIR := bin
SRCS := $(wildcard $(SRCDIR)/*.c)
# Generate object file paths from source file paths
OBJS := $(SRCS:$(SRCDIR)/%.c=$(OBJDIR)/%.o)

.PHONY: build clean bear set-dirs
.DEFAULT_GOAL := build

$(OBJDIR)/%.o: $(SRCDIR)/%.c setup-dirs
	$(CC) $(CFLAGS) -c $< -o $@

setup-dirs:
	mkdir -p $(OBJDIR)
	mkdir -p $(BINDIR)

build: $(OBJS) setup-dirs
	$(CC) -o $(BINDIR)/$(BINARY) $(OBJS)

clean:
	rm -rf $(OBJDIR) $(BINDIR) src/*.pch

bear:
	bear -- make
