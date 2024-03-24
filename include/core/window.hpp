#pragma once
#include "GLFW/glfw3.h"
#include "util/types.hpp"

class Window
{
  public:
    Window(u32 width, u32 height);
    ~Window();

    Result<Unit, std::string> TryInitialize();
    void PollEvents();
    void FinishFrame();

    inline u32 GetWidth() { return m_width * 2; }
    inline u32 GetHeight() { return m_height * 2; }

  private:
    void SetupCallbacks();
    void ErrorCallback(int error, const char *description);
    void ResizeCallback(GLFWwindow *window, i32 width, i32 height);

    GLFWwindow *m_window;
    u32 m_width, m_height;
};