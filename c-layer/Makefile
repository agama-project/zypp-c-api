AR=ar
CXX=g++
CXXFLAGS=-I./include -I./internal -Izypp -Wall -std=c++14 -lzypp
DEPS = include/lib.h include/callbacks.h internal/callbacks.hxx
OBJ = lib.o callbacks.o

clean:
	rm *.o *.a

%.o: %.cxx $(DEPS)
	$(CXX) -c -o $@ $< $(CXXFLAGS)

agama-zypp.a: $(OBJ)
	$(AR) -crs $@ $^
