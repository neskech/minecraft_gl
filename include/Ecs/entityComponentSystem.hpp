#pragma once
#include "Ecs/component.hpp"
#include "Ecs/componentManager.hpp"
#include "Ecs/entityManager.hpp"
#include "Ecs/systemManager.hpp"
#include "util/macros.hpp"
#include <type_traits>
class EntityComponentSystem
{
  public:
    EntityComponentSystem();
    NO_COPY_OR_MOVE_CONSTRUCTORS(EntityComponentSystem)

    Entity MakeEntity();

    void DeleteEntity(Entity entity);

    template <typename ComponentType, typename... Args>
      requires std::is_base_of_v<Component::Component, ComponentType>
    ComponentType &AddComponent(Entity entity, Args &&...args);

    template <typename ComponentType>
      requires std::is_base_of_v<Component::Component, ComponentType>
    void RemoveComponent(Entity entity);

    template <typename ComponentType>
      requires std::is_base_of_v<Component::Component, ComponentType>
    bool HasComponent(Entity entity);

    template <typename ComponentType>
      requires std::is_base_of_v<Component::Component, ComponentType>
    ComponentType &GetComponent(Entity entity);

    template <typename ComponentType>
      requires std::is_base_of_v<Component::Component, ComponentType>
    const ComponentType &GetComponentConst(Entity entity);

  private:
    SystemManager m_systemManager;
    ComponentManager m_componentManager;
    EntityManager m_entityManager;
};

namespace Event
{
  struct EntityDestroyed
  {
      Entity entity;
  };
} // namespace Event
