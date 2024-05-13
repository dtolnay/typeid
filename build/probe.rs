// This code determines whether TypeId::of is usable in const in the current
// toolchain.

#![no_std]
#![feature(const_type_id)]

use core::any::TypeId;

pub const TYPEID: TypeId = TypeId::of::<()>();
