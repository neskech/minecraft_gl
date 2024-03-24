#pragma once
#include "pch.hpp"
#include "util/contracts.hpp"
#include "util/types.hpp"
#include <algorithm>

template <typename T, usize... Dimensions>
class StaticNDArray
{
  public:
    StaticNDArray(const StaticNDArray &other) = default;
    StaticNDArray(StaticNDArray &&other) = default;

    StaticNDArray()
    {
      std::array<usize, sizeof...(Dimensions)> dims{Dimensions...};

      usize prod = 1;
      m_coefficients[0] = prod;

      for (u32 i = 1; i < dims.size(); i++) {
        prod *= dims[dims.size() - i];
        m_coefficients[i] = prod;
      }

      /*
        For 3D arrays, are dimensions are
        H = height
        R = rows
        C = columns

        [H, R, C]

        We want our indices to be multiplied with
        their coefficients like so
        (x, y, z) -> (x * RC, y * C, z * 1)

        Before reversal, our coefficient list looks like this
        [1, C, RC]

        After, it's this
        [RC, C, 1]
      */

      std::reverse(m_coefficients.begin(), m_coefficients.end());
    }

    T &operator[](std::integral auto... indices)
    {
      static_assert(sizeof...(indices) == sizeof...(Dimensions));

      usize index1D = 0;
      usize coefIndex = 0;
      ((index1D += m_coefficients[coefIndex++] * indices), ...);

      Assert(0 <= index1D && index1D < m_data.size(), "Index out of bounds");
      return m_data[index1D];
    }

    T &operator[](std::integral auto index1D)
    {
      Assert(0 <= index1D && index1D < m_data.size(), "Index out of bounds");
      return m_data[index1D];
    }

    void ZeroOut(T &defaultValue)
    {
      for (usize i = 0; i < m_data.size(); i++)
        m_data[i] = defaultValue;
    }

    usize NDimensionalIndexTo1D(std::integral auto... indices)
    {
      static_assert(sizeof...(indices) == sizeof...(Dimensions));

      usize index1D = 0;
      usize coefIndex = 0;
      ((index1D += m_coefficients[coefIndex++] * indices), ...);

      return index1D;
    }

    std::array<usize, sizeof...(Dimensions)>
    OneDimensionalIndexToND(usize index)
    {
      usize i = 0;
      std::print("{}", m_coefficients[0]);
      return std::array<usize, sizeof...(Dimensions)>{
          ((index / m_coefficients[i++]) % Dimensions)...};
    }

    std::array<T, (... * Dimensions)> &GetInnerArray() { return m_data; }
    usize GetSize() { return m_data.size(); }

  private:
    std::array<T, (... * Dimensions)> m_data;
    std::array<u32, sizeof...(Dimensions)> m_coefficients;
};

template <typename T, usize... Dimensions>
class NDArray
{
  public:
    NDArray(const NDArray &other) = default;

    NDArray(NDArray<T, Dimensions...> &&other)
    {
      m_data = std::move(other.m_data);
    }

    NDArray()
    {
      m_data.reserve((... * Dimensions));

      std::array<usize, sizeof...(Dimensions)> dims{Dimensions...};

      usize prod = 1;
      m_coefficients[0] = prod;

      for (u32 i = 1; i < dims.size() - 1; i--) {
        prod *= dims[dims.size() - i];
        m_coefficients[i] = prod;
      }

      /*
        For 3D arrays, are dimensions are
        H = height
        R = rows
        C = columns

        [H, R, C]

        We want our indices to be multiplied with
        their coefficients like so
        (x, y, z) -> (x * RC, y * C, z * 1)

        Before reversal, our coefficient list looks like this
        [1, C, RC]

        After, it's this
        [RC, C, 1]
      */

      std::reverse(m_coefficients.begin(), m_coefficients.end());
    }

    T &operator[](std::integral auto... indices)
    {
      static_assert(sizeof...(indices) == sizeof...(Dimensions));

      usize index1D = 0;
      usize coefIndex = 0;
      ((index1D += m_coefficients[coefIndex++] * indices), ...);

      Assert(0 <= index1D && index1D < m_data.size(), "Index out of bounds");
      return m_data[index1D];
    }

    T &operator[](std::integral auto index1D)
    {
      Assert(0 <= index1D && index1D < m_data.size(), "Index out of bounds");
      return m_data[index1D];
    }

    void ZeroOut(T &defaultValue)
    {
      for (usize i = 0; i < m_data.size(); i++)
        m_data[i] = defaultValue;
    }

    usize NDimensionalIndexTo1D(std::integral auto... indices)
    {
      static_assert(sizeof...(indices) == sizeof...(Dimensions));

      usize index1D = 0;
      usize coefIndex = 0;
      ((index1D += m_coefficients[coefIndex++] * indices), ...);

      return index1D;
    }

    std::array<usize, sizeof...(Dimensions)>
    OneDimensionalIndexToND(usize index)
    {
      usize i = 0;
      return std::array<usize, sizeof...(Dimensions)>{
          ((index / m_coefficients[i++]) % Dimensions)...};
    }

    std::vector<T> &GetInnerArray() { return m_data; }
    usize GetSize() { return m_data.size(); }

  private:
    std::vector<T> m_data;
    std::array<u32, sizeof...(Dimensions)> m_coefficients;
};
