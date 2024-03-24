#include "Ecs/component.hpp"
#include "Ecs/signature.hpp"
#include "util/NDArray.hpp"
#include <expected>
#include <print>
void *operator new(usize stuff) { return malloc(stuff); }

int main() { std::vector<int> a = {1};
a.clear(); }
