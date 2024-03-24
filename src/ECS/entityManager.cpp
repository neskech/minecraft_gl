#include "Ecs/entityManager.hpp"
#include "Ecs/EcsConstants.hpp"
#include "signature.hpp"
#include "util/contracts.hpp"
#include "util/types.hpp"

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

  EntityData data{.signature = 0,
                  .name = "",
                  .tagName = "",
                  .children = {},
                  .parent = Optional::None<Entity>()};
  m_entityData[id] = data;
  m_entityCount++;

  return Entity(id);
}

void EntityManager::DeleteEntity(Entity entity)
{
  Requires(0 <= entity.GetID() && entity.GetID() < MAX_ENTITIES);

  m_idQueue.push(entity.GetID());

  EntityData &data = m_entityData[entity.GetID()];
  // TODO: Might still keep some capacity and waste memory
  data.children.clear();

  m_entityCount--;
}

bool EntityManager::HasComponent(Entity entity, usize componentID) const
{
  Requires(0 <= componentID && componentID < MAX_COMPONENTS);

  const Signature &bits = m_entityData[entity.GetID()].signature;
  return bits.test(componentID);
}

void EntityManager::AddComponent(Entity entity, usize componentID)
{
  Requires(0 <= componentID && componentID < MAX_COMPONENTS);

  Signature &bits = m_entityData[entity.GetID()].signature;
  bits.set(componentID);
}

void EntityManager::RemoveComponent(Entity entity, usize componentID)
{
  Requires(0 <= componentID && componentID < MAX_COMPONENTS);

  Signature &bits = m_entityData[entity.GetID()].signature;
  bits.set(componentID, 0);
}