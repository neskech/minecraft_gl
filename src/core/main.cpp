#include "eventManager.hpp"
#include "util/contracts.hpp"
#include <expected>
#include <print>

struct Shit
{
};
void s(const Shit &shit) { std::println("FUCK"); }

int main()
{
  auto handle = EventManager::Subscribe<Shit>(s);
  EventManager::Invoke(Shit());
  EventManager::UnSubscribe<Shit>(handle);
  EventManager::Invoke(Shit());
}
