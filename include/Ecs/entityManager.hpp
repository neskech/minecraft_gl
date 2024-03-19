#pragma once
#include "Ecs/signature.hpp"
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
    void DeleteEntity(EntityID entity);

    Signature GetSignature(EntityID entity) const;
    bool HasComponent(EntityID entity, usize componentID) const;
    void AddComponent(EntityID entity, usize componentID);
    void RemoveComponent(EntityID entity, usize componentID);

  private:
    std::queue<EntityID> m_idQueue;
    std::array<Signature, MAX_ENTITIES> m_signatures;
    usize m_entityCount = 0;
};