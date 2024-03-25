#pragma once
#include "eventManager.hpp"
#include "scene/scene.hpp"
#include "util/macros.hpp"
#include "window.hpp"
class Application
{
  public:
    explicit Application();
    NO_COPY_OR_MOVE_CONSTRUCTORS(Application)

    void Initialize();
    void Run();

  private:
    Input m_input;
    Window m_window;
    EventManager m_eventManager;
    SceneManager m_sceneManager;
};