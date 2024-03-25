#pragma once
#include "pch.hpp"
#include "util/contracts.hpp"
#include "util/types.hpp"
#include <functional>
#include <utility>

template <typename EventType>
class CallbackContainer
{
  public:
    using Callback = std::function<void(const EventType &)>;

    struct SubscriberHandle
    {
        friend CallbackContainer<EventType>;

        bool operator==(const SubscriberHandle &other) const
        {
          return id == other.id;
        }

        struct HashFunction
        {
            usize operator()(const SubscriberHandle &handle) const
            {
              return std::hash<usize>{}(handle.id);
            }
        };

      private:
        u32 id;
    };

    struct CallbackAndHandle
    {
        Callback callback;
        SubscriberHandle handle;
        CallbackAndHandle(Callback &&callback, SubscriberHandle handle)
            : callback(std::move(callback)), handle(handle)
        {}
    };

    CallbackContainer() {}

    SubscriberHandle Subscribe(Callback callback);
    void UnSubscribe(SubscriberHandle handle);
    void Invoke(const EventType &event);

  private:
    std::vector<CallbackAndHandle> m_callbacks;
    std::vector<SubscriberHandle> m_freeHandles;
    std::unordered_map<SubscriberHandle, usize,
                       typename SubscriberHandle::HashFunction>
        m_handleMap;
};

/* To Shorten */
template <typename EvType>
using Class = CallbackContainer<EvType>;
template <typename EvType>
using SubscriberHandle = CallbackContainer<EvType>::SubscriberHandle;

template <typename EType>
SubscriberHandle<EType> Class<EType>::Subscribe(Callback callback)
{
  SubscriberHandle handle;

  if (m_freeHandles.empty())
    handle.id = m_callbacks.size();
  else {
    handle = m_freeHandles.back();
    m_freeHandles.pop_back();
  }

  usize lastIndex = m_callbacks.size();
  m_handleMap[handle] = lastIndex;

  m_callbacks.emplace_back(std::move(callback), handle);

  return handle;
}

template <typename EType>
void Class<EType>::UnSubscribe(SubscriberHandle deletionHandle)
{
  Requires(m_handleMap.contains(deletionHandle));

  usize elemToDeleteIndex = m_handleMap.at(deletionHandle);
  usize lastIndex = m_callbacks.size() - 1;

  /* Swap target element with last element for O(1) delete */
  if (elemToDeleteIndex != lastIndex) {
    SubscriberHandle swappedHandle = m_callbacks[lastIndex].handle;
    m_handleMap[swappedHandle] = elemToDeleteIndex;
    std::swap(m_callbacks[elemToDeleteIndex], m_callbacks[lastIndex]);
  }

  m_callbacks.pop_back();
  m_handleMap.erase(deletionHandle);
  m_freeHandles.push_back(deletionHandle);
}

template <typename EType>
void Class<EType>::Invoke(const EType &event)
{
  for (const auto &[callback, _] : m_callbacks)
    callback(event);
}

class EventManager
{
  public:
    EventManager()
    {
      Assert(s_instance == nullptr, "Can only initialize event manager once!");
      s_instance = this;
    }

    template <typename EventType, typename Function>
    typename CallbackContainer<EventType>::SubscriberHandle static Subscribe(
        Function f);

    template <typename EventType, typename Method, typename ClassInstance>
    typename CallbackContainer<EventType>::SubscriberHandle static Subscribe(
        Method method, ClassInstance instance);

    template <typename EventType>
    void static UnSubscribe(
        typename CallbackContainer<EventType>::SubscriberHandle handle);

    template <typename EventType>
    static void Invoke(const EventType &event);

    template <typename EventType>
    static void Invoke(EventType &&event);

  private:
    static inline EventManager *s_instance;
    template <typename EventType>
    static inline CallbackContainer<EventType> s_callbackContainer;
};

template <typename EventType, typename Function>
typename CallbackContainer<EventType>::SubscriberHandle
EventManager::Subscribe(Function f)
{
  return s_callbackContainer<EventType>.Subscribe(f);
}

template <typename EventType, typename Method, typename ClassInstance>
typename CallbackContainer<EventType>::SubscriberHandle
EventManager::Subscribe(Method method, ClassInstance instance)
{
  typename CallbackContainer<EventType>::Callback callback =
      std::bind(method, instance, std::placeholders::_1);
  return s_callbackContainer<EventType>.Subscribe(callback);
}

template <typename EventType>
void EventManager::UnSubscribe(
    typename CallbackContainer<EventType>::SubscriberHandle handle)
{
  s_callbackContainer<EventType>.UnSubscribe(handle);
}

template <typename EventType>
void EventManager::Invoke(const EventType &event)
{
  s_callbackContainer<EventType>.Invoke(event);
}

template <typename EventType>
void EventManager::Invoke(EventType &&event)
{
  s_callbackContainer<EventType>.Invoke(std::forward<EventType>(event));
}