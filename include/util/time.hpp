#pragma once

class Time
{
  public:
    static inline float GetDeltaTime() { return Instance().m_deltaTime; }
    static inline float GetTime() { return 0; }
    static inline float GetTimeScale() { return Instance().m_timeScale; };
    static inline void SetTimeScale(float scale)
    {
      Instance().m_timeScale = scale;
    }

  private:
    static inline Time &Instance()
    {
      static Time time{};
      return time;
    }
    float m_deltaTime = 0;
    float m_timeScale = 1;
};