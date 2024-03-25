#include "core/application.hpp"
#include "GLFW/glfw3.h"
#include "util/constants.hpp"
#include "util/contracts.hpp"
#include "util/time.hpp"

Application::Application() : m_window(WINDOW_WIDTH, WINDOW_HEIGHT) {

}

void Application::Initialize()
{
  auto result = m_window.TryInitialize();
  if (!result.has_value()) {
    Assert(false, result.error());
  }
}

void Application::Run()
{
  f64 now;
  f64 before = glfwGetTime();

  while (!m_window.AboutToClose()) {
    now = glfwGetTime();
    f64 delta = now - before;
    Time::Instance().m_deltaTime = Time::GetTimeScale() * delta;
    before = now;

   // m_sceneManager.Update();

    m_window.FinishFrame();
    m_window.PollEvents();
  }
}
