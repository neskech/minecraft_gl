#pragma once
#include "Ecs/entityManager.hpp"
#include "Ecs/typeId.hpp"
#include "pch.hpp"
#include "util/contracts.hpp"
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
    Ref<System> RegisterSystem(Args &&...args)
    {
      Requires(GetSystemID<SystemType>() == m_systems.size(),
               "Cannot add the same system twice!");

      Ref<SystemType> system = MakeRef<SystemType>(std::forward<Args>(args)...);
      m_systems.push_back(system);
      return system;
    }

    template <typename SystemType>
    Ref<System> GetSystem()
    {
      usize id = GetSystemID<SystemType>();
      Assert(0 <= id && id < m_systems.size());

      Ref<SystemType> systemPtr = m_systems[id];
      return systemPtr;
    }

    void EntityDestroyed(Entity entity)
    {
      for (auto &system : m_systems) {
        auto &entitySet = system->m_entities;

        if (entitySet.contains(entity)) {
          system->OnEntityDestroyed(entity);
          entitySet.erase(entity);
        }
      }
    }

    void EntitySignatureChanged(Entity entity, Signature signature)
    {
      for (auto &system : m_systems) {
        auto &entitySet = system->m_entities;

        bool containsEntity = entitySet.contains(entity);
        bool sameSignature = system->m_signature == signature;
        if (containsEntity && !sameSignature) {
          system->OnEntityExit(entity);
          entitySet.erase(entity);
        }

        if (!containsEntity && sameSignature) {
          system->OnEntityEnter(entity);
          entitySet.insert(entity);
        }
      }
    }

  private:
    template <typename SystemType>
    usize GetSystemID()
    {
      return TypeIdMaker<System>::GetId<SystemType>();
    }

    std::vector<Ref<System>> m_systems;
};