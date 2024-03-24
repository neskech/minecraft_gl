#pragma once

namespace ECS::Component
{
  struct Component
  {
      virtual ~Component() {}
  };

  struct LargeComponent : Component
  {
      virtual ~LargeComponent() {}
  };
  
} // namespace Component