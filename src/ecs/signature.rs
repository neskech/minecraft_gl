use super::{componentIdMaker::ComponentIdMaker, constants::MAX_COMPONENT_TYPES};

#[derive(Debug)]
pub struct Signature {
    bitset: bit_set::BitSet,
}

impl Signature {
    fn New() -> Signature {
        Signature {
            bitset: bit_set::BitSet::with_capacity(MAX_COMPONENT_TYPES),
        }
    }
}
pub struct SignatureBuilder<'a> {
    signature: Signature,
    componentIdMaker: &'a mut ComponentIdMaker,
}

impl<'a> SignatureBuilder<'a> {
    pub fn New(componentIdMaker: &mut ComponentIdMaker) -> SignatureBuilder {
        SignatureBuilder {
            signature: Signature::New(),
            componentIdMaker,
        }
    }

    pub fn AddComponent<T: 'static>(mut self) -> SignatureBuilder<'a> {
        let id = self.componentIdMaker.GetComponentId::<T>();
        self.signature.bitset.insert(id);
        self
    }

    pub fn Build(self) -> Signature {
        self.signature
    }
}
