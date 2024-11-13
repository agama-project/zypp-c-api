CC=gcc
CFLAGS= -I. -I../c-layer/include -Wall -lzypp -lstdc++
OBJ = main.o

clean:
	rm *.o main

%.o: %.c $(DEPS)
	$(CC) -c -o $@ $< $(CFLAGS)

main: $(OBJ)
	$(CC) -o $@ $^ ../c-layer/agama-zypp.a $(CFLAGS)
