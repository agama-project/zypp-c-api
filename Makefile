CXX=g++
CXXFLAGS=-I. -Izypp -Wall -std=c++14 -lzypp
C=gcc
CFLAGS=-I. -Wall -lzypp -lstdc++
DEPS = lib.h
OBJ = lib.o callbacks.o main.o

%.o: %.cxx $(DEPS)
	$(CXX) -c -o $@ $< $(CXXFLAGS)

%.o: %.c $(DEPS)
	$(C) -c -o $@ $< $(CFLAGS)


main: $(OBJ)
	$(CC) -o $@ $^ $(CFLAGS)
