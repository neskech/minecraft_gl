#include "Ecs/components/script.hpp"
#include "components/transform.hpp"
#include "entity.hpp"
#include "entityWrapper.hpp"

namespace ECS::Component
{
  ScriptBase::ScriptBase(EntityWrapper entity, Input &input)
      : m_entity(entity), m_input(input)
  {
    if (!m_entity.HasComponent<Transform>())
      m_entity.AddComponent<Transform>();
  }

  EntityWrapper ScriptBase::MakeEntity()
  {
    Entity ent = m_entity.GetECS().MakeEntity();
    return EntityWrapper(ent, m_entity.GetECS());
  }

  Transform &ScriptBase::GetTransform()
  {
    return m_entity.GetComponent<Transform>();
  }

  void ScriptBase::DestroyImmediate() { m_entity.Destroy(); }
} // namespace ECS::Component