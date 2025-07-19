TARGET := program

CFLAGS := -std=c++20
LFLAGS := -L/usr/lib/gcc/x86_64-linux-gnu/13/ -I/lib64/ld-linux-x86-64.so.2 -lc -lstdc++

DEBUG :=
OBJECTS := $(patsubst src/%.cpp, obj/%.o, $(wildcard src/*.cpp))

.PHONY: clean

#
#
#
.DELETE_ON_ERROR:
bin/$(TARGET): $(OBJECTS) | bin
	@printf "\033[33m= Linker =\033[0m\n"
	ld -o $@ $(LFLAGS) $^

#
#
#
obj/%.o: src/%.cpp | obj
ifndef DEBUG
	g++ -c $< $(CFLAGS) -o $@
else
	g++ -g -c $< $(CFLAGS) -o $@
endif

#
#
#
bin obj:
	@mkdir $@

clean:
	-@$(RM) -frv bin obj
