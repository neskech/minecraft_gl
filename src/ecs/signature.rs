use super::{constants::MAX_COMPONENT_TYPES, typeIdMaker::TypeIdMaker};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Signature {
    bitset: bit_set::BitSet,
}

impl Signature {
    fn New() -> Signature {
        Signature {
            bitset: bit_set::BitSet::with_capacity(MAX_COMPONENT_TYPES),
        }
    }

    pub fn AddComponent<T: 'static>(&mut self, componentIdMaker: &mut TypeIdMaker) {
        let id = componentIdMaker.GetTypeId::<T>();
        self.bitset.insert(id);
    }

    pub fn RemoveComponent<T: 'static>(&mut self, componentIdMaker: &mut TypeIdMaker) {
        let id = componentIdMaker.GetTypeId::<T>();
        self.bitset.remove(id);
    }
}
pub struct SignatureBuilder<'a> {
    signature: Signature,
    componentIdMaker: &'a mut TypeIdMaker,
}

impl<'a> SignatureBuilder<'a> {
    pub fn New(componentIdMaker: &mut TypeIdMaker) -> SignatureBuilder {
        SignatureBuilder {
            signature: Signature::New(),
            componentIdMaker,
        }
    }

    pub fn AddComponent<T: 'static>(mut self) -> SignatureBuilder<'a> {
        let id = self.componentIdMaker.GetTypeId::<T>();
        self.signature.bitset.insert(id);
        self
    }

    pub fn Build(self) -> Signature {
        self.signature
    }
}
