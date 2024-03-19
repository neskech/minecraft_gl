#pragma once
#include "Ecs/entityManager.hpp"
#include "Ecs/typeId.hpp"
#include "pch.hpp"
#include "util/macros.hpp"
#include "util/types.hpp"
#include <vector>

class SystemManager;

class System
{
  public:
    friend class SystemManager;
    System(Signature sig) : m_signature(sig) {}

    virtual void OnEntityEnter(Entity entity);
    virtual void OnEntityDestroyed(Entity entity);
    virtual void OnEntityExit(Entity entity);

    virtual ~System() {}

  private:
    std::unordered_set<Entity, Entity::Hasher> m_entities;
    Signature m_signature;
};

class SystemManager
{
  public:
    SystemManager() {}
    NO_COPY_OR_MOVE_CONSTRUCTORS(SystemManager)

    template <typename SystemType, typename... Args>
    Ref<System> RegisterSystem(Args &&...args);

    template <typename SystemType>
    Ref<System> GetSystem();

    void EntityDestroyed(Entity entity);

    void EntitySignatureChanged(Entity entity, Signature signature);

  private:
    template <typename SystemType> usize GetSystemID()
    {
      return TypeIdMaker<System>::GetId<SystemType>();
    }

    std::vector<Ref<System>> m_systems;
};