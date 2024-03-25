#pragma once
#include "Ecs/EcsConstants.hpp"
#include "Ecs/component.hpp"
#include "Ecs/componentAllocator.hpp"
#include "entityManager.hpp"
#include "pch.hpp"
#include "util/contracts.hpp"
#include "util/macros.hpp"
#include "util/types.hpp"
#include <type_traits>

namespace ECS
{
  class ComponentManager
  {
    public:
      ComponentManager() {}
      NO_COPY_OR_MOVE_CONSTRUCTORS(ComponentManager)

      template <typename ComponentType, typename... Args>
        requires std::is_constructible_v<ComponentType, Args...>
      void AddComponent(Entity entity, Args &&...args)
      {
        if constexpr (std::is_base_of_v<Component::LargeComponent,
                                        ComponentType>) {
          auto &allocator = GetDynamicComponentAllocator<ComponentType>();
          allocator.AllocateComponent(entity, std::forward<Args>(args)...);
        }
        else {
          auto &allocator = GetComponentAllocator<ComponentType>();
          allocator.AllocateComponent(entity, std::forward<Args>(args)...);
        }
      }

      template <typename ComponentType>
      ComponentType &GetComponent(Entity entity)
      {
        if constexpr (std::is_base_of_v<Component::LargeComponent,
                                        ComponentType>) {
          auto &allocator = GetDynamicComponentAllocator<ComponentType>();
          return allocator.GetComponent(entity);
        }
        else {
          auto &allocator = GetComponentAllocator<ComponentType>();
          return allocator.GetComponent(entity);
        }
      }

      template <typename ComponentType>
      void DeleteComponent(Entity entity)
      {
        if constexpr (IsLargeComponent<ComponentType>()) {
          auto &allocator = GetDynamicComponentAllocator<ComponentType>();
          allocator.FreeComponent(entity);
        }
        else {
          auto &allocator = GetComponentAllocator<ComponentType>();
          allocator.FreeComponent(entity);
        }
      }

      void EntityDestroyed(Entity entity, const EntityManager &manager)
      {
        for (u32 i = 0; i < m_size; i++) {
          if (manager.HasComponent(entity, i))
            m_componentLists[i].get()->FreeComponent(entity);
        }
      }

      static inline bool IsValidComponentID(usize componentID)
      {
        return 0 <= componentID && componentID < MAX_COMPONENTS;
      }

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

      template <typename ComponentType>
      void MakeComponentAllocator()
      {
        usize id = ComponentID<ComponentType>();

        Assert(id < m_size, std::format("Tried making new component array for "
                                        "{} when it already exsits!",
                                        typeid(ComponentType).name()));

        Assert(
            id < MAX_COMPONENTS,
            std::format("Exceeded maximum number of components (which is {})",
                        MAX_COMPONENTS));

        if constexpr (std::is_base_of_v<Component::LargeComponent,
                                        ComponentType>) {
          m_componentLists[id] =
              Box<IComponentAllocator>(new DynamicComponentAllocator<ComponentType>());
        }
        else {
          m_componentLists[id] = Box<IComponentAllocator>(new ComponentAllocator<ComponentType>());
        }

        m_size++;
        Ensures(m_size == id);
      }

      template <typename ComponentType>
      bool DoesAllocatorExist()
      {
        usize id = ComponentID<ComponentType>();
        return id < m_size;
      }

      template <typename ComponentType>
      usize ComponentID()
      {
        return TypeIdMaker<Component::Component>::GetId<ComponentType>();
      }

      std::array<Box<IComponentAllocator>, MAX_COMPONENTS> m_componentLists;
      usize m_size = 0;
  };

} // namespace ECS