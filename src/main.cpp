#include <iostream>
#include <expected>
#include <print>
#include "hello.hpp"
#include "event/eventManager.hpp"


int main()
{
  
    auto man = EventManager();
    man.Subscribe<Event>([](auto e){});
}