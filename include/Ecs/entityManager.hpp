#pragma once
#include "Ecs/signature.hpp"
#include "Layer.hpp"
#include "pch.hpp"
#include "util/macros.hpp"
#include "util/types.hpp"

class EntityComponentSystem;
typedef usize EntityID;

class Entity
{
  public:
    friend class EntityManager;

    inline EntityID GetID() { return m_id; }

    bool operator==(const Entity &other) const { return m_id == other.m_id; }
    struct Hasher
    {

        usize operator()(const Entity &handle) const
        {
          return std::hash<usize>{}(handle.m_id);
        }
    };

  private:
    explicit Entity(EntityID id) : m_id(id) {}
    EntityID m_id;
};

class EntityManager
{
  public:
    EntityManager() {}
    NO_COPY_OR_MOVE_CONSTRUCTORS(EntityManager)

    Entity MakeEntity();
    void DeleteEntity(Entity entity);

    Signature GetSignature(Entity entity) const;
    bool HasComponent(Entity entity, usize componentID) const;
    void AddComponent(Entity entity, usize componentID);
    void RemoveComponent(Entity entity, usize componentID);

    LayerMask GetLayerMask(Entity entity) const;
    void AddToLayer(Entity entity);
    std::vector<Entity> GetEntitiesByLayer(LayerMask mask) const;

    Entity GetEntityByName(std::string name) const;
    Entity GetEntityByTag(std::string tag) const;

    std::vector<Entity> GetEntitiesByName(std::string name) const;
    std::vector<Entity> GetEntitiesByTag(std::string tag) const;

    Option<Entity> GetParent(Entity id) const;
    std::vector<Entity>& GetChildren(Entity id) const;

  private:
    struct EntityData
    {
        Signature signature;
        std::string name;
        std::string tagName;
        std::vector<Entity> children;
        Option<Entity> parent;
    };

    std::queue<EntityID> m_idQueue;
    LayerRegistry m_layerRegistry;
    std::array<EntityData, MAX_ENTITIES> m_entityData;
    usize m_entityCount = 0;
};