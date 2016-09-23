SRC_PREFIX ?= $(CURDIR)

FC      := gfortran-4.9
F_FLAGS := -O6 -fPIC -g -march=native --std=legacy

QUADPACK_ARCHIVE := libquadpack_native.a

QUADPACK_OBJECTS := \
        qpsrt.o     dqpsrt.o \
        qelg.o      dqelg.o \
        qk15i.o     dqk15i.o \
        qk21.o      dqk21.o \
        qagi.o      qagie.o \
        dqagi.o     dqagie.o \
        qags.o      qagse.o \
        dqags.o     dqagse.o

.PHONY: all clean

%.o: $(SRC_PREFIX)/%.f
	$(FC) $(F_FLAGS) -c -o $@ $<

all: $(QUADPACK_ARCHIVE)

clean:
	rm -f $(QUADPACK_ARCHIVE)
	rm -f $(QUADPACK_OBJECTS)

$(QUADPACK_ARCHIVE): $(QUADPACK_OBJECTS)
	ar rc $@ $^
