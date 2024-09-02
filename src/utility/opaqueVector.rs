use std::{
    alloc::{alloc, dealloc, realloc, Layout},
    any::TypeId,
    ptr::null_mut,
};

const RESIZE_FACTOR: f64 = 1.5;
const SHRINK_REQUIREMENT: f64 = 2.0;

#[derive(Debug)]
pub struct OpaqueVector {
    data: *mut u8,
    size: usize,
    capacity: usize,
    typeSize: usize,
    typeId: TypeId,
}

impl OpaqueVector {
    pub fn New<T: 'static>(capacity: usize) -> OpaqueVector {
        let typeSize = std::mem::size_of::<T>();
        let layout = Layout::array::<u8>(capacity * typeSize).unwrap();
        let data = unsafe { alloc(layout) };
        OpaqueVector {
            data,
            size: 0,
            capacity: capacity,
            typeSize,
            typeId: TypeId::of::<T>(),
        }
    }

    fn TryResize<T>(&mut self, newSize: usize) {
        if (newSize > self.capacity) {
            return self.Grow(newSize);
        }

        let shrinkedCap = self.capacity as f64 / SHRINK_REQUIREMENT;
        if (newSize < self.capacity && newSize as f64 <= shrinkedCap) {
            return self.Shrink::<T>(newSize);
        }
    }

    fn Grow(&mut self, newSize: usize) {
        let newCapacity = self.typeSize * (RESIZE_FACTOR * self.capacity as f64).ceil() as usize;
        self.capacity = newCapacity / self.typeSize;
        assert!(self.capacity >= newSize);

        let layout = Layout::array::<u8>(newCapacity).unwrap();
        unsafe {
            self.data = realloc(self.data, layout, newCapacity);
        }
    }

    fn Shrink<T>(&mut self, newSize: usize) {
        let newCapacity = self.typeSize * newSize;
        let oldCapacity = self.capacity;
        self.capacity = newCapacity / self.typeSize;
        assert!(self.capacity <= oldCapacity);

        let layout = Layout::array::<u8>(newCapacity).unwrap();
        unsafe {
            self.data = realloc(self.data, layout, newCapacity);
        }
    }

    pub fn Push<T: 'static>(&mut self, element: T) {
        assert!(self.typeId == TypeId::of::<T>());
        assert!(std::mem::size_of::<T>() == self.typeSize);
        self.TryResize::<T>(self.size + 1);

        unsafe {
            let offset = self.size * self.typeSize;
            let ptr: *mut T = self.data.add(offset).cast::<T>();
            ptr.write(element);
        }

        self.size += 1;
    }

    pub fn Pop<T: 'static>(&mut self) -> T {
        assert!(self.typeId == TypeId::of::<T>());
        assert!(std::mem::size_of::<T>() == self.typeSize);
        assert!(self.size > 0);

        let last = unsafe {
            let offset = (self.size - 1) * self.typeSize;
            let ptr = self.data.add(offset).cast::<T>();
            ptr.read()
        };

        self.TryResize::<T>(self.size - 1);
        self.size -= 1;

        last
    }

    pub fn Get<T: 'static>(&mut self, index: usize) -> &mut T {
        assert!(self.typeId == TypeId::of::<T>());
        assert!(std::mem::size_of::<T>() == self.typeSize);
        assert!(self.size > 0);

        unsafe {
            let offset = index * self.typeSize;
            let ptr = self.data.add(offset).cast::<T>();
            return &mut *ptr;
        }
    }

    pub fn GetLength(&self) -> usize {
        self.size
    }

    pub fn GetCapacity(&self) -> usize {
        self.capacity
    }

    pub fn DropElements<T: 'static>(&mut self) {
        assert!(self.typeId == TypeId::of::<T>());
        assert!(std::mem::size_of::<T>() == self.typeSize);
        for i in 0..self.size  {
            unsafe {
                let offset = i * self.typeSize;
                let ptr = self.data.add(offset).cast::<T>();
                std::mem::drop(ptr.read());
            }
        }
        
    }
}

impl Drop for OpaqueVector {
    fn drop(&mut self) {
        let layout = Layout::array::<u8>(self.capacity * self.typeSize).unwrap();
        unsafe { dealloc(self.data, layout) }
    }
}
