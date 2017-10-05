#include <stdlib.h>
#include <boost/fiber/all.hpp>
#include <boost/format.hpp>
#include <iostream>
#include <fstream>

using namespace std;
namespace boost::fibers;
namespace boost::this_fiber;
namespace boost::format;

void collect_data(int x, string url)
{
    curlpp::options::Url urli(url);
    curlpp::Easy myRequest;
    myRequest.setOpt(urli);
    myRequest.perform();
    ostringstream os;
    os << myRequest;

    ofstream outFile(format("/data/%1%.txt") % x);
    outFile << os.str();
    outFile.close();
}

void long_running(int x)
{
    int sumi = x;
    for (int i = 1; i < x; i++)
    {
        sumi *= i;
    }
    collect_dat(x, "https://en.wikipedia.org/wiki/Immanuel_Kant");
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
        fibers::fiber f(long_running(i));
    }
    return 0;
}