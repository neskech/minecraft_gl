
#include "GLFW/glfw3.h"
#include "event/event.hpp"
#include "macros.hpp"
#include "util/inputMap.hpp"
#include "util/types.hpp"
#include <bitset>

constexpr usize NUM_KEYS = 350;
constexpr usize NUM_MOUSE_BUTTONS = 8;

class Input
{
  public:
    Input() {}
    NO_COPY_OR_MOVE_CONSTRUCTORS(Input)

    bool IsKeyPressed(KeyInput key,
                      KeyModifiers modifiers = KeyModifiers::None);
    bool IsMouseButtonDown(MouseInput button,
                           KeyModifiers modifiers = KeyModifiers::None);

    inline float GetMouseX() { return m_mouseX; }
    inline float GetMouseY() { return m_mouseY; }
    inline float GetScrollX() { return m_scrollX; }
    inline float GetScrollY() { return m_scrollY; }

    void OnWindowKeyEvent(GLFWwindow *, i32 key, i32 scancode, i32 action,
                          i32 mods);
    void OnWindowMousePressedEvent(GLFWwindow *, i32 button, i32 action,
                                   i32 mods);
    void OnWindowMouseMoveEvent(GLFWwindow *, double xpos, double ypos);
    void OnWindowMouseScrolledEvent(GLFWwindow *, double xoffset,
                                    double yoffset);

  private:
    std::array<std::bitset<8>, NUM_KEYS> m_keyPresses;
    std::array<std::bitset<8>, NUM_MOUSE_BUTTONS> m_mouseButtons;
    float m_mouseX, m_mouseY;
    float m_scrollX, m_scrollY;
};

namespace Event
{
  struct KeyPressed : Event
  {
      KeyInput key;
      KeyModifiers modifiers;
  };

  struct KeyUp : Event
  {
      KeyInput key;
      KeyModifiers modifiers;
  };

  struct MousePressed : Event
  {
      MouseInput button;
      KeyModifiers modifiers;
  };

  struct MouseMoved : Event
  {
      float mouseX;
      float mouseY;
  };

  struct MouseScrolled : Event
  {
      float scrollX;
      float scrollY;
  };
} // namespace Event