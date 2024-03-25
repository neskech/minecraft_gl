#pragma once
#include "GLFW/glfw3.h"
#include "util/input.hpp"
#include "util/types.hpp"

class Window
{
  public:
    Window(u32 width, u32 height);
    ~Window();

    Result<Unit, std::string> TryInitialize();

    void SetupCallbacks();

    void PollEvents();

    void FinishFrame();

    bool AboutToClose();

    static inline u32 GetWidth() { return s_instance->m_width; }

    static inline u32 GetHeight() { return s_instance->m_height; }

    static inline u32 GetFramebufferWidth()
    {
      i32 width;
      glfwGetFramebufferSize(s_instance->m_window, &width, nullptr);
      return width;
    }

    static inline u32 GetFramebufferHeight()
    {
      i32 height;
      glfwGetFramebufferSize(s_instance->m_window, nullptr, &height);
      return height;
    }

  private:
    static void ResizeCallback(GLFWwindow *window, i32 width, i32 height);

    inline static Window *s_instance;
    GLFWwindow *m_window;
    u32 m_width, m_height;
};