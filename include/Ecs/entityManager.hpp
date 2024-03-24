#pragma once
#include "Ecs/signature.hpp"
#include "EcsConstants.hpp"
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
    inline bool HasValidID() { return 0 <= m_id && m_id < MAX_ENTITIES; }

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

    Entity MakeEntity(std::string name = "");
    void DeleteEntity(Entity entity);

    Signature GetSignature(Entity entity) const;
    bool HasComponent(Entity entity, usize componentID) const;
    void AddComponent(Entity entity, usize componentID);
    void RemoveComponent(Entity entity, usize componentID);

    LayerMask GetLayerMask(Entity entity) const;
    void AddToLayer(Entity entity, LayerMask mask);
    void RemoveFromLayer(Entity entity, LayerMask mask);
    std::vector<Entity> GetEntitiesByLayer(LayerMask mask) const;

    Option<Entity> GetEntityByName(std::string_view name) const;
    Option<Entity> GetEntityByTag(std::string_view tag) const;

    std::vector<Entity> GetEntitiesByName(std::string_view name) const;
    std::vector<Entity> GetEntitiesByTag(std::string_view tag) const;

    Option<Entity> GetParent(Entity entity) const;
    std::vector<Entity> &GetChildren(Entity entity);

    inline bool IsEntityAlive(Entity entity) const
    {

      return m_entityData[entity.GetID()].isAlive;
    }

  private:
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

    std::queue<EntityID> m_idQueue;
    std::array<EntityData, MAX_ENTITIES> m_entityData;
    usize m_entityCount = 0;
};