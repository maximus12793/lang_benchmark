#------------------------------------------------------------------------------

SOURCE=main.cpp 
MYPROGRAM=grab_data
MYINCLUDES=-I/usr/local/Cellar/curlpp/0.8.1/include/ -I/usr/local/Cellar/curlpp/0.8.1/include/ -I/Users/maximilian.roquemore/cpp_libs/boost_1_65_1/
MYLIBS=-L/usr/local/Cellar/curlpp/0.8.1/lib/ -L/usr/local/Cellar/curlpp/0.8.1/lib/ -L/Users/maximilian.roquemore/cpp_libs/boost_1_65_1/libs/ -L/Users/maximilian.roquemore/cpp_libs/boost_1_65_1/stage/lib -L/opt/local/lib -L/usr/local/lib -L/usr/local/Cellar/boost/1.65.1/lib
CC=clang++
OTHERFLAGS=-lcurlpp -lcurl -lpthread -lboost_system 

#------------------------------------------------------------------------------



all: $(MYPROGRAM)



$(MYPROGRAM): $(SOURCE)

	$(CC) -std=c++14 $(MYINCLUDES) $(SOURCE) -o$(MYPROGRAM) $(OTHERFLAGS)

clean:

	rm -f $(MYPROGRAM)
