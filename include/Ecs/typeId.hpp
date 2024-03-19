
#pragma once
#include "util/types.hpp"

template <typename Category> class TypeIdMaker
{
  public:
    template <typename T> static usize GetId()
    {
      auto& self = Instance();

      static usize typeId = self.m_globalCount;

      if (typeId == self.m_globalCount)
        self.m_globalCount++;

      return typeId;
    }

  private:
    static TypeIdMaker<Category> &Instance()
    {
      static TypeIdMaker<Category> self{};
      return self;
    }

    usize m_globalCount = 0;
};