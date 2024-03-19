#include "Ecs/systemManager.hpp"
#include "Ecs/entityManager.hpp"
#include "util/contracts.hpp"
#include "util/types.hpp"

template <typename SystemType, typename... Args>
Ref<System> SystemManager::RegisterSystem(Args &&...args)
{
  Requires(GetSystemID<SystemType>() == m_systems.size(),
           "Cannot add the same system twice!");

  Ref<SystemType> system = MakeRef<SystemType>(std::forward<Args>(args)...);
  m_systems.push_back(system);
  return system;
}

template <typename SystemType> Ref<System> SystemManager::GetSystem()
{
  usize id = GetSystemID<SystemType>();
  Assert(0 <= id && id < m_systems.size());

  Ref<SystemType> systemPtr = m_systems[id];
  return systemPtr;
}

void SystemManager::EntityDestroyed(Entity entity) {
    for (auto& system : m_systems) {
        auto& entitySet = system->m_entities;

        if (entitySet.contains(entity)) {
            system->OnEntityDestroyed(entity);
            entitySet.erase(entity);
        }
    }
}

void SystemManager::EntitySignatureChanged(Entity entity, Signature signature) {
    for (auto& system : m_systems) {
        auto& entitySet = system->m_entities;

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