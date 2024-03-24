#pragma once
#include "EcsConstants.hpp"
#include "pch.hpp"
#include "util/macros.hpp"

namespace ECS
{
  using LayerMask = std::bitset<MAX_LAYERS>;

  class LayerRegistry
  {
    public:
      LayerRegistry();
      NO_COPY_OR_MOVE_CONSTRUCTORS(LayerRegistry)

      void AddLayerName(std::string_view name);
      LayerMask GetLayerMaskByName(std::string_view name);
      usize GetLayerIndexByName(std::string_view name);
      usize GetLayerCount();

    private:
      std::unordered_map<std::string_view, usize> m_layerToIndex;
  };

} // namespace ECS