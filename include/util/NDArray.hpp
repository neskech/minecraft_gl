#pragma once
#include "pch.hpp"
#include "util/contracts.hpp"
#include "util/types.hpp"
#include <cstring>
#include <memory>

template <typename T, usize Rows, usize Columns> class StaticArray2D
{
  public:
    StaticArray2D() = default;
    StaticArray2D(const StaticArray2D &other) = default;
    StaticArray2D(StaticArray2D &&other) = default;

    T &operator[](usize row, usize col)
    {
      Requires(0 <= row && row < Rows);
      Requires(0 <= col && col < Columns);
      return m_data[row * Columns + col];
    }

    void ZeroOut(T &defaultValue)
    {
      for (usize i = 0; i < Rows * Columns; i++)
        m_data[i] = defaultValue;
    }

    usize Size() { return m_data.size(); }

    std::array<T, Rows * Columns> &GetInner() { return m_data; }

  private:
    std::array<T, Rows * Columns> m_data;
};

template <typename T, usize Slices, usize Rows, usize Columns>
class StaticArray3D
{
  public:
    StaticArray3D() = default;
    StaticArray3D(const StaticArray3D &other) = default;
    StaticArray3D(StaticArray3D &&other) = default;

    T &operator[](usize slice, usize row, usize col)
    {
      Requires(0 <= slice && slice < Slices);
      Requires(0 <= row && row < Rows);
      Requires(0 <= col && col < Columns);
      return m_data[slice * Rows * Columns + row * Columns + col];
    }

    void ZeroOut(T &defaultValue)
    {
      for (usize i = 0; i < Slices * Rows * Columns; i++)
        m_data[i] = defaultValue;
    }

    usize Size() { return m_data.size(); }

    std::array<T, Slices * Rows * Columns> &GetInner() { return m_data; }

  private:
    std::array<T, Slices * Rows * Columns> m_data;
};

template <typename T> class Array2D
{
  public:
    Array2D(usize rows, usize columns)
    {
      m_rows = rows;
      m_cols = columns;
      m_data = std::unique_ptr<T>(new T[rows * columns]);
    }

    Array2D(const Array2D &other)
    {
      T *data = m_data.get();
      T *otherData = other.m_data.get();
      memcpy(data, otherData, Size() * sizeof(T));
    }

    Array2D(Array2D &&other)
    {
      m_data.reset();
      m_data = std::move(other.m_data);
    }

    T &operator[](usize row, usize col)
    {
      Requires(0 <= row && row < m_rows);
      Requires(0 <= col && col < m_cols);
      return m_data.get()[row * m_cols + col];
    }

    void ZeroOut(T &defaultValue)
    {
      for (usize i = 0; i < m_rows * m_cols; i++)
        m_data.get()[i] = defaultValue;
    }

    usize Rows() { return m_rows; }
    usize Columns() { return m_cols; }
    usize Size() { return m_rows * m_cols; }

  private:
    Box<T> m_data;
    usize m_rows;
    usize m_cols;
};

template <typename T> class Array3D
{
  public:
    Array3D(usize slices, usize rows, usize columns)
    {
      m_slices = slices;
      m_rows = rows;
      m_cols = columns;
      m_data = std::unique_ptr<T>(new T[slices * rows * columns]);
    }

    Array3D(const Array3D &other)
    {
      T *data = m_data.get();
      T *otherData = other.m_data.get();
      memcpy(data, otherData, Size() * sizeof(T));
    }

    Array3D(Array3D &&other)
    {
      m_data.reset();
      m_data = std::move(other.m_data);
    }

    T &operator[](usize slice, usize row, usize col)
    {
      Requires(0 <= slice && slice < m_slices);
      Requires(0 <= row && row < m_rows);
      Requires(0 <= col && col < m_cols);
      return m_data.get()[row * m_cols + col];
    }

    void ZeroOut(T &defaultValue)
    {
      for (usize i = 0; i < m_slices * m_rows * m_cols; i++)
        m_data.get()[i] = defaultValue;
    }

    usize Slices() { return m_slices; }
    usize Rows() { return m_rows; }
    usize Columns() { return m_cols; }
    usize Size() { return m_slices * m_rows * m_cols; }

  private:
    Box<T> m_data;
    usize m_slices;
    usize m_rows;
    usize m_cols;
};