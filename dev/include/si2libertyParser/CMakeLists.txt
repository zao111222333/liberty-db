project(Liberty C)
cmake_minimum_required(VERSION 3.2.2)

set(CMAKE_MODULE_PATH ${CMAKE_MODULE_PATH} ${CMAKE_CURRENT_SOURCE_DIR}/cmake_modules)

add_definitions(-DVERSION="2.5")
add_subdirectory(src)
