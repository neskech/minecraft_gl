
#pragma once
#include "util/types.hpp"

class TypeIdMaker
{
  public:
    template <typename T> usize GetId()
    {
      static usize typeId = m_globalCount;

      if (typeId == m_globalCount)
        m_globalCount++;

      return typeId;
    }

  private:
    usize m_globalCount = 0;
};