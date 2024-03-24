#pragma once
#include "EcsConstants.hpp"
#include "util/types.hpp"

namespace ECS
{
  class EntityManager;
  typedef usize EntityID;

  class Entity
  {
    public:
      friend class EntityManager;

      inline EntityID GetID() { return m_id; }
      inline bool HasValidID() { return 0 <= m_id && m_id < MAX_ENTITIES; }

      bool operator==(const Entity &other) const { return m_id == other.m_id; }
      struct Hasher
      {

          usize operator()(const Entity &handle) const
          {
            return std::hash<usize>{}(handle.m_id);
          }
      };

    private:
      explicit Entity(EntityID id) : m_id(id) {}
      EntityID m_id;
  };

} // namespace ECS