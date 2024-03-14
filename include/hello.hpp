#pragma once
#include "pch.hpp"

template <typename T>
using Box = std::unique_ptr<T>;

template <typename T>
using Ref = std::shared_ptr<T>;

template <typename T, typename... Args>
inline Box<T> MakeBox(Args&&... args) {
    return std::make_unique<T>(std::forward<T>(args)...);
}

template <typename T, typename... Args>
inline Box<T> MakeRef(Args&&... args) {
    return std::make_shared<T>(std::forward<T>(args)...);
}

template <typename T>
using Option = std::optional<T>;

template <typename T, typename E>
using Result = std::expected<T, E>;

template <typename T>
inline Option<T> Some(T&& t) {
    return std::optional(t);
}

template <typename T>
inline Option<T> None() {
    return std::optional<T>();
}

template <typename T, typename E>
inline Result<T, E> Ok(T&& t) {
    return std::forward<T>(t);
}

template <typename T, typename E>
inline Option<T> Err(E&& e) {
   return std::expected(std::forward<E>(e));
}

#define NO_COPY_OR_MOVE(T, x) \
    T## x(T##x &) = delete; \
    T##x (T##x &&) = delete; \
    T## x(const T##x &) = delete; \
    T## x(const T##x &&) = delete; \
    
#define NO_COPY_OR_MOVE_CONSTRUCTORS(T) NO_COPY_OR_MOVE(T, )
