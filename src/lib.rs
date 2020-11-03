#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::ops::Deref;

pub struct TypeSet {
    elements: HashMap<TypeId, Box<dyn Any>>,
}

impl TypeSet {
    pub fn new() -> Self {
        Self {
            elements: HashMap::new(),
        }
    }

    pub fn insert<E: Sized + 'static>(&mut self, element: E) {
        self.elements.insert(TypeId::of::<E>(), Box::new(element));
    }

    pub fn contains<E: Sized + 'static>(&self) -> bool {
        self.elements.contains_key(&TypeId::of::<E>())
    }

    pub fn get<E: Sized + 'static>(&self) -> Option<&E> {
        unsafe { return self.get_ptr::<E>()?.as_ref() }
    }

    pub fn get_mut<E: Sized + 'static>(&mut self) -> Option<&mut E> {
        unsafe { return self.get_ptr::<E>()?.as_mut() }
    }
}

impl TypeSet {
    fn get_ptr<E: Sized + 'static>(&self) -> Option<*mut E> {
        let possibly_element = self.elements.get(&TypeId::of::<E>());
        if let Some(element) = possibly_element {
            let ptr: *mut E;
            unsafe {
                let box_ref = element.deref();
                ptr = std::mem::transmute::<&dyn Any, u128>(box_ref) as *mut E;
            }
            Some(ptr)
        } else {
            None
        }
    }
}
