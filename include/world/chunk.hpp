#pragma once
#include "glm/fwd.hpp"
#include "util/NDArray.hpp"
#include "util/types.hpp"
#include "world/block.hpp"
#include "world/chunkMesher.hpp"
#include "world/generation/worldGenerator.hpp"
#include "rendering/mesh.hpp"

class Chunk
{
  public:

  enum State {
    Empty,
    Blocked,
    Meshed
  };

  Chunk();
  Chunk(const Chunk& other);
  Chunk(Chunk&& other);

  void GenerateBlocks(WorldGenerator& generator);
  void GenerateMesh(ChunkMesher& mesher);

  private:
    Array3D<BlockID> m_blocks;
    State m_state;
    Vector2 m_position;
};