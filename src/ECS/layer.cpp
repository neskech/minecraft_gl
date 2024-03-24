#include "Ecs/Layer.hpp"
#include "EcsConstants.hpp"
#include "util/contracts.hpp"

namespace ECS
{
  void LayerRegistry::AddLayerName(std::string_view name)
  {
    Requires(!m_layerToIndex.contains(name), "Layer already exists!");
    Requires(m_layerToIndex.size() + 1 < MAX_LAYERS, "Too many layers!");
    m_layerToIndex[name] = m_layerToIndex.size();
  }

  LayerMask LayerRegistry::GetLayerMaskByName(std::string_view name)
  {
    LayerMask mask;
    mask.set(GetLayerIndexByName(name));
    return mask;
  }

  usize LayerRegistry::GetLayerIndexByName(std::string_view name)
  {
    Requires(m_layerToIndex.contains(name), "Layer does not exist!");
    return m_layerToIndex.at(name);
  }

  usize LayerRegistry::GetLayerCount() { return m_layerToIndex.size(); }
} // namespace ECS