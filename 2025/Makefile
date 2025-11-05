CXX = g++
CXXFLAGS = -std=c++17 -Wall -Wextra
SRC = main.cpp $(wildcard solution/*.cpp)
OBJ = $(SRC:.cpp=.o)
TARGET = main.o

all: $(TARGET)

$(TARGET): $(SRC)
	$(CXX) $(CXXFLAGS) $(SRC) -o $(TARGET)

clean:
	rm -f $(TARGET) $(OBJ)

run: all
	./$(TARGET)
