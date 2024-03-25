#pragma once
#include "Ecs/component.hpp"
#include "Ecs/componentManager.hpp"
#include "Ecs/entityManager.hpp"
#include "Ecs/systemManager.hpp"
#include "Layer.hpp"
#include "entity.hpp"
#include "event/eventManager.hpp"
#include "util/contracts.hpp"
#include "util/macros.hpp"
#include <type_traits>

namespace Event
{
  struct EntityDestroyed
  {
      ECS::Entity entity;
  };
} // namespace Event

namespace ECS
{

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
        Requires(entity.HasValidID());
        Requires(m_entityManager.IsEntityAlive(entity));

        m_componentManager.EntityDestroyed(entity, m_entityManager);
        m_systemManager.EntityDestroyed(entity);
        m_entityManager.DeleteEntity(entity);
      }

      template <typename ComponentType, typename... Args>
        requires std::is_base_of_v<Component::Component, ComponentType>
      ComponentType &AddComponent(Entity entity, Args &&...args)
      {
        Requires(entity.HasValidID());
        Requires(m_entityManager.IsEntityAlive(entity));

        usize componentId =
            TypeIdMaker<Component::Component>::GetId<ComponentType>();
        Assert(ComponentManager::IsValidComponentID(componentId),
               "Too many components!");

        Signature sig = m_entityManager.GetEntityData(entity).signature;

        m_entityManager.AddComponent(entity, componentId);
        m_componentManager.AddComponent<ComponentType>(
            entity, std::forward<Args>(args)...);
        m_systemManager.EntitySignatureChanged(entity, sig);

        return m_componentManager.GetComponent<ComponentType>(entity);
      }

      template <typename ComponentType>
        requires std::is_base_of_v<Component::Component, ComponentType>
      void RemoveComponent(Entity entity)
      {
        Requires(entity.HasValidID());
        Requires(m_entityManager.IsEntityAlive(entity));

        usize componentId =
            TypeIdMaker<Component::Component>::GetId<ComponentType>();
        Assert(ComponentManager::IsValidComponentID(componentId),
               "Too many components!");

        Signature sig = m_entityManager.GetEntityData(entity).signature;

        m_systemManager.EntitySignatureChanged(entity, sig);
        m_entityManager.RemoveComponent(entity, componentId);
        m_componentManager.DeleteComponent<ComponentType>(entity);
      }

      template <typename ComponentType>
        requires std::is_base_of_v<Component::Component, ComponentType>
      inline bool HasComponent(Entity entity)
      {
        Requires(entity.HasValidID());
        Requires(m_entityManager.IsEntityAlive(entity));

        usize componentId =
            TypeIdMaker<Component::Component>::GetId<ComponentType>();
        Assert(ComponentManager::IsValidComponentID(componentId),
               "Too many components!");

        return m_entityManager.HasComponent(entity, componentId);
      }

      template <typename ComponentType>
        requires std::is_base_of_v<Component::Component, ComponentType>
      inline ComponentType &GetComponent(Entity entity)
      {
        Requires(entity.HasValidID());
        Requires(m_entityManager.IsEntityAlive(entity));

        return m_componentManager.GetComponent<ComponentType>(entity);
      }

      template <typename ComponentType>
        requires std::is_base_of_v<Component::Component, ComponentType>
      inline const ComponentType &GetComponentConst(Entity entity)
      {
        Requires(entity.HasValidID());
        Requires(m_entityManager.IsEntityAlive(entity));

        return m_componentManager.GetComponent<ComponentType>(entity.GetID());
      }

      inline std::vector<Entity> GetEntitiesByLayer(LayerMask mask) const
      {
        return m_entityManager.GetEntitiesByLayer(mask);
      }

      inline Option<Entity> GetEntityByName(std::string_view name) const
      {
        return m_entityManager.GetEntityByName(name);
      }

      inline Option<Entity> GetEntityByTag(std::string_view tag) const
      {
        return m_entityManager.GetEntityByTag(tag);
      }

      inline std::vector<Entity> GetEntitiesByName(std::string_view name) const
      {
        return m_entityManager.GetEntitiesByName(name);
      }

      inline std::vector<Entity> GetEntitiesByTag(std::string_view tag) const
      {
        return m_entityManager.GetEntitiesByTag(tag);
      }

      inline EntityData &GetEntityData(Entity entity)
      {
        Requires(entity.HasValidID());
        Requires(m_entityManager.IsEntityAlive(entity));

        return m_entityManager.GetEntityData(entity);
      }

      inline LayerRegistry &GetLayerRegistry() { return m_layerRegistry; }

    private:
      SystemManager m_systemManager;
      ComponentManager m_componentManager;
      EntityManager m_entityManager;
      LayerRegistry m_layerRegistry;
  };

} // namespace ECS