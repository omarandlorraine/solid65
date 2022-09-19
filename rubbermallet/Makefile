CC=gcc
CFLAGS=-g -Wall -Wextra -pedantic
SOURCES=test.c fake6502.c
BUILD_DIR=build/
OBJECTS=$(SOURCES:%.cpp=$(BUILD_DIR)%.o)

default: test

$(OBJECTS): $(BUILD_DIR)%.o:%.cpp
	@mkdir -p build/
	$(CC) -c $(CFLAGS) -o $@ $^

test: $(OBJECTS)
	$(CC) $(CFLAGS) $(LDOPTS) -o $@ $^

.PHONY: clean
clean:
	rm -rf $(BUILD_DIR) test
