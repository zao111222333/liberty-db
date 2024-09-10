// g++ -std=c++17 -fPIC -shared -I. -o libprojs.so projs.cpp ot/liberty/*.cpp ot/unit/*.cpp -lstdc++fs

#include <filesystem>
#include <iostream>
#include <sstream>
#include <vector>
#include <cstring>
#include <ot/liberty/celllib.hpp>

// OpenTimer
extern "C" void* ot_parse_lib(const char* s) {
    auto* lib = new ot::Celllib();
    size_t fsize = std::strlen(s);
    std::vector<char> buffer(fsize + 1);
    std::memcpy(buffer.data(), s, fsize);
    buffer[fsize] = '\0';
    lib->read_buffer(buffer);
    return static_cast<void*>(lib);
}

extern "C" void ot_write_lib(void* ptr) {
    auto* lib = static_cast<ot::Celllib*>(ptr);
    std::stringstream ss;
    ss << *lib;
    asm volatile("" : : "g"(&ss) : "memory");
    return;
}

extern "C" void ot_drop_lib(void* ptr) {
    auto* lib = static_cast<ot::Celllib*>(ptr);
    delete lib;
    return;
}