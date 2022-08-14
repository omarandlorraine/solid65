CC=g++
CFLAGS=-g -Wall -Wextra -pedantic
SOURCES=test.cpp mos6502.cpp
BUILD_DIR=build/
OBJECTS=$(SOURCES:%.cpp=$(BUILD_DIR)%.o)

default: test

mos6502.h: mos6502/mos6502.h
	cp $< $@

mos6502.cpp: mos6502/mos6502.cpp mos6502.h
	cp $< $@

$(OBJECTS): $(BUILD_DIR)%.o:%.cpp
	@mkdir -p build/
	$(CC) -c $(CFLAGS) -o $@ $^

test: $(OBJECTS)
	$(CC) $(CFLAGS) $(LDOPTS) -o $@ $^

.PHONY: clang-tidy
clang-tidy: mos6502.cpp
	clang-tidy --checks=modernize-\* *.cpp

.PHONY: cppcheck
cppcheck: mos6502.cpp
	cppcheck *.cpp

.PHONY: clean
clean:
	rm -rf $(BUILD_DIR) test mos6502.cpp mos6502.h
