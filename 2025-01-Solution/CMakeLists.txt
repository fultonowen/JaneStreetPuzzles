cmake_minimum_required(VERSION 3.30)
project(2025_01_Solution)

set(CMAKE_CXX_STANDARD 20)
if(NOT CMAKE_BUILD_TYPE)
    set(CMAKE_BUILD_TYPE Release)
endif()

set(CMAKE_CXX_FLAGS "-Wall -Wextra")
set(CMAKE_CXX_FLAGS_DEBUG "-g")
set(CMAKE_CXX_FLAGS_RELEASE "-O3")

add_library(board lib/Game.cpp lib/Solver.cpp lib/Core.cpp)
target_include_directories(board PUBLIC include)

add_executable(2025_01_Solution main.cpp)
target_link_libraries(2025_01_Solution board)
