#include <iostream>
#include <expected>
#include <print>
#include "hello.hpp"

int main()
{
    std::println("hello {} {}", 5, "fuck");
    auto s = Sex();
    s.fuck();
}