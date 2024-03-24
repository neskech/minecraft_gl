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
    virtual void FreeComponent(Entity entity) = 0;
    virtual ~IComponentAllocator() = default;
};

template <typename ComponentType>
class ComponentAllocator : IComponentAllocator
{
  public:
    ComponentAllocator() { m_typeName = typeid(ComponentType).name(); }

    NO_COPY_OR_MOVE_CONSTRUCTORS(ComponentAllocator)

    template <typename... Args>
      requires std::is_constructible_v<ComponentType, Args...>
    void AllocateComponent(Entity entity, Args &&...args)
    {
      Requires(!m_indexMap.contains(entity.GetID()),
               std::format("Entity of entity.GetID() {} tried allocating "
                           "component of type {} twice!",
                           entity.GetID(), m_typeName));

      usize newIndex = m_size;
      m_components[newIndex] = T(std::forward<Args>(args)...);
      m_indexMap[entity.GetID()] = newIndex;
      m_entityMap[newIndex] = entity.GetID();

      m_size++;
    }

    void FreeComponent(Entity entity)
    {
      Requires(m_indexMap.contains(entity.GetID()));

      usize lastIndex = m_size - 1;
      usize index = m_indexMap.at(entity.GetID());
      Assert(0 <= index && index < m_size);
      Assert(m_entityMap.contains(index));

      EntityID movedID = m_entityMap.at(lastIndex);
      m_indexMap[movedID] = index;
      m_indexMap[index] = movedID;
      m_components[index] = std::move(m_components[lastIndex]);

      m_size--;
      m_entityMap.erase(entity.GetID());
      m_indexMap.erase(lastIndex);
    }

    ComponentType &GetComponent(Entity entity)
    {
      Requires(m_indexMap.contains(entity.GetID()),
               std::format("Entity does not have component of type {}!",
                           m_typeName));

      usize index = m_indexMap.at(entity.GetID());
      Assert(0 <= index && index < m_size);

      return m_components[index];
    }

  private:
    std::array<ComponentType, MAX_ENTITIES> m_components;
    std::unordered_map<EntityID, usize> m_indexMap;
    std::unordered_map<usize, EntityID> m_entityMap;
    usize m_size = 0;
    std::string m_typeName;
};

template <typename ComponentType>
class DynamicComponentAllocator : IComponentAllocator
{
  public:
    DynamicComponentAllocator() { m_typeName = typeid(ComponentType).name(); }

    NO_COPY_OR_MOVE_CONSTRUCTORS(DynamicComponentAllocator)

    template <typename... Args>
      requires std::is_constructible_v<ComponentType, Args...>
    void AllocateComponent(EntityID id, Args &&...args)
    {
      Requires(
          !m_indexMap.contains(id),
          std::format(
              "Entity of id {} tried allocating component of type {} twice!",
              id, m_typeName));

      usize newIndex = m_components.size();
      m_components.emplace_back(std::forward<Args>(args)...);
      m_indexMap[id] = newIndex;
      m_entityMap[newIndex] = id;
    }

    void FreeComponent(Entity entity)
    {
      Requires(m_indexMap.contains(entity.GetID()));

      usize lastIndex = m_components.size() - 1;
      usize index = m_indexMap.at(entity.GetID());
      Assert(0 <= index && index < m_components.size());
      Assert(m_entityMap.contains(index));

      EntityID movedID = m_entityMap.at(lastIndex);
      m_indexMap[movedID] = index;
      m_indexMap[index] = movedID;
      m_components[index] = std::move(m_components[lastIndex]);

      m_components.pop_back();
      m_entityMap.erase(entity.GetID());
      m_indexMap.erase(lastIndex);
    }

    ComponentType &GetComponent(Entity entity)
    {
      Requires(m_indexMap.contains(entity.GetID()),
               std::format("Entity does not have component of type {}!",
                           m_typeName));

      usize index = m_indexMap.at(entity.GetID());
      Assert(0 <= index && index < m_components.size());

      return m_components[index];
    }

  private:
    std::vector<ComponentType> m_components;
    std::unordered_map<EntityID, usize> m_indexMap;
    std::unordered_map<usize, EntityID> m_entityMap;
    std::string m_typeName;
};