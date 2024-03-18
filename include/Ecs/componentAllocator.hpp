#pragma once
#include "Ecs/entityManager.hpp"
#include "EcsConstants.hpp"
#include "pch.hpp"
#include "util/contracts.hpp"
#include "util/macros.hpp"
#include "util/types.hpp"
#include <type_traits>
#include <unordered_map>

class IComponentAllocator
{
  public:
    virtual void FreeComponent(EntityID id) = 0;
    virtual ~IComponentAllocator() = default;
};

template <typename ComponentType> class ComponentAllocator : IComponentAllocator
{
  public:
    ComponentAllocator() { m_typeName = typeid(ComponentType).name(); }

    NO_COPY_OR_MOVE_CONSTRUCTORS(ComponentAllocator)

    template <typename... Args>
      requires std::is_constructible_v<ComponentType, Args...>
    void AllocateComponent(EntityID id, Args &&...args)
    {
      Requires(
          !m_indexMap.contains(id),
          std::format(
              "Entity of id {} tried allocating component of type {} twice!",
              id, m_typeName));

      usize newIndex = m_size;
      m_components[newIndex] = T(std::forward<Args>(args)...);
      m_indexMap[id] = newIndex;
      m_entityMap[newIndex] = id;

      m_size++;
    }

    void FreeComponent(EntityID id)
    {
      Requires(m_indexMap.contains(id));

      usize lastIndex = m_components.size() - 1;
      usize index = m_indexMap.at(id);
      Assert(0 <= index && index < m_components.size());
      Assert(m_entityMap.contains(index));

      EntityID movedID = m_entityMap.at(lastIndex);
      m_indexMap[movedID] = index;
      m_indexMap[index] = movedID;
      m_components[index] = std::move(m_components[lastIndex]);

      m_components.pop_back();
      m_entityMap.erase(id);
      m_indexMap.erase(lastIndex);
    }

    ComponentType &GetComponent(EntityID id)
    {
      Requires(m_indexMap.contains(id),
               std::format("Entity does not have component of type {}!",
                           m_typeName));

      usize index = m_indexMap.at(id);
      Assert(0 <= index && index < m_size);

      return m_components[index];
    }

    const ComponentType &GetConstComponent(EntityID id) const
    {
      return GetComponent(id);
    }

  private:
    std::array<ComponentType, MAX_ENTITIES> m_components;
    std::unordered_map<EntityID, usize> m_indexMap;
    std::unordered_map<usize, EntityID> m_entityMap;
    usize m_size = 0;
    std::string m_typeName;
};