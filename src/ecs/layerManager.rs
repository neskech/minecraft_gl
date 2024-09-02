use std::collections::HashMap;

use crate::ecs::constants::MAX_COMPONENT_TYPES;

use super::constants::MAX_LAYERS;

pub struct LayerMask {
    bitSet: bit_set::BitSet,
}

impl LayerMask {
    fn New() -> LayerMask {
        LayerMask {
            bitSet: bit_set::BitSet::with_capacity(MAX_LAYERS),
        }
    }
}

pub struct LayerManager {
    layerNameToIndex: HashMap<String, usize>,
}

impl LayerManager {
    pub fn New() -> LayerManager {
        LayerManager {
            layerNameToIndex: HashMap::new(),
        }
    }

    pub fn AddLayer(&mut self, layerName: &str) {
        assert!(!self.layerNameToIndex.contains_key(layerName));
        assert!(self.layerNameToIndex.len() + 1 < MAX_COMPONENT_TYPES);
        self.layerNameToIndex
            .insert(layerName.into(), self.layerNameToIndex.len());
    }

    pub fn GetLayerMaskFromLayer(&self, layerName: &str) -> LayerMask {
        assert!(self.layerNameToIndex.contains_key(layerName));
        let mut mask = LayerMask::New();
        mask.bitSet.insert(self.GetLayerIndexByName(layerName));
        mask
    }

    pub fn GetLayerMaskFromLayers(&self, layerNames: Vec<&str>) -> LayerMask {
        let mut mask = LayerMask::New();
        for name in layerNames {
            assert!(self.layerNameToIndex.contains_key(name));
            mask.bitSet.insert(self.GetLayerIndexByName(name));
        }
        mask
    }

    pub fn GetLayerCount(&self) -> usize {
        self.layerNameToIndex.len()
    }

    fn GetLayerIndexByName(&self, layerName: &str) -> usize {
        assert!(self.layerNameToIndex.contains_key(layerName));
        *self.layerNameToIndex.get(layerName).unwrap()
    }
}
