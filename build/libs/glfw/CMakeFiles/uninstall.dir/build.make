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

# Utility rule file for uninstall.

# Include any custom commands dependencies for this target.
include libs/glfw/CMakeFiles/uninstall.dir/compiler_depend.make

# Include the progress variables for this target.
include libs/glfw/CMakeFiles/uninstall.dir/progress.make

libs/glfw/CMakeFiles/uninstall:
	cd /home/ness/Projects/Personal/GameDev/Minecraft/MinecraftCpp/build/libs/glfw && /home/linuxbrew/.linuxbrew/Cellar/cmake/3.28.3/bin/cmake -P /home/ness/Projects/Personal/GameDev/Minecraft/MinecraftCpp/build/libs/glfw/cmake_uninstall.cmake

uninstall: libs/glfw/CMakeFiles/uninstall
uninstall: libs/glfw/CMakeFiles/uninstall.dir/build.make
.PHONY : uninstall

# Rule to build all files generated by this target.
libs/glfw/CMakeFiles/uninstall.dir/build: uninstall
.PHONY : libs/glfw/CMakeFiles/uninstall.dir/build

libs/glfw/CMakeFiles/uninstall.dir/clean:
	cd /home/ness/Projects/Personal/GameDev/Minecraft/MinecraftCpp/build/libs/glfw && $(CMAKE_COMMAND) -P CMakeFiles/uninstall.dir/cmake_clean.cmake
.PHONY : libs/glfw/CMakeFiles/uninstall.dir/clean

libs/glfw/CMakeFiles/uninstall.dir/depend:
	cd /home/ness/Projects/Personal/GameDev/Minecraft/MinecraftCpp/build && $(CMAKE_COMMAND) -E cmake_depends "Unix Makefiles" /home/ness/Projects/Personal/GameDev/Minecraft/MinecraftCpp /home/ness/Projects/Personal/GameDev/Minecraft/MinecraftCpp/libs/glfw /home/ness/Projects/Personal/GameDev/Minecraft/MinecraftCpp/build /home/ness/Projects/Personal/GameDev/Minecraft/MinecraftCpp/build/libs/glfw /home/ness/Projects/Personal/GameDev/Minecraft/MinecraftCpp/build/libs/glfw/CMakeFiles/uninstall.dir/DependInfo.cmake "--color=$(COLOR)"
.PHONY : libs/glfw/CMakeFiles/uninstall.dir/depend

