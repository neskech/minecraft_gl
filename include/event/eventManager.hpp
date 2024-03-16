#pragma once
#include "event.hpp"
#include "pch.hpp"
#include "util/macros.hpp"
#include "util/types.hpp"

namespace Event
{
  class EventManager
  {
    public:
      EventManager() : m_globalCount(0) {}
      NO_COPY_OR_MOVE_CONSTRUCTORS(EventManager)

      template <typename E>
        requires std::is_base_of_v<Event, E>
      static void Invoke(const E &event)
      {
        EventManager &self = EventManager::Instance();

        u32 typeID = self.typeId<E>();
        for (const auto &[f, _] : self.m_functions[typeID])
          f(static_cast<const Event &>(event));
      }

      template <typename E, typename Fn>
        requires std::invocable<Fn, const E &> &&
                 (std::is_base_of_v<Event, E> || std::is_same_v<Event, E>)
      static void Subscribe(Fn fn)
      {
        EventManager &self = EventManager::Instance();

        u32 typeID = self.typeId<E>();
        if (typeID + 1 > self.m_functions.size())
          self.m_functions.emplace_back();

        FunctionList &functionList = self.m_functions[typeID];

        Function func;
        func.f = [&](const Event &e) { fn(static_cast<const E &>(e)); };
        func.address = Optional::None<usize>();

        functionList.push_back(func);
      }

      template <typename E, typename Fn>
        requires std::invocable<Fn, const E &> &&
                 (std::is_base_of_v<Event, E> || std::is_same_v<Event, E>)
      static void SubscribeRef(const Fn &fn)
      {
        EventManager &self = EventManager::Instance();

        u32 typeID = self.typeId<E>();
        if (typeID + 1 > self.m_functions.size())
          self.m_functions.emplace_back();

        FunctionList &functionList = self.m_functions[typeID];

        Function func;
        func.f = [&](const Event &e) { fn(static_cast<const E &>(e)); };
        func.address = Optional::Some<usize>(reinterpret_cast<usize>(&fn));

        functionList.push_back(func);
      }

      template <typename E, typename Fn>
        requires std::invocable<Fn, const E &> &&
                 (std::is_base_of_v<Event, E> || std::is_same_v<Event, E>)
      static void UnSubscribe(Fn &fn)
      {
        EventManager &self = EventManager::Instance();

        u32 typeID = self.typeId<E>();
        if (typeID + 1 > self.m_functions.size())
          self.m_functions.emplace_back();

        FunctionList &functionList = self.m_functions[typeID];
        for (i32 i = functionList.size() - 1; i >= 0; i--) {
          auto &[_, adr] = functionList[i];
          if (adr.has_value() && adr.value() == reinterpret_cast<usize>(&fn))
            functionList.erase(std::next(functionList.begin(), i));
        }
      }

    private:
      static EventManager &Instance()
      {
        static EventManager manager{};
        return manager;
      }

      template <typename T> u32 typeId()
      {
        static u32 id = m_globalCount;

        if (id == m_globalCount)
          m_globalCount++;

        return id;
      }

      using Func = std::function<void(const Event &)>;

      struct Function
      {
          Func f;
          Option<usize> address;
      };

      using FunctionList = std::vector<Function>;

      std::vector<FunctionList> m_functions;
      u32 m_globalCount = 0;
  };
} // namespace Event
