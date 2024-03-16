#pragma once
#include "pch.hpp"

void Requires(bool condition, const std::string_view errMsg = "");
void Assert(bool condition, const std::string_view errMsg = "");
void Ensures(bool condition, const std::string_view errMsg = "");