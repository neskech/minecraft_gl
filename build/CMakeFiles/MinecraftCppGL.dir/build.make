# CMAKE generated file: DO NOT EDIT!
# Generated by "Unix Makefiles" Generator, CMake Version 3.28

# Delete rule output on recipe failure.
.DELETE_ON_ERROR:

#=============================================================================
# Special targets provided by cmake.

# Disable implicit rules so canonical targets will work.
.SUFFIXES:

# Disable VCS-based implicit rules.
% : %,v

# Disable VCS-based implicit rules.
% : RCS/%

# Disable VCS-based implicit rules.
% : RCS/%,v

# Disable VCS-based implicit rules.
% : SCCS/s.%

# Disable VCS-based implicit rules.
% : s.%

.SUFFIXES: .hpux_make_needs_suffix_list

# Command-line flag to silence nested $(MAKE).
$(VERBOSE)MAKESILENT = -s

#Suppress display of executed commands.
$(VERBOSE).SILENT:

# A target that is always out of date.
cmake_force:
.PHONY : cmake_force

#=============================================================================
# Set environment variables for the build.

# The shell in which to execute make rules.
SHELL = /bin/sh

# The CMake executable.
CMAKE_COMMAND = /home/linuxbrew/.linuxbrew/Cellar/cmake/3.28.3/bin/cmake

# The command to remove a file.
RM = /home/linuxbrew/.linuxbrew/Cellar/cmake/3.28.3/bin/cmake -E rm -f

# Escaping for special characters.
EQUALS = =

# The top-level source directory on which CMake was run.
CMAKE_SOURCE_DIR = /home/ness/Projects/Personal/GameDev/Minecraft/MinecraftCpp

# The top-level build directory on which CMake was run.
CMAKE_BINARY_DIR = /home/ness/Projects/Personal/GameDev/Minecraft/MinecraftCpp/build

# Include any dependencies generated for this target.
include CMakeFiles/MinecraftCppGL.dir/depend.make
# Include any dependencies generated by the compiler for this target.
include CMakeFiles/MinecraftCppGL.dir/compiler_depend.make

# Include the progress variables for this target.
include CMakeFiles/MinecraftCppGL.dir/progress.make

# Include the compile flags for this target's objects.
include CMakeFiles/MinecraftCppGL.dir/flags.make

CMakeFiles/MinecraftCppGL.dir/cmake_pch.hxx.pch: CMakeFiles/MinecraftCppGL.dir/flags.make
CMakeFiles/MinecraftCppGL.dir/cmake_pch.hxx.pch: CMakeFiles/MinecraftCppGL.dir/cmake_pch.hxx.cxx
CMakeFiles/MinecraftCppGL.dir/cmake_pch.hxx.pch: CMakeFiles/MinecraftCppGL.dir/cmake_pch.hxx
CMakeFiles/MinecraftCppGL.dir/cmake_pch.hxx.pch: CMakeFiles/MinecraftCppGL.dir/compiler_depend.ts
	@$(CMAKE_COMMAND) -E cmake_echo_color "--switch=$(COLOR)" --green --progress-dir=/home/ness/Projects/Personal/GameDev/Minecraft/MinecraftCpp/build/CMakeFiles --progress-num=$(CMAKE_PROGRESS_1) "Building CXX object CMakeFiles/MinecraftCppGL.dir/cmake_pch.hxx.pch"
	/home/linuxbrew/.linuxbrew/bin/clang++ $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -Winvalid-pch -fpch-instantiate-templates -Xclang -emit-pch -Xclang -include -Xclang /home/ness/Projects/Personal/GameDev/Minecraft/MinecraftCpp/build/CMakeFiles/MinecraftCppGL.dir/cmake_pch.hxx -x c++-header -MD -MT CMakeFiles/MinecraftCppGL.dir/cmake_pch.hxx.pch -MF CMakeFiles/MinecraftCppGL.dir/cmake_pch.hxx.pch.d -o CMakeFiles/MinecraftCppGL.dir/cmake_pch.hxx.pch -c /home/ness/Projects/Personal/GameDev/Minecraft/MinecraftCpp/build/CMakeFiles/MinecraftCppGL.dir/cmake_pch.hxx.cxx

