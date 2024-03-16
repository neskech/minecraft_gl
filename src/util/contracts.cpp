#include "util/contracts.hpp"
#include <cstdlib>

void Requires(bool condition, const std::string_view errMsg)
{
  if (!condition) {
    std::print("\033[31m {}", errMsg);
    assert(false);
  }
}

void Assert(bool condition, const std::string_view errMsg)
{
  if (!condition) {
    std::println("\033[31m {}", errMsg);
    assert(false);
  }
}

void Ensures(bool condition, const std::string_view errMsg)
{
  if (!condition) {
    std::print("\033[31m {}", errMsg);
    assert(false);
  }
}