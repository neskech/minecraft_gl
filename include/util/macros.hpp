#pragma once

#define NO_COPY_OR_MOVE(T, x)                                                  \
  T##x(T##x &) = delete;                                                       \
  T##x(T##x &&) = delete;                                                      \
  T##x(const T##x &) = delete;                                                 \
  T##x(const T##x &&) = delete;

#define NO_COPY_OR_MOVE_CONSTRUCTORS(T) NO_COPY_OR_MOVE(T, )