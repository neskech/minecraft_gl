#pragma once
#include "Ecs/signature.hpp"
#include "EcsConstants.hpp"
#include "Layer.hpp"
#include "entity.hpp"
#include "pch.hpp"
#include "util/macros.hpp"
#include "util/types.hpp"

namespace ECS
{
  struct EntityData
  {
      EntityData()
          : signature(0), name(""), tagName(""), layerMask(0), children({}),
            parent(Optional::None<Entity>()), isAlive(false)
      {}

      EntityData &operator=(EntityData &&other)
      {
        signature = other.signature;
        name = std::move(other.name);
        tagName = std::move(other.tagName);
        children = std::move(other.children);
        layerMask = other.layerMask;
        parent = other.parent;
        isAlive = other.isAlive;
        return *this;
      }

      Signature signature;
      std::string name;
      std::string tagName;
      LayerMask layerMask;
      std::vector<Entity> children;
      Option<Entity> parent;
      /* False if the entity is deleted */
      bool isAlive;
  };

  class EntityManager
  {
    public:
      EntityManager() {}
      NO_COPY_OR_MOVE_CONSTRUCTORS(EntityManager)

      Entity MakeEntity(std::string name = "");
      void DeleteEntity(Entity entity);

      Signature GetSignature(Entity entity) const;
      bool HasComponent(Entity entity, usize componentID) const;
      void AddComponent(Entity entity, usize componentID);
      void RemoveComponent(Entity entity, usize componentID);

      std::vector<Entity> GetEntitiesByLayer(LayerMask mask) const;
      std::vector<Entity> GetEntitiesByName(std::string_view name) const;
      std::vector<Entity> GetEntitiesByTag(std::string_view tag) const;

      Option<Entity> GetEntityByName(std::string_view name) const;
      Option<Entity> GetEntityByTag(std::string_view tag) const;

      inline EntityData &GetEntityData(Entity entity)
      {
        return m_entityData[entity.GetID()];
      }

      inline bool IsEntityAlive(Entity entity) const
      {

        return m_entityData[entity.GetID()].isAlive;
      }

    private:
      std::queue<EntityID> m_idQueue;
      std::array<EntityData, MAX_ENTITIES> m_entityData;
      usize m_entityCount = 0;
  };
} // namespace ECS