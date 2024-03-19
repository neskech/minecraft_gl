#pragma once
#include "Ecs/EcsConstants.hpp"
#include "pch.hpp"
#include "util/macros.hpp"

class EntityComponentSystem;
typedef usize EntityID;

class Entity
{
  public:
    friend class EntityManager;
    friend class EntityComponentSystem;

    inline EntityID GetID() { return m_id; }

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
    void DeleteEntity(EntityID entity);

    bool HasComponent(EntityID entity, usize componentID) const;
    void AddComponent(EntityID entity, usize componentID);
    void RemoveComponent(EntityID entity, usize componentID);

  private:
    std::queue<EntityID> m_idQueue;
    std::array<Signature, MAX_ENTITIES> m_signatures;
    usize m_entityCount = 0;
};