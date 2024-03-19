#pragma once

#include "Ecs/EcsConstants.hpp"
#include "Ecs/componentManager.hpp"
class SignatureBuilder
{
  public:
    SignatureBuilder(const ComponentManager& manager): m_manager(manager) {}

    template <typename ComponentType> SignatureBuilder &AddComponentType();

    template <typename... ComponentTypes> SignatureBuilder &AddComponentTypes();

    template <typename... ComponentTypes> Signature Build();

    Signature Finish();

  private:
    Signature m_signature = 0;
    const ComponentManager& m_manager;
};