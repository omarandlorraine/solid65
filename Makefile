all: generate_test_case

test clean:
	make -C rubbermallet $@
	make -C gianlucag $@

generate_test_case:
