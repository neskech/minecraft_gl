#pragma once
#include "Ecs/EcsConstants.hpp"
#include "Ecs/component.hpp"
#include "Ecs/componentAllocator.hpp"
#include "Ecs/typeId.hpp"
#include "pch.hpp"
#include "util/contracts.hpp"
#include "util/macros.hpp"
#include "util/types.hpp"
#include <type_traits>

class ComponentManager
{
  public:
    ComponentManager() {}
    NO_COPY_OR_MOVE_CONSTRUCTORS(ComponentManager)

    template <typename ComponentType, typename... Args>
      requires std::is_constructible_v<ComponentType, Args...>
    void AddComponent(EntityID id, Args &&...args);

    template <typename ComponentType> ComponentType &GetComponent(EntityID id);

    template <typename ComponentType> void DeleteComponent(EntityID id);

    void EntityDestroyed(EntityID id, const EntityManager &manager);

    template <typename ComponentType> inline usize ComponentID();

  private:
    template <typename ComponentType>
    ComponentAllocator<ComponentType> &GetComponentAllocator()
    {
      if (!DoesAllocatorExist<ComponentType>())
        MakeComponentAllocator<ComponentType>();

      usize id = ComponentID<ComponentType>();
      auto *c = dynamic_cast<ComponentAllocator<ComponentType> *>(
          m_componentLists[id].get());
      return *c;
    }

    template <typename ComponentType>
    DynamicComponentAllocator<ComponentType> &GetDynamicComponentAllocator()
    {
      if (!DoesAllocatorExist<ComponentType>())
        MakeComponentAllocator<ComponentType>();

      usize id = ComponentID<ComponentType>();
      auto *c = dynamic_cast<DynamicComponentAllocator<ComponentType> *>(
          m_componentLists[id].get());
      return *c;
    }

    template <typename ComponentType> void MakeComponentAllocator()
    {
      usize id = ComponentID<ComponentType>();

      Assert(
          id < m_size,
          std::format(
              "Tried making new component array for {} when it already exsits!",
              typeid(ComponentType).name()));

      Assert(id < MAX_COMPONENTS,
             std::format("Exceeded maximum number of components (which is {})",
                         MAX_COMPONENTS));

      if constexpr (IsLargeComponent<ComponentType>()) {
        m_componentLists[id] =
            MakeBox<DynamicComponentAllocator<ComponentType>>();
      }
      else {
        m_componentLists[id] = MakeBox<ComponentAllocator<ComponentType>>();
      }

      m_size++;
      Ensures(m_size == id);
    }

    template <typename ComponentType> bool DoesAllocatorExist()
    {
      usize id = ComponentID<ComponentType>();
      return id < m_size;
    }

    template <typename ComponentType> constexpr bool IsLargeComponent()
    {
      return std::is_base_of_v<Component::LargeComponent, ComponentType>;
    }

    std::array<Box<IComponentAllocator>, MAX_COMPONENTS> m_componentLists;
    usize m_size = 0;
    TypeIdMaker m_typeIdMaker;
};
