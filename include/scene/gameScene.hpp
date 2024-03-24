#pragma once
#include "entityComponentSystem.hpp"
#include "entityWrapper.hpp"

class IGameScene
{
  public:
    IGameScene();
    NO_COPY_OR_MOVE_CONSTRUCTORS(IGameScene)

    ECS::EntityWrapper MakeEntity();

    virtual ~IGameScene() {} 

  protected:
    ECS::EntityComponentSystem m_Ecs;
};