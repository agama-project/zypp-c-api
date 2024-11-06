CXX=g++
CXXFLAGS=-I. -Wall -std=c++14 -lzypp
C=gcc
CFLAGS=-I. -Izypp -Wall -lzypp -lstdc++
DEPS = lib.h
OBJ = lib.o main.o

%.o: %.cxx $(DEPS)
	$(CXX) -c -o $@ $< $(CXXFLAGS)

%.o: %.c $(DEPS)
	$(C) -c -o $@ $< $(CFLAGS)


main: $(OBJ)
	$(CC) -o $@ $^ $(CFLAGS)
