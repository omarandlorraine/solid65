CC=gcc
CFLAGS=-g -Wall -Wextra -pedantic
SOURCES=test.c fake6502.c
BUILD_DIR=build/
OBJECTS=$(SOURCES:%.c=$(BUILD_DIR)%.o)

default: test

$(OBJECTS): $(BUILD_DIR)%.o:%.c
	@mkdir -p build/
	$(CC) -c $(CFLAGS) -o $@ $^

test: $(OBJECTS)
	$(CC) $(CFLAGS) $(LDOPTS) -o $@ $^

.PHONY: clean
clean:
	rm -rf $(BUILD_DIR) test
