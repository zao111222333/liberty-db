################################################################################
# Fetch files
################################################################################

file(GLOB LIB_SOURCES "*.c")
list(REMOVE_ITEM LIB_SOURCES "main.c")
message(${LIB_SOURCES})
file(GLOB LIB_HEADERS "../include/*.h")

################################################################################
# Set variables
################################################################################

SET(CMAKE_FIND_LIBRARY_SUFFIXES ".a")
SET(BUILD_SHARED_LIBRARIES OFF)
SET(CMAKE_EXE_LINKER_FLAGS "-static")

################################################################################
# Library target
################################################################################

# Add library target
add_library(si2dr_liberty STATIC ${LIB_SOURCES} ${LIB_HEADERS})

# Tell cmake target's dependencies
target_include_directories(si2dr_liberty
    PUBLIC ${CMAKE_CURRENT_SOURCE_DIR}/../include
)

################################################################################
# Installation rules
################################################################################

# Install rule for target
install(
    TARGETS si2dr_liberty
    DESTINATION lib
    EXPORT si2dr_libertyConfig
)

# Install rule for headers
install(
    FILES ${CMAKE_CURRENT_SOURCE_DIR}/../include/si2dr_liberty.h
    DESTINATION include
)