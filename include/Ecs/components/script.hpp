#pragma once
#include "component.hpp"
#include "components/transform.hpp"
#include "entityWrapper.hpp"
#include "util/input.hpp"

namespace ECS::Component
{
  class ScriptBase : Component::Component
  {
    public:
      ScriptBase(EntityWrapper entity, Input &input);

      virtual void Start() = 0;
      virtual void Update() = 0;
      virtual void OnDestroy() = 0;

      EntityWrapper MakeEntity();
      Transform &GetTransform();
      void DestroyImmediate();

    protected:
      EntityWrapper m_entity;
      Input &m_input;
  };
} // namespace ECS::Component