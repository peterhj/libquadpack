SRC_PREFIX ?= $(CURDIR)

FC      := gfortran-4.9
F_FLAGS := -O6 -fPIC -g -march=native --std=legacy

QUADPACK_ARCHIVE := libquadpack_native.a

QUADPACK_OBJECTS := \
        d1mach.o \
        dqelg.o \
        dqpsrt.o \
        dqk21.o \
        qagi.o  dqagi.o \
        qagie.o dqagie.o \
        qags.o  dqags.o \
        qagse.o dqagse.o

.PHONY: all clean

%.o: $(SRC_PREFIX)/%.f
	$(FC) $(F_FLAGS) -c -o $@ $<

all: $(QUADPACK_ARCHIVE)

clean:
	rm -f $(QUADPACK_ARCHIVE)
	rm -f $(QUADPACK_OBJECTS)

$(QUADPACK_ARCHIVE): $(QUADPACK_OBJECTS)
	ar rc $@ $^
