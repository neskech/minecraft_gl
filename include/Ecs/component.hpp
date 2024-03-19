#pragma once

namespace Component
{
  struct Component
  {
      virtual ~Component() {}
  };

  struct LargeComponent : Component
  {
      virtual ~LargeComponent() {}
  };

  struct Transform : Component
  {
  };

  struct Chunk : LargeComponent
  {
  };
} // namespace Component