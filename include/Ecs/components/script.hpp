#pragma once
#include "component.hpp"
#include "entityManager.hpp"
#include "util/input.hpp"

namespace Component
{
  class ScriptBase : Component::Component
  {
    public:
      ScriptBase(EntityComponentSystem &ecs, Input &input, Entity entity)
          : m_Ecs(ecs), m_input(input), m_entity(entity)
      {}

      virtual void Start() {}
      virtual void Update() {}
      virtual void OnDestroy() {}

    protected:
      EntityComponentSystem &m_Ecs;
      Input &m_input;
      Entity m_entity;
  };
} // namespace Component