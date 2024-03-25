#include "scene/scene.hpp"
#include "util/types.hpp"

SceneManager::SceneManager() : m_currentScene(nullptr)
{
  //m_currentScene->OnEnter();
}

void SceneManager::Update() { m_currentScene->Update(); }

void SceneManager::ChangeScene(Box<Scene> &scene)
{
  m_currentScene->OnExit();
  m_currentScene = std::move(scene);
  m_currentScene->OnEnter();
}