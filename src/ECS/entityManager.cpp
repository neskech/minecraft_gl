#include "Ecs/entityManager.hpp"
#include "Ecs/EcsConstants.hpp"
#include "util/contracts.hpp"

Entity EntityManager::MakeEntity()
{
  Requires(m_entityCount + 1 < MAX_ENTITIES, "Too many entities!");

  usize id;

  if (!m_idQueue.empty()) {
    id = m_idQueue.front();
    m_idQueue.pop();
  }
  else
    id = m_entityCount;

  m_signatures[id] = 0;
  m_entityCount++;

  return Entity(id);
}

void EntityManager::DeleteEntity(EntityID id)
{
  Requires(0 <= id && id < MAX_ENTITIES);

  m_idQueue.push(id);
  m_signatures[id] = 0;
}

bool EntityManager::HasComponent(EntityID id, usize componentID) const
{
  Requires(0 <= componentID && componentID < MAX_COMPONENTS);

  auto &bits = m_signatures[id];
  return bits.test(componentID);
}

void EntityManager::AddComponent(EntityID id, usize componentID)
{
  Requires(0 <= componentID && componentID < MAX_COMPONENTS);

  auto &bits = m_signatures[id];
  bits.set(componentID);
}

void EntityManager::RemoveComponent(EntityID id, usize componentID)
{
  Requires(0 <= componentID && componentID < MAX_COMPONENTS);

  auto &bits = m_signatures[id];
  bits.set(componentID, 0);
}