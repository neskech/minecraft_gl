#include "Ecs/componentManager.hpp"
#include "Ecs/entityManager.hpp"

template <typename ComponentType, typename... Args>
  requires std::is_constructible_v<ComponentType, Args...>
void ComponentManager::AddComponent(EntityID id, Args &&...args)
{
  if constexpr (IsLargeComponent<ComponentType>()) {
    auto &allocator = GetDynamicComponentAllocator<ComponentType>();
    allocator.AllocateComponent(id, std::forward<Args>(args)...);
  }
  else {
    auto &allocator = GetComponentAllocator<ComponentType>();
    allocator.AllocateComponent(id, std::forward<Args>(args)...);
  }
}

template <typename ComponentType>
ComponentType &ComponentManager::GetComponent(EntityID id)
{
  if constexpr (IsLargeComponent<ComponentType>()) {
    auto &allocator = GetDynamicComponentAllocator<ComponentType>();
    return allocator.GetComponent(id);
  }
  else {
    auto &allocator = GetComponentAllocator<ComponentType>();
    return allocator.GetComponent(id);
  }
}

template <typename ComponentType>
void ComponentManager::DeleteComponent(EntityID id)
{
  if constexpr (IsLargeComponent<ComponentType>()) {
    auto &allocator = GetDynamicComponentAllocator<ComponentType>();
    allocator.FreeComponent(id);
  }
  else {
    auto &allocator = GetComponentAllocator<ComponentType>();
    allocator.FreeComponent(id);
  }
}

void ComponentManager::EntityDestroyed(EntityID id,
                                       const EntityManager &manager)
{
  for (u32 i = 0; i < m_size; i++) {
    if (manager.HasComponent(id, i))
      m_componentLists[i].get()->FreeComponent(id);
  }
}

template <typename ComponentType> usize ComponentManager::ComponentID()
{
  return m_typeIdMaker.GetId<ComponentType>();
}