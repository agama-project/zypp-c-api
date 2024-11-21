CC=gcc
CFLAGS= -Wall -I. -I../c-layer/include -Wall -lzypp -lstdc++
OBJ = main.o
DEPS = ../c-layer/include/lib.h ../c-layer/include/callbacks.h

all: main

clean:
	rm -vf *.o main

main: $(OBJ)
	$(CC) -o $@ $^ ../c-layer/libagama-zypp.a $(CFLAGS)

%.o: %.c $(DEPS)
	$(CC) -c -o $@ $< $(CFLAGS)
