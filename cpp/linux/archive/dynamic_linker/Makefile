.DELETE_ON_ERROR:

TARGET 	:= program

CC		:= g++
CFLAGS	:=
LFLAGS	:= -I/lib64/ld-linux-x86-64.so.2 -lc

DEBUG	:=
OBJS	:= $(patsubst %.cpp, obj/%.o, $(wildcard *.cpp))

.PHONY: all clean

bin/$(TARGET): $(OBJS) | bin
	@printf "\033[33m== Linker ==\033[0m\n"
ifndef DEBUG
	ld -s -o $@ $(LFLAGS) $^
else
	ld -o $@ $(LFLAGS) $^
endif

obj/%.o: %.cpp | obj
ifndef DEBUG
	$(CC) -c $< $(CFLAGS) -o $@
else
	$(CC) -g -c $< $(CFLAGS) -o $@
endif

clean:
	-@$(RM) -frv bin obj

bin obj:
	@mkdir $@
