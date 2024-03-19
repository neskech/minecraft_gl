#include "Ecs/entityComponentSystem.hpp"
#include "component.hpp"
#include "eventManager.hpp"
#include "typeId.hpp"

EntityComponentSystem::EntityComponentSystem()
{
  EventManager::Subscribe<Event::EntityDestroyed>(
      [&](const Event::EntityDestroyed &event) { DeleteEntity(event.entity); });
}

Entity EntityComponentSystem::MakeEntity()
{
  return m_entityManager.MakeEntity();
}

void EntityComponentSystem::DeleteEntity(Entity entity)
{
  m_componentManager.EntityDestroyed(entity.m_id, m_entityManager);
  m_systemManager.EntityDestroyed(entity);
  m_entityManager.DeleteEntity(entity.m_id);
}

template <typename ComponentType, typename... Args>
  requires std::is_base_of_v<Component::Component, ComponentType>
ComponentType &EntityComponentSystem::AddComponent(Entity entity,
                                                   Args &&...args)
{
  usize componentId = TypeIdMaker<Component::Component>::GetId<ComponentType>();
  Signature sig = m_entityManager.GetSignature(entity.GetID());

  m_entityManager.AddComponent(entity.m_id, componentId);
  m_componentManager.AddComponent<ComponentType>(entity.m_id,
                                                 std::forward<Args>(args)...);
  m_systemManager.EntitySignatureChanged(entity, sig);

  return m_componentManager.GetComponent<ComponentType>(entity.m_id);
}

template <typename ComponentType>
  requires std::is_base_of_v<Component::Component, ComponentType>
void EntityComponentSystem::RemoveComponent(Entity entity)
{
  usize componentId = TypeIdMaker<Component::Component>::GetId<ComponentType>();
  Signature sig = m_entityManager.GetSignature(entity.GetID());

  m_systemManager.EntitySignatureChanged(entity, sig);
  m_entityManager.RemoveComponent(entity.m_id, componentId);
  m_componentManager.DeleteComponent<ComponentType>(entity.m_id);
}

template <typename ComponentType>
  requires std::is_base_of_v<Component::Component, ComponentType>
bool EntityComponentSystem::HasComponent(Entity entity)
{
  usize componentId = TypeIdMaker<Component::Component>::GetId<ComponentType>();
  return m_entityManager.HasComponent(entity.m_id, componentId);
}

template <typename ComponentType>
  requires std::is_base_of_v<Component::Component, ComponentType>
ComponentType &EntityComponentSystem::GetComponent(Entity entity)
{
  return m_componentManager.GetComponent<ComponentType>(entity.m_id);
}

template <typename ComponentType>
  requires std::is_base_of_v<Component::Component, ComponentType>
const ComponentType &EntityComponentSystem::GetComponentConst(Entity entity)
{
  return m_componentManager.GetComponent<ComponentType>(entity.m_id);
}