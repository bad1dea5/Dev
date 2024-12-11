TARGET 	:= program

CFLAGS	:=
LFLAGS	:=

DEBUG	:=
OBJS	:= $(patsubst %.s, obj/%.o, $(wildcard *.s))

.PHONY: all clean

.DELETE_ON_ERROR:
bin/$(TARGET): $(OBJS) | bin
	@printf "\033[33m== Linker ==\033[0m\n"
ifndef DEBUG
	ld -s -o $@ $(LFLAGS) $^
else
	ld -o $@ $(LFLAGS) $^
endif

obj/%.o: %.s | obj
ifndef DEBUG
	as -c $< $(CFLAGS) -o $@
else
	as --gstabs -c $< $(CFLAGS) -o $@
endif

clean:
	-@$(RM) -frv bin obj

bin obj:
	@mkdir $@
