
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
    Input();
    NO_COPY_OR_MOVE_CONSTRUCTORS(Input)

    static bool IsKeyPressed(KeyInput key,
                      KeyModifiers modifiers = KeyModifiers::None);
    static bool IsMouseButtonDown(MouseInput button,
                           KeyModifiers modifiers = KeyModifiers::None);

    static inline float GetMouseX() { return s_instance->m_mouseX; }
    static inline float GetMouseY() { return s_instance->m_mouseY; }
    static inline float GetScrollX() { return s_instance->m_scrollX; }
    static inline float GetScrollY() { return s_instance->m_scrollY; }

    static void OnWindowKeyEvent(GLFWwindow *, i32 key, i32 scancode, i32 action,
                          i32 mods);
    static void OnWindowMousePressedEvent(GLFWwindow *, i32 button, i32 action,
                                   i32 mods);
    static void OnWindowMouseMoveEvent(GLFWwindow *, double xpos, double ypos);
    static void OnWindowMouseScrolledEvent(GLFWwindow *, double xoffset,
                                    double yoffset);

  private:
    inline static Input *s_instance;
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