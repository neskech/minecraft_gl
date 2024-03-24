#pragma once

#include "entityManager.hpp"
class EntityWrapper
{
  public:
    EntityWrapper(Entity entity, EntityComponentSystem &ecs)
        : m_entity(entity), m_Ecs(ecs)
    {}

    inline Option<Entity> GetParent() const {}

    inline std::vector<Entity> &GetChildren() {}

    inline std::string &GetName() {}

    inline std::string &GetTag() {}

    inline void SetName(std::string_view name) {}

    inline void SetTag(std::string_view tag) {}

    inline 

  private:
    Entity m_entity;
    EntityComponentSystem &m_Ecs;
};

using EntityW = EntityWrapper;