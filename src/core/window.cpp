#include "core/window.hpp"
#include "GLFW/glfw3.h"
#include "pch.hpp"
#include "util/contracts.hpp"
#include "util/types.hpp"

Window::Window(u32 width, u32 height) : m_width(width), m_height(height)
{
  Assert(s_instance == nullptr, "Window can only be initialized once");
  s_instance = this;
}

Window::~Window()
{
  glfwDestroyWindow(m_window);
  glfwTerminate();
}

bool Window::AboutToClose() { return glfwWindowShouldClose(m_window); }

Result<Unit, std::string> Window::TryInitialize()
{
  if (!glfwInit()) {
    glfwTerminate();
    return ErrWithCopy<Unit>("ERROR: GLFW failed to initialize\n");
  }

  SetupCallbacks();

  glfwWindowHint(GLFW_CONTEXT_VERSION_MAJOR, 3);
  glfwWindowHint(GLFW_CONTEXT_VERSION_MINOR, 2);
  //glfwWindowHint(GLFW_OPENGL_FORWARD_COMPAT, GL_TRUE);
  glfwWindowHint(GLFW_OPENGL_PROFILE, GLFW_OPENGL_CORE_PROFILE);

  m_window = glfwCreateWindow(m_width, m_height, "Terraria", NULL, NULL);
  std::println("wq {}", (usize)m_window);
  if (m_window == nullptr) {
    glfwTerminate();
    return ErrWithCopy<Unit>("ERROR: Window failed to initialize\n");
  }

  glfwMakeContextCurrent(m_window);

  bool isGladInit = gladLoadGLLoader((GLADloadproc)glfwGetProcAddress);
  ;
  if (!isGladInit) {
    return ErrWithCopy<Unit>("failed to initialze OPENGL\n");
  }

  glfwSwapInterval(1);

  return Ok<Unit, std::string>(Unit{});
}

static void ErrorCallback(int error, const char *description)
{
  std::println("Glfw Error: {}", description);
}

void Window::ResizeCallback(GLFWwindow *window, i32 width, i32 height)
{
  glViewport(0, 0, width, height);

  i32 left, right, top, bottom;
  glfwGetWindowFrameSize(window, &left, &top, &right, &bottom);

  Assert(right - left > 0 && top - bottom > 0);

  s_instance->m_width = right - left;
  s_instance->m_height = top - bottom;
}

void Window::SetupCallbacks()
{
  glfwSetErrorCallback(ErrorCallback);
  glfwSetFramebufferSizeCallback(m_window, ResizeCallback);
  glfwSetKeyCallback(m_window, Input::OnWindowKeyEvent);
  glfwSetCursorPosCallback(m_window, Input::OnWindowMouseMoveEvent);
  glfwSetMouseButtonCallback(m_window, Input::OnWindowMousePressedEvent);
  glfwSetScrollCallback(m_window, Input::OnWindowMouseScrolledEvent);
}

void Window::PollEvents() { glfwPollEvents(); }

void Window::FinishFrame() { glfwSwapBuffers(m_window); }