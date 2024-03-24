#pragma once
#include "util/macros.hpp"
#include "util/types.hpp"

class SceneManager;

class Scene
{
  public:
    Scene(SceneManager &sceneManager) : m_sceneManager(sceneManager) {}

    virtual void OnEnter() = 0;
    virtual void Update() = 0;
    virtual void OnExit() = 0;

    virtual ~Scene() = 0;

  protected:
    SceneManager &m_sceneManager;
};

class SceneManager
{
  public:
    SceneManager(Box<Scene> &scene);
    NO_COPY_OR_MOVE_CONSTRUCTORS(SceneManager)

    void Update();
    void ChangeScene(Box<Scene> &new_scene);

  private:
    Box<Scene> m_currentScene;
};