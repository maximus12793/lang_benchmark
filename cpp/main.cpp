#include <stdlib.h>
#include <boost/fiber/all.hpp>
#include <boost/format.hpp>
#include <iostream>
#include <fstream>
#include <sstream>
#include <curlpp/cURLpp.hpp>
#include <curlpp/Easy.hpp>
#include <curlpp/Options.hpp>

using namespace std;
using namespace boost::fibers;
using namespace boost::this_fiber;

int ct = 0;

void collect_data()
{
    auto url = "https://en.wikipedia.org/wiki/Immanuel_Kant";
    ct++;
    std::ostringstream os;
    os << curlpp::options::Url(url);
    ofstream myfile;
    myfile.open(str(boost::format("/data/%1%.txt") % ct));
    myfile << os.str();
    myfile.close();
}

int main()
{
    vector<int> q(10);
    auto g = 33;
    for (auto i : q)
    {
        if (i == 0)
        {
            continue;
        }
        boost::fibers::fiber(long_running);
    }
    return 0;
}