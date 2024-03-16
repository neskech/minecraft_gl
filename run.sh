#!/bin/bash

if [[ "$1" = "clean" ]]; then
    rm -rf ./build/**
    rm -rf ./build/.cache/
    rm -rf ./build/CMakeFiles/
elif [[ "$1" = "val" ]]; then
    cmake -S . -B ./build -DCMAKE_BUILD_TYPE="Debug"
    cd build
    make
    valgrind ./MinecraftCppGL
else
    cmake -S . -B ./build -DCMAKE_BUILD_TYPE="$1"
    cd build
    make
    ./MinecraftCppGL
fi
