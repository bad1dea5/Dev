TARGET := program

CFLAGS :=
LFLAGS := -lc -I/lib64/ld-linux-x86-64.so.2

DEBUG :=
OBJECTS := $(patsubst %.s, obj/%.o, $(wildcard *.s))

.PHONY: clean

#
#
#
.DELETE_ON_ERROR:
bin/$(TARGET): $(OBJECTS) | bin
	@printf "\033[33m= Linker =\033[0m\n"
ifndef DEBUG
	ld -s -o $@ $(LFLAGS) $^
else
	ld -o $@ $(LFLAGS) $^
endif

#
#
#
obj/%.o: %.s | obj
	as -c $< $(CFLAGS) -o $@

#
#
#
bin obj:
	@mkdir $@

clean:
	-@$(RM) -frv bin obj
