#pragma once
#include "component.hpp"
#include "util/types.hpp"

namespace ECS::Component
{
  class Camera : Component::Component
  {
    public:
      Camera();

    private:
      f32 fieldOfView;
  };
} // namespace ECS::Component