CMakeFiles/MinecraftCppGL.dir/cmake_pch.hxx.i: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color "--switch=$(COLOR)" --green "Preprocessing CXX source to CMakeFiles/MinecraftCppGL.dir/cmake_pch.hxx.i"
	/home/linuxbrew/.linuxbrew/bin/clang++ $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -Winvalid-pch -fpch-instantiate-templates -Xclang -emit-pch -Xclang -include -Xclang /home/ness/Projects/Personal/GameDev/Minecraft/MinecraftCpp/build/CMakeFiles/MinecraftCppGL.dir/cmake_pch.hxx -x c++-header -E /home/ness/Projects/Personal/GameDev/Minecraft/MinecraftCpp/build/CMakeFiles/MinecraftCppGL.dir/cmake_pch.hxx.cxx > CMakeFiles/MinecraftCppGL.dir/cmake_pch.hxx.i

CMakeFiles/MinecraftCppGL.dir/cmake_pch.hxx.s: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color "--switch=$(COLOR)" --green "Compiling CXX source to assembly CMakeFiles/MinecraftCppGL.dir/cmake_pch.hxx.s"
	/home/linuxbrew/.linuxbrew/bin/clang++ $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -Winvalid-pch -fpch-instantiate-templates -Xclang -emit-pch -Xclang -include -Xclang /home/ness/Projects/Personal/GameDev/Minecraft/MinecraftCpp/build/CMakeFiles/MinecraftCppGL.dir/cmake_pch.hxx -x c++-header -S /home/ness/Projects/Personal/GameDev/Minecraft/MinecraftCpp/build/CMakeFiles/MinecraftCppGL.dir/cmake_pch.hxx.cxx -o CMakeFiles/MinecraftCppGL.dir/cmake_pch.hxx.s

CMakeFiles/MinecraftCppGL.dir/src/main.cpp.o: CMakeFiles/MinecraftCppGL.dir/flags.make
CMakeFiles/MinecraftCppGL.dir/src/main.cpp.o: /home/ness/Projects/Personal/GameDev/Minecraft/MinecraftCpp/src/main.cpp
CMakeFiles/MinecraftCppGL.dir/src/main.cpp.o: CMakeFiles/MinecraftCppGL.dir/cmake_pch.hxx
CMakeFiles/MinecraftCppGL.dir/src/main.cpp.o: CMakeFiles/MinecraftCppGL.dir/cmake_pch.hxx.pch
CMakeFiles/MinecraftCppGL.dir/src/main.cpp.o: CMakeFiles/MinecraftCppGL.dir/compiler_depend.ts
	@$(CMAKE_COMMAND) -E cmake_echo_color "--switch=$(COLOR)" --green --progress-dir=/home/ness/Projects/Personal/GameDev/Minecraft/MinecraftCpp/build/CMakeFiles --progress-num=$(CMAKE_PROGRESS_2) "Building CXX object CMakeFiles/MinecraftCppGL.dir/src/main.cpp.o"
	/home/linuxbrew/.linuxbrew/bin/clang++ $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -Winvalid-pch -Xclang -include-pch -Xclang /home/ness/Projects/Personal/GameDev/Minecraft/MinecraftCpp/build/CMakeFiles/MinecraftCppGL.dir/cmake_pch.hxx.pch -Xclang -include -Xclang /home/ness/Projects/Personal/GameDev/Minecraft/MinecraftCpp/build/CMakeFiles/MinecraftCppGL.dir/cmake_pch.hxx -MD -MT CMakeFiles/MinecraftCppGL.dir/src/main.cpp.o -MF CMakeFiles/MinecraftCppGL.dir/src/main.cpp.o.d -o CMakeFiles/MinecraftCppGL.dir/src/main.cpp.o -c /home/ness/Projects/Personal/GameDev/Minecraft/MinecraftCpp/src/main.cpp

CMakeFiles/MinecraftCppGL.dir/src/main.cpp.i: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color "--switch=$(COLOR)" --green "Preprocessing CXX source to CMakeFiles/MinecraftCppGL.dir/src/main.cpp.i"
	/home/linuxbrew/.linuxbrew/bin/clang++ $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -Winvalid-pch -Xclang -include-pch -Xclang /home/ness/Projects/Personal/GameDev/Minecraft/MinecraftCpp/build/CMakeFiles/MinecraftCppGL.dir/cmake_pch.hxx.pch -Xclang -include -Xclang /home/ness/Projects/Personal/GameDev/Minecraft/MinecraftCpp/build/CMakeFiles/MinecraftCppGL.dir/cmake_pch.hxx -E /home/ness/Projects/Personal/GameDev/Minecraft/MinecraftCpp/src/main.cpp > CMakeFiles/MinecraftCppGL.dir/src/main.cpp.i

