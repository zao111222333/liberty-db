// g++ -std=c++17 -fPIC -shared -I. -o libprojs.so projs.cpp ot/liberty/*.cpp ot/unit/*.cpp -lstdc++fs

#include <filesystem>
#include <ot/liberty/celllib.hpp>

// OpenTimer
extern "C" void ot_parse_lib(const char* path) {
    ot::Celllib lib;
    lib.read(std::filesystem::path(path));
}