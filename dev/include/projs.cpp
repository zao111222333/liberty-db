// g++ -std=c++17 -fPIC -shared -I. -o libprojs.so projs.cpp ot/liberty/*.cpp ot/unit/*.cpp -L./si2libertyParser/build -lsi2dr_liberty -lstdc++fs

#include <filesystem>
#include <iostream>
#include <sstream>
#include <vector>
#include <cstring>
#include <ot/liberty/celllib.hpp>
#include "si2libertyParser/include/si2dr_liberty.h"
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

extern "C" void ot_write_lib(void* ptr, int debug) {
    auto* lib = static_cast<ot::Celllib*>(ptr);
    if (debug) {
        std::cout << *lib;
    } else {
        std::stringstream ss;
        ss << *lib;
        asm volatile("" : : "g"(&ss) : "memory");
    }
    return;
}

extern "C" void ot_drop_lib(void* ptr) {
    auto* lib = static_cast<ot::Celllib*>(ptr);
    delete lib;
    return;
}

extern "C" int liberty_parser_parse(void);

// si2dr_liberty
extern "C" void* si2dr_liberty_parse_lib(const char* s) {
    auto* err = new si2drErrorT();
    si2drGroupsIdT groups;
    si2drGroupIdT group;
    extern FILE *liberty_parser2_in;
    extern int syntax_errors;
    liberty_parser2_in = fmemopen((void *)s, strlen(s), "r");
    if (liberty_parser2_in == NULL) {
        perror("fmemopen failed");
        return nullptr;
    }
    si2drPIInit(&*err);
    *err = SI2DR_NO_ERROR;
    liberty_parser_parse();
    si2drPIUnSetNocheckMode(err);
    fclose(liberty_parser2_in);
    if( syntax_errors )
      *err = SI2DR_SYNTAX_ERROR;
    // char* filename = new char[strlen(s) + 1]; // +1 for null terminator
    // std::strcpy(filename, s);
    // si2drReadLibertyFile(filename, &*err);
    // if( *err == SI2DR_INVALID_NAME )
	// {
	// 	std::cout << "COULD NOT OPEN "<< filename << " for parsing-- quitting...\n";
	// 	exit(301);
	// }
	else if (*err == SI2DR_SYNTAX_ERROR )
	{
        std::cout << "Syntax Errors were detected in the input file!\n\n";
		exit(401);
	}
    groups = si2drPIGetGroups(&*err);
    while( !si2drObjectIsNull((group=si2drIterNextGroup(groups,&*err)),&*err) )
	{
		
		std::cout << "\n\nChecking the database...\n\n";
		si2drCheckLibertyLibrary(group, &*err);

		if( *err == SI2DR_NO_ERROR )
			std::cout << "Passed\n\n";
		else
			std::cout << "Errors detected\n\n";

	}
	si2drIterQuit(groups,&*err);
    return static_cast<void*>(err);
}

extern "C" void si2dr_liberty_write_lib(void* ptr, int debug) {
    char buffer[10000];
    memset(buffer, 0, sizeof(buffer));
    auto* err = static_cast<si2drErrorT*>(ptr);
    si2drGroupIdT group;
    si2drGroupsIdT groups = si2drPIGetGroups(&*err);
    FILE *mem_file = fmemopen(buffer, sizeof(buffer), "w");
    if (mem_file == NULL) {
        perror("fmemopen failed");
        return;
    }
    while( !si2drObjectIsNull((group=si2drIterNextGroup(groups,&*err)),&*err) )
    {
        si2drNamesIdT gnames;
        si2drStringT gname;
        
        gnames = si2drGroupGetNames(group, &*err);
        gname = si2drIterNextName(gnames,&*err);
        si2drIterQuit(gnames,&*err);
        lib__write_group(mem_file, group, "", NULL);
        // si2drWriteLibertyFile(buf1, group, NULL, &*err);
    }
    si2drIterQuit(groups,&*err);
    fclose(mem_file);
    if (debug)
        printf("%s", buffer);
    asm volatile("" : : "g"(&buffer) : "memory");
    return;
}

extern "C" void si2dr_liberty_drop_lib(void* ptr) {
    auto* err = static_cast<si2drErrorT*>(ptr);
    delete err;
    return;
}