CMakeFiles/MinecraftCppGL.dir/src/main.cpp.s: cmake_force
	@$(CMAKE_COMMAND) -E cmake_echo_color "--switch=$(COLOR)" --green "Compiling CXX source to assembly CMakeFiles/MinecraftCppGL.dir/src/main.cpp.s"
	/home/linuxbrew/.linuxbrew/bin/clang++ $(CXX_DEFINES) $(CXX_INCLUDES) $(CXX_FLAGS) -Winvalid-pch -Xclang -include-pch -Xclang /home/ness/Projects/Personal/GameDev/Minecraft/MinecraftCpp/build/CMakeFiles/MinecraftCppGL.dir/cmake_pch.hxx.pch -Xclang -include -Xclang /home/ness/Projects/Personal/GameDev/Minecraft/MinecraftCpp/build/CMakeFiles/MinecraftCppGL.dir/cmake_pch.hxx -S /home/ness/Projects/Personal/GameDev/Minecraft/MinecraftCpp/src/main.cpp -o CMakeFiles/MinecraftCppGL.dir/src/main.cpp.s

# Object files for target MinecraftCppGL
MinecraftCppGL_OBJECTS = \
"CMakeFiles/MinecraftCppGL.dir/src/main.cpp.o"

# External object files for target MinecraftCppGL
MinecraftCppGL_EXTERNAL_OBJECTS =

MinecraftCppGL: CMakeFiles/MinecraftCppGL.dir/cmake_pch.hxx.pch
MinecraftCppGL: CMakeFiles/MinecraftCppGL.dir/src/main.cpp.o
MinecraftCppGL: CMakeFiles/MinecraftCppGL.dir/build.make
MinecraftCppGL: libs/glfw/src/libglfw3.a
MinecraftCppGL: libs/Glad/libglad.a
MinecraftCppGL: libs/stb_image/libstb_image.a
MinecraftCppGL: /usr/lib/x86_64-linux-gnu/librt.a
MinecraftCppGL: /usr/lib/x86_64-linux-gnu/libm.so
MinecraftCppGL: CMakeFiles/MinecraftCppGL.dir/link.txt
	@$(CMAKE_COMMAND) -E cmake_echo_color "--switch=$(COLOR)" --green --bold --progress-dir=/home/ness/Projects/Personal/GameDev/Minecraft/MinecraftCpp/build/CMakeFiles --progress-num=$(CMAKE_PROGRESS_3) "Linking CXX executable MinecraftCppGL"
	$(CMAKE_COMMAND) -E cmake_link_script CMakeFiles/MinecraftCppGL.dir/link.txt --verbose=$(VERBOSE)

# Rule to build all files generated by this target.
CMakeFiles/MinecraftCppGL.dir/build: MinecraftCppGL
.PHONY : CMakeFiles/MinecraftCppGL.dir/build

CMakeFiles/MinecraftCppGL.dir/clean:
	$(CMAKE_COMMAND) -P CMakeFiles/MinecraftCppGL.dir/cmake_clean.cmake
.PHONY : CMakeFiles/MinecraftCppGL.dir/clean

CMakeFiles/MinecraftCppGL.dir/depend:
	cd /home/ness/Projects/Personal/GameDev/Minecraft/MinecraftCpp/build && $(CMAKE_COMMAND) -E cmake_depends "Unix Makefiles" /home/ness/Projects/Personal/GameDev/Minecraft/MinecraftCpp /home/ness/Projects/Personal/GameDev/Minecraft/MinecraftCpp /home/ness/Projects/Personal/GameDev/Minecraft/MinecraftCpp/build /home/ness/Projects/Personal/GameDev/Minecraft/MinecraftCpp/build /home/ness/Projects/Personal/GameDev/Minecraft/MinecraftCpp/build/CMakeFiles/MinecraftCppGL.dir/DependInfo.cmake "--color=$(COLOR)"
.PHONY : CMakeFiles/MinecraftCppGL.dir/depend

