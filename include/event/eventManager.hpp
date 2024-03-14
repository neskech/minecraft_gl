
#include "hello.hpp"
#include <concepts>
#include <type_traits>
#include <utility>
#include "event.hpp"

class EventManager {
    public:

    EventManager() {}
    NO_COPY_OR_MOVE_CONSTRUCTORS(EventManager)

    template<typename E>
    requires std::is_base_of_v<Event, E>
    void Invoke(const E&& event) {}

    template<typename E, typename Fn>
    requires std::invocable<Fn, E> && (std::is_base_of_v<Event, E> || std::is_same_v<Event, E>)
    void Subscribe(const Fn&& event) {}
    
    private:

    static EventManager m_instance; 
};

