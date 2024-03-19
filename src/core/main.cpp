#include "Ecs/component.hpp"
#include "Ecs/signature.hpp"
#include <expected>
#include <print>
struct h{};
int main()
{
  auto sig = SignatureBuilder()
                 .AddComponentType<h>()
                 .Finish();
}
