BASEDIR := $(realpath $(dir $(lastword $(MAKEFILE_LIST))))
INCLUDES = -I$(BASEDIR)/include

SRCDIR = $(BASEDIR)/src

SRCS = $(filter-out $(SRCDIR)/main.c, $(wildcard $(SRCDIR)/*.c))

OBJS = $(patsubst $(SRCDIR)/%.c, %.o, $(SRCS))

LIBRARY = libsi2dr_liberty.a

%.o: $(SRCDIR)/%.c
	$(CC) $(INCLUDES) -c -fPIC $< -o $@

%.c: %.y
	@echo "Skipping Yacc"

%.c: %.l
	@echo "Skipping lex"

# static
$(LIBRARY): $(OBJS)
	ar rcs $(LIBRARY) $(OBJS)

# clean
clean:
	rm -f $(OBJS) $(LIBRARY)