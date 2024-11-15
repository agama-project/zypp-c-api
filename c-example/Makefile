CC=gcc
CFLAGS= -I. -I../c-layer/include -Wall -lzypp -lstdc++
OBJ = main.o

all: main

clean:
	rm -vf *.o main

main: $(OBJ)
	$(CC) -o $@ $^ ../c-layer/libagama-zypp.a $(CFLAGS)

%.o: %.c $(DEPS)
	$(CC) -c -o $@ $< $(CFLAGS)
