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
      m_componentManager.EntityDestroyed(entity, m_entityManager);
      m_systemManager.EntityDestroyed(entity);
      m_entityManager.DeleteEntity(entity);
    }

    template <typename ComponentType, typename... Args>
      requires std::is_base_of_v<Component::Component, ComponentType>
    ComponentType &AddComponent(Entity entity, Args &&...args)
    {
      usize componentId =
          TypeIdMaker<Component::Component>::GetId<ComponentType>();
      Signature sig = m_entityManager.GetSignature(entity);

      m_entityManager.AddComponent(entity, componentId);
      m_componentManager.AddComponent<ComponentType>(
          entity.GetID(), std::forward<Args>(args)...);
      m_systemManager.EntitySignatureChanged(entity, sig);

      return m_componentManager.GetComponent<ComponentType>(entity.GetID());
    }

    template <typename ComponentType>
      requires std::is_base_of_v<Component::Component, ComponentType>
    void RemoveComponent(Entity entity)
    {
      usize componentId =
          TypeIdMaker<Component::Component>::GetId<ComponentType>();
      Signature sig = m_entityManager.GetSignature(entity);

      m_systemManager.EntitySignatureChanged(entity, sig);
      m_entityManager.RemoveComponent(entity, componentId);
      m_componentManager.DeleteComponent<ComponentType>(entity);
    }

    template <typename ComponentType>
      requires std::is_base_of_v<Component::Component, ComponentType>
    inline bool HasComponent(Entity entity)
    {
      usize componentId =
          TypeIdMaker<Component::Component>::GetId<ComponentType>();
      return m_entityManager.HasComponent(entity, componentId);
    }

    template <typename ComponentType>
      requires std::is_base_of_v<Component::Component, ComponentType>
    inline ComponentType &GetComponent(Entity entity)
    {
      return m_componentManager.GetComponent<ComponentType>(entity.GetID());
    }

    template <typename ComponentType>
      requires std::is_base_of_v<Component::Component, ComponentType>
    inline const ComponentType &GetComponentConst(Entity entity)
    {
      return m_componentManager.GetComponent<ComponentType>(entity.GetID());
    }

  private:
    SystemManager m_systemManager;
    ComponentManager m_componentManager;
    EntityManager m_entityManager;
};