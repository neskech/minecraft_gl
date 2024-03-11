cmake_minimum_required(VERSION 3.28.3)
set(CMAKE_CXX_STANDARD 23)
set(CMAKE_CXX_STANDARD_REQUIRED on)
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)
set(CMAKE_CXX_COMPILER "clang++")
set(CMAKE_CXX_FLAGS "-pedantic -Wall -stdlib=libc++ -std=c++23")

if (CMAKE_BUILD_TYPE MATCHES Release)
    set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -O3")
else ()
    set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -g")
endif ()

project(MinecraftCppGL)

file(GLOB_RECURSE file_sources src/*.cpp)
add_executable(${PROJECT_NAME} ${file_sources})
target_include_directories(${PROJECT_NAME} PUBLIC ${PROJECT_SOURCE_DIR}/include)

# GLFW
add_subdirectory(libs/glfw EXCLUDE_FROM_ALL)

# GLAD
add_subdirectory(libs/Glad EXCLUDE_FROM_ALL)

# GLM
add_subdirectory(libs/glm EXCLUDE_FROM_ALL)

# FAST NOISE
target_include_directories(${PROJECT_NAME} PUBLIC libs/FastNoiseLite/include)

# STB IMAGE
add_subdirectory(libs/stb_image EXCLUDE_FROM_ALL)

target_link_libraries(${PROJECT_NAME} glfw glm glad stb_image)