#include "util/input.hpp"
#include "GLFW/glfw3.h"
#include "eventManager.hpp"
#include "util/contracts.hpp"
#include "util/inputMap.hpp"
#include <bitset>

constexpr usize IS_PRESSED_BIT_OFFSET = 7;
constexpr i32 MODIFIERS_BIT_MASK = 0b00011111;

void Input::OnWindowKeyEvent(GLFWwindow *, i32 key, i32 scancode, i32 action,
                             i32 mods)
{
  Requires(0 <= key && key < NUM_KEYS, "unknown keyboard key");

  switch (action) {
  case GLFW_REPEAT:
  case GLFW_PRESS: {
    auto &bits = m_keyPresses[key];
    bits.set(IS_PRESSED_BIT_OFFSET, 1);
    bits |= MODIFIERS_BIT_MASK & mods;

    Event::KeyPressed ev;
    ev.key = static_cast<KeyInput>(key);
    ev.modifiers = static_cast<KeyModifiers>(mods);
    EventManager::Invoke<Event::KeyPressed>(ev);

    break;
  }
  case GLFW_RELEASE: {
    auto &bits = m_keyPresses[key];
    bits.set(IS_PRESSED_BIT_OFFSET, 0);
    bits |= MODIFIERS_BIT_MASK & mods;

    Event::KeyUp ev;
    ev.key = static_cast<KeyInput>(key);
    ev.modifiers = static_cast<KeyModifiers>(mods);
    EventManager::Invoke<Event::KeyUp>(ev);

    break;
  }
  }
}

void Input::OnWindowMousePressedEvent(GLFWwindow *, i32 button, i32 action,
                                      i32 mods)
{
  Requires(0 <= button && button < NUM_MOUSE_BUTTONS, "unknown mouse button");

  switch (action) {
  case GLFW_PRESS: {
    auto &bits = m_mouseButtons[button];
    bits.set(IS_PRESSED_BIT_OFFSET, 1);
    bits |= MODIFIERS_BIT_MASK & mods;

    Event::MousePressed ev;
    ev.button = static_cast<MouseInput>(button);
    ev.modifiers = static_cast<KeyModifiers>(mods);
    EventManager::Invoke<Event::MousePressed>(ev);

    break;
  }
  default: {
    auto &bits = m_mouseButtons[button];
    bits.set(IS_PRESSED_BIT_OFFSET, 0);
    bits |= MODIFIERS_BIT_MASK & mods;

    Event::MousePressed ev;
    ev.button = static_cast<MouseInput>(button);
    ev.modifiers = static_cast<KeyModifiers>(mods);
    EventManager::Invoke<Event::MousePressed>(ev);

    break;
  }
  }
}

void Input::OnWindowMouseMoveEvent(GLFWwindow *, double xpos, double ypos)
{
  m_mouseX = xpos;
  m_mouseY = ypos;

  Event::MouseMoved ev;
  ev.mouseX = m_mouseX;
  ev.mouseY = m_mouseY;
  EventManager::Invoke<Event::MouseMoved>(ev);
}

void Input::OnWindowMouseScrolledEvent(GLFWwindow *, double xoffset,
                                       double yoffset)
{
  m_scrollX = xoffset;
  m_scrollY = yoffset;

  Event::MouseScrolled ev;
  ev.scrollX = m_scrollX;
  ev.scrollY = m_scrollY;
  EventManager::Invoke<Event::MouseScrolled>(ev);
}

bool Input::IsKeyPressed(KeyInput key, KeyModifiers modifiers)
{
  const auto &bits = m_keyPresses[static_cast<usize>(key)];
  bool isPressed = bits.test(IS_PRESSED_BIT_OFFSET);
  bool isModifiers = static_cast<bool>(bits.to_ulong() & modifiers);
  return isPressed && isModifiers;
}

bool Input::IsMouseButtonDown(MouseInput key, KeyModifiers modifiers)
{
  const auto &bits = m_mouseButtons[static_cast<usize>(key)];
  bool isPressed = bits.test(IS_PRESSED_BIT_OFFSET);
  bool isModifiers = static_cast<bool>(bits.to_ulong() & modifiers);
  return isPressed && isModifiers;
}