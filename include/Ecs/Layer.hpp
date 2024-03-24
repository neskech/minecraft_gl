#pragma once
#include "EcsConstants.hpp"
#include "pch.hpp"
#include "util/macros.hpp"

using LayerMask = std::bitset<MAX_LAYERS>;

class LayerRegistry
{
  public:
    LayerRegistry();
    NO_COPY_OR_MOVE_CONSTRUCTORS(LayerRegistry)

    void AddLayerName(const char* name);
    std::bitset<MAX_LAYERS> GetLayerNameMask(const char* name);
    usize GetLayerNameIndex(const char* name);
    usize GetLayerCount();
    
  private:
    std::unordered_map<const char *, usize> m_layerToIndex;
};