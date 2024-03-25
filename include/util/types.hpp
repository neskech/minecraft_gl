#pragma once
#include "glm/fwd.hpp"
#include "pch.hpp"
#include <__expected/unexpected.h>

template <typename T> using Box = std::unique_ptr<T>;

template <typename T> using Ref = std::shared_ptr<T>;

template <typename T, typename... Args> inline Box<T> MakeBox(Args &&...args)
{
  return std::make_unique<T>(std::forward<T>(args)...);
}

template <typename T, typename... Args> inline Ref<T> MakeRef(Args &&...args)
{
  return std::make_shared<T>(std::forward<T>(args)...);
}

template <typename T> using Option = std::optional<T>;

template <typename T, typename E> using Result = std::expected<T, E>;

namespace Optional
{

  template <typename T> inline Option<T> Some(T &&t)
  {
    return std::optional(t);
  }

  template <typename T> inline Option<T> None() { return std::optional<T>(); }

} // namespace Option

template <typename T, typename E> inline Result<T, E> Ok(T &&t)
{
  return std::forward<T>(t);
}

template <typename T, typename E> inline Result<T, E> Err(E &&e)
{
  return std::unexpected<E>(std::forward<E>(e));
}

template <typename T, typename E> inline Result<T, E> ErrWithCopy(E e)
{
  return std::unexpected<E>(e);
}

struct Unit {};
typedef glm::vec2 Vector2;
typedef glm::vec3 Vector3;
typedef glm::vec4 Vector4;
typedef glm::ivec2 IntVector2;
typedef glm::ivec3 IntVector3;
typedef glm::mat3 Matrix3;
typedef glm::mat4 Matrix4;

typedef u_int8_t u8;
typedef u_int16_t u16;
typedef u_int32_t u32;
typedef u_int64_t u64;
typedef size_t usize;

typedef int8_t i8;
typedef int16_t i16;
typedef int32_t i32;
typedef int64_t i64;
typedef int64_t isize;

typedef float f32;
typedef double f64;