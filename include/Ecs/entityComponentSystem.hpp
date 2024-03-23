#pragma once
#include "Ecs/component.hpp"
#include "Ecs/componentManager.hpp"
#include "Ecs/entityManager.hpp"
#include "Ecs/systemManager.hpp"
#include "event/eventManager.hpp"
#include "util/macros.hpp"
#include <type_traits>

namespace Event
{
  struct EntityDestroyed
  {
      Entity entity;
  };
} // namespace Event

class EntityComponentSystem
{
  public:
    EntityComponentSystem()
    {
      EventManager::Subscribe<Event::EntityDestroyed>(
          [&](const Event::EntityDestroyed &event) {
            DeleteEntity(event.entity);
          });
    }

    NO_COPY_OR_MOVE_CONSTRUCTORS(EntityComponentSystem)

    Entity MakeEntity() { return m_entityManager.MakeEntity(); }

    void DeleteEntity(Entity entity)
    {
      m_componentManager.EntityDestroyed(entity.m_id, m_entityManager);
      m_systemManager.EntityDestroyed(entity);
      m_entityManager.DeleteEntity(entity.m_id);
    }

    template <typename ComponentType, typename... Args>
      requires std::is_base_of_v<Component::Component, ComponentType>
    ComponentType &AddComponent(Entity entity, Args &&...args)
    {
      usize componentId =
          TypeIdMaker<Component::Component>::GetId<ComponentType>();
      Signature sig = m_entityManager.GetSignature(entity.GetID());

      m_entityManager.AddComponent(entity.m_id, componentId);
      m_componentManager.AddComponent<ComponentType>(
          entity.m_id, std::forward<Args>(args)...);
      m_systemManager.EntitySignatureChanged(entity, sig);

      return m_componentManager.GetComponent<ComponentType>(entity.m_id);
    }

    template <typename ComponentType>
      requires std::is_base_of_v<Component::Component, ComponentType>
    void RemoveComponent(Entity entity)
    {
      usize componentId =
          TypeIdMaker<Component::Component>::GetId<ComponentType>();
      Signature sig = m_entityManager.GetSignature(entity.GetID());

      m_systemManager.EntitySignatureChanged(entity, sig);
      m_entityManager.RemoveComponent(entity.m_id, componentId);
      m_componentManager.DeleteComponent<ComponentType>(entity.m_id);
    }

    template <typename ComponentType>
      requires std::is_base_of_v<Component::Component, ComponentType>
    bool HasComponent(Entity entity)
    {
      usize componentId =
          TypeIdMaker<Component::Component>::GetId<ComponentType>();
      return m_entityManager.HasComponent(entity.m_id, componentId);
    }

    template <typename ComponentType>
      requires std::is_base_of_v<Component::Component, ComponentType>
    ComponentType &GetComponent(Entity entity)
    {
      return m_componentManager.GetComponent<ComponentType>(entity.m_id);
    }

    template <typename ComponentType>
      requires std::is_base_of_v<Component::Component, ComponentType>
    const ComponentType &GetComponentConst(Entity entity)
    {
      return m_componentManager.GetComponent<ComponentType>(entity.m_id);
    }

  private:
    SystemManager m_systemManager;
    ComponentManager m_componentManager;
    EntityManager m_entityManager;
};