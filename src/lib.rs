#![no_std]
#![cfg_attr(const_type_id, feature(const_type_id))]

use core::any::TypeId;
use core::cmp::Ordering;
use core::fmt::{self, Debug};
use core::hash::{Hash, Hasher};
use core::marker::PhantomData;
use core::mem;

#[derive(Copy, Clone)]
pub struct ConstTypeId {
    #[cfg(const_type_id)]
    type_id: TypeId,
    #[cfg(not(const_type_id))]
    type_id_fn: fn() -> TypeId,
}

impl ConstTypeId {
    #[must_use]
    pub const fn of<T: ?Sized + 'static>() -> Self {
        ConstTypeId {
            #[cfg(const_type_id)]
            type_id: TypeId::of::<T>(),
            #[cfg(not(const_type_id))]
            type_id_fn: TypeId::of::<T>,
        }
    }

    #[inline]
    fn get(self) -> TypeId {
        #[cfg(const_type_id)]
        return self.type_id;
        #[cfg(not(const_type_id))]
        return (self.type_id_fn)();
    }
}

impl Debug for ConstTypeId {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        Debug::fmt(&self.get(), formatter)
    }
}

impl PartialEq for ConstTypeId {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.get() == other.get()
    }
}

impl PartialEq<TypeId> for ConstTypeId {
    fn eq(&self, other: &TypeId) -> bool {
        self.get() == *other
    }
}

impl Eq for ConstTypeId {}

impl PartialOrd for ConstTypeId {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(Ord::cmp(self, other))
    }
}

impl Ord for ConstTypeId {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        Ord::cmp(&self.get(), &other.get())
    }
}

impl Hash for ConstTypeId {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.get().hash(state);
    }
}

#[must_use]
pub fn of<T: ?Sized>() -> TypeId {
    trait NonStaticAny {
        fn get_type_id(&self) -> TypeId
        where
            Self: 'static;
    }

    impl<T: ?Sized> NonStaticAny for PhantomData<T> {
        fn get_type_id(&self) -> TypeId
        where
            Self: 'static,
        {
            TypeId::of::<T>()
        }
    }

    let phantom_data = PhantomData::<T>;
    NonStaticAny::get_type_id(unsafe {
        mem::transmute::<&dyn NonStaticAny, &(dyn NonStaticAny + 'static)>(&phantom_data)
    })
}
