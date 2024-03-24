#include "core/window.hpp"
#include "GLFW/glfw3.h"
#include "pch.hpp"
#include "util/types.hpp"
#include <GL/gl.h>

Window::Window(u32 width, u32 height) : m_width(width), m_height(height) {}

Window::~Window()
{
  glfwDestroyWindow(m_window);
  glfwTerminate();
}

Result<Unit, std::string> Window::TryInitialize()
{
  if (!glfwInit()) {
    glfwTerminate();
    return ErrWithCopy<Unit>("ERROR: GLFW failed to initialize\n");
  }

  glfwWindowHint(GLFW_CONTEXT_VERSION_MAJOR, 4);
  glfwWindowHint(GLFW_CONTEXT_VERSION_MINOR, 1);
  glfwWindowHint(GLFW_OPENGL_FORWARD_COMPAT, GL_TRUE);
  glfwWindowHint(GLFW_OPENGL_PROFILE, GLFW_OPENGL_CORE_PROFILE);

  m_window = glfwCreateWindow(m_width, m_height, "Terraria", NULL, NULL);
  if (m_window == nullptr) {
    glfwTerminate();
    return ErrWithCopy<Unit>("ERROR: Window failed to initialize\n");
  }

  SetupCallbacks();
  glfwMakeContextCurrent(m_window);

  bool isGladInit = gladLoadGLLoader((GLADloadproc)glfwGetProcAddress);

  if (!isGladInit) {
    return ErrWithCopy<Unit>("failed to initialze OPENGL\n");
  }

  glfwSwapInterval(1);

  return Ok<Unit, std::string>(Unit{});
}

void Window::SetupCallbacks()
{
  glfwSetErrorCallback(std::bind(this->ErrorCallback, this, std::placeholders::_1));
  glfwSetFramebufferSizeCallback(m_window, (GLFWframebuffersizefun) ResizeCallback);

  //   glfwSetKeyCallback(glfw_window, KeyListener::keyCallBack);
  //   glfwSetCursorPosCallback(glfw_window,
  //                            MouseListener::cursor_position_callback);
  //   glfwSetMouseButtonCallback(glfw_window,
  //   MouseListener::mouse_button_callback); glfwSetScrollCallback(glfw_window,
  //   MouseListener::scroll_callback);
}

void Window::ErrorCallback(int error, const char *description) {
       std::println("Glfw Error: {}", description);
}

void Window::ResizeCallback(GLFWwindow *window, i32 width, i32 height)
{
  m_width = width;
  m_height = height;
  glViewport(0, 0, width, height);
}

void Window::PollEvents() { glfwPollEvents(); }

void Window::FinishFrame() { glfwSwapBuffers(m_window); }