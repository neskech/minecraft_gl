#pragma once
#include "Layer.hpp"
#include "entity.hpp"
#include "entityComponentSystem.hpp"
#include "entityManager.hpp"

namespace ECS
{

  class EntityWrapper
  {
    public:
      EntityWrapper(Entity entity, EntityComponentSystem &ecs)
          : m_entity(entity), m_Ecs(ecs)
      {}

      inline void Destroy() { m_Ecs.DeleteEntity(m_entity); }

      template <typename ComponentType, typename... Args>
        requires std::is_base_of_v<Component::Component, ComponentType>
      inline ComponentType &AddComponent(Args &&...args)
      {
        return m_Ecs.AddComponent<ComponentType>(m_entity,
                                                 std::forward<Args>(args)...);
      }

      template <typename ComponentType>
        requires std::is_base_of_v<Component::Component, ComponentType>
      void RemoveComponent()
      {
        return m_Ecs.RemoveComponent<ComponentType>(m_entity);
      }

      template <typename ComponentType>
        requires std::is_base_of_v<Component::Component, ComponentType>
      inline bool HasComponent()
      {
        return m_Ecs.HasComponent<ComponentType>(m_entity);
      }

      template <typename ComponentType>
        requires std::is_base_of_v<Component::Component, ComponentType>
      inline ComponentType &GetComponent()
      {
        return m_Ecs.GetComponent<ComponentType>(m_entity);
      }

      template <typename ComponentType>
        requires std::is_base_of_v<Component::Component, ComponentType>
      inline const ComponentType &GetComponentConst()
      {
        return m_Ecs.GetComponentConst<ComponentType>(m_entity);
      }

      inline Option<Entity> &GetParent() const
      {
        return m_Ecs.GetEntityData(m_entity).parent;
      }

      inline std::vector<Entity> &GetChildren()
      {
        return m_Ecs.GetEntityData(m_entity).children;
      }

      inline std::string &GetName()
      {
        return m_Ecs.GetEntityData(m_entity).name;
      }

      inline std::string &GetTag()
      {
        return m_Ecs.GetEntityData(m_entity).tagName;
      }

      inline LayerMask GetLayerMask()
      {
        return m_Ecs.GetEntityData(m_entity).layerMask;
      }

      inline void AddLayerToLayerMask(LayerMask mask)
      {
        LayerMask &l = m_Ecs.GetEntityData(m_entity).layerMask;
        l |= mask;
      }

      inline void RemoveLayerToLayerMask(LayerMask mask)
      {
        LayerMask &l = m_Ecs.GetEntityData(m_entity).layerMask;
        l &= ~mask;
      }

      LayerRegistry &GetLayerRegistry() { return m_Ecs.GetLayerRegistry(); }

      inline EntityComponentSystem &GetECS() { return m_Ecs; }

    private:
      Entity m_entity;
      EntityComponentSystem &m_Ecs;
  };

  using EntityW = EntityWrapper;

} // namespace ECS