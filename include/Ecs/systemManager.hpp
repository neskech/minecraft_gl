#pragma once
#include "Ecs/EcsConstants.hpp"
#include "Ecs/entityManager.hpp"
#include "pch.hpp"

class SystemManager;

class System
{
  public:
    friend class SystemManager;
    System(Signature sig) : m_signature(sig) {}

  private:
    std::unordered_set<Entity> m_entities;
    Signature m_signature;
};

class SystemManager
{
};