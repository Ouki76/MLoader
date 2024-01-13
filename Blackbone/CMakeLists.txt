cmake_minimum_required (VERSION 3.13)
project (injector)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS}")

cmake_policy(SET CMP0015 NEW)

add_compile_definitions(_SILENCE_CXX17_CODECVT_HEADER_DEPRECATION_WARNING _SILENCE_ALL_CXX17_DEPRECATION_WARNINGS)

add_library(injector SHARED main.cpp)

add_subdirectory(./3rd_party)
add_subdirectory(./src)