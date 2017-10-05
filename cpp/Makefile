#------------------------------------------------------------------------------

SOURCE=main.cpp 
MYPROGRAM=grab_data
MYINCLUDES=/home/scale/g++Projects/gLib/
CC=clang++
OTHERFLAGS=-lboost

#------------------------------------------------------------------------------



all: $(MYPROGRAM)



$(MYPROGRAM): $(SOURCE)

	$(CC) -I$(MYINCLUDES) $(SOURCE) -o$(MYPROGRAM) $(OTHERFLAGS)

clean:

	rm -f $(MYPROGRAM)
