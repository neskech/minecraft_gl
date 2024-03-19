#pragma once
#include "Ecs/EcsConstants.hpp"
#include "Ecs/component.hpp"
#include "typeId.hpp"

using Signature = std::bitset<MAX_COMPONENTS>;

class SignatureBuilder
{
  public:
    SignatureBuilder() {}

    template <typename ComponentType> SignatureBuilder &AddComponentType();
    template <typename... ComponentTypes> SignatureBuilder &AddComponentTypes();
    Signature Finish() {return m_signature; }

  private:
    template <typename ComponentType> usize ComponentID()
    {
      return TypeIdMaker<Component::Component>::GetId<ComponentType>();
    }

    Signature m_signature = 0;
};

template <typename ComponentType>
SignatureBuilder &SignatureBuilder::AddComponentType()
{
  usize id = ComponentID<ComponentType>();
  m_signature.set(id);
  
  return *this;
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
    id = ComponentID<LastType>();
  }
  else {
    using FirstType = NthTypeOf<0, ComponentTypes...>;
    id = ComponentID<FirstType>();
    AddComponentTypes<ComponentTypes...>();
  }

  m_signature.set(id);

  return *this;
}
