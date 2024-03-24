#include "Ecs/entityManager.hpp"
#include "Ecs/EcsConstants.hpp"
#include "Layer.hpp"
#include "signature.hpp"
#include "util/contracts.hpp"
#include "util/types.hpp"

namespace ECS
{

  Entity EntityManager::MakeEntity(std::string name)
  {
    Requires(m_entityCount + 1 < MAX_ENTITIES, "Too many entities!");

    usize id;

    if (!m_idQueue.empty()) {
      id = m_idQueue.front();
      m_idQueue.pop();
    }
    else
      id = m_entityCount;

    EntityData data;
    data.name = std::move(name);
    m_entityData[id] = std::move(data);

    m_entityCount++;

    return Entity(id);
  }

  void EntityManager::DeleteEntity(Entity entity)
  {
    m_idQueue.push(entity.GetID());

    /* Reset the data to empty values */
    m_entityData[entity.GetID()] = EntityData();

    m_entityCount--;
  }

  bool EntityManager::HasComponent(Entity entity, usize componentID) const
  {
    const Signature &bits = m_entityData[entity.GetID()].signature;
    return bits.test(componentID);
  }

  void EntityManager::AddComponent(Entity entity, usize componentID)
  {
    Signature &bits = m_entityData[entity.GetID()].signature;
    bits.set(componentID);
  }

  void EntityManager::RemoveComponent(Entity entity, usize componentID)
  {
    Signature &bits = m_entityData[entity.GetID()].signature;
    bits.set(componentID, 0);
  }

  /* TODO: Increase the speed of this functions by making a list of indices of
   * alive entities, and the entity -> index in that list hashmap */

  std::vector<Entity> EntityManager::GetEntitiesByLayer(LayerMask mask) const
  {
    std::vector<Entity> entities;

    for (u32 i = 0; i < m_entityCount; i++) {
      const EntityData &data = m_entityData[i];
      if (data.isAlive && data.layerMask == mask)
        entities.push_back(Entity(i));
    }

    return entities;
  }

  Option<Entity> EntityManager::GetEntityByName(std::string_view name) const
  {
    for (u32 i = 0; i < m_entityCount; i++) {
      const EntityData &data = m_entityData[i];
      if (data.isAlive && data.name == name)
        return Optional::Some<Entity>(Entity(i));
    }

    return Optional::None<Entity>();
  }

  Option<Entity> EntityManager::GetEntityByTag(std::string_view tag) const
  {
    for (u32 i = 0; i < m_entityCount; i++) {
      const EntityData &data = m_entityData[i];
      if (data.isAlive && data.tagName == tag)
        return Optional::Some<Entity>(Entity(i));
    }

    return Optional::None<Entity>();
  }

  std::vector<Entity>
  EntityManager::GetEntitiesByName(std::string_view name) const
  {
    std::vector<Entity> entities;

    for (u32 i = 0; i < m_entityCount; i++) {
      const EntityData &data = m_entityData[i];
      if (data.isAlive && data.name == name)
        entities.push_back(Entity(i));
    }

    return entities;
  }

  std::vector<Entity>
  EntityManager::GetEntitiesByTag(std::string_view tag) const
  {
    std::vector<Entity> entities;

    for (u32 i = 0; i < m_entityCount; i++) {
      const EntityData &data = m_entityData[i];
      if (data.isAlive && data.tagName == tag)
        entities.push_back(Entity(i));
    }

    return entities;
  }

} // namespace ECS