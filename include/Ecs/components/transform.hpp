#pragma once
#include "component.hpp"
#include "util/types.hpp"

namespace ECS::Component
{
  struct Transform : Component::Component
  {

      Vector3 position;
  };
} // namespace ECS::Component