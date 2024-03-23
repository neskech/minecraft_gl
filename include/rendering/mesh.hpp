#pragma once
#include "pch.hpp"
#include "util/types.hpp"

class IMesh
{
  public:
    f32 *GetVertices();
    u32 *GetIndices();
};

template <typename VertexType>
class Mesh
{
  public:
  private:
    std::vector<VertexType> m_vertices;
    Option<std::vector<u32>> m_indices;
};