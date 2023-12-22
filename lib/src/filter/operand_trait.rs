use std::fmt::Debug;

pub trait FilterOperand: Debug {}

impl<T: FilterOperand + ?Sized> FilterOperand for Box<T> {}
impl FilterOperand for u32 {}
impl FilterOperand for u64 {}
impl FilterOperand for bool {}
impl FilterOperand for String {}
