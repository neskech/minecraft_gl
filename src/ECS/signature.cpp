#include "Ecs/signature.hpp"
#include "Ecs/EcsConstants.hpp"

template <typename ComponentType>
SignatureBuilder &SignatureBuilder::AddComponentType()
{
  usize id = m_manager.ComponentID<ComponentType>();
  m_signature.set(id);
}

template <int N, typename... Ts>
using NthTypeOf = typename std::tuple_element<N, std::tuple<Ts...>>::type;

template <typename... ComponentTypes>
SignatureBuilder &SignatureBuilder::AddComponentTypes()
{
  constexpr usize N = sizeof...(ComponentTypes);
  usize id;

  if constexpr (N == 1) {
    using LastType = NthTypeOf<N - 1, ComponentTypes...>;
    id = m_manager.ComponentID<LastType>();
  }
  else {
    using FirstType = NthTypeOf<0, ComponentTypes...>;
    id = m_manager.ComponentID<FirstType>();
    AddComponentTypes<ComponentTypes...>();
  }

  m_signature.set(id);
}

template <typename... ComponentTypes> Signature SignatureBuilder::Build() {
    AddComponentTypes<ComponentTypes...>();
    return Finish();
}

Signature SignatureBuilder::Finish() {
    return m_signature;
}