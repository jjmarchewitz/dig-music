use chrono::prelude::*;
use std::fmt::Debug;

pub trait FilterOperand: Debug {}

impl<T: FilterOperand + ?Sized> FilterOperand for Box<T> {}
impl FilterOperand for u32 {}
impl FilterOperand for u64 {}
impl FilterOperand for bool {}
impl FilterOperand for String {}
impl FilterOperand for NaiveDate {}
impl FilterOperand for NaiveDateTime {}
impl FilterOperand for NaiveTime {}

pub enum FilterOperand {
    U32(u32),
    U64(u64),
    Bool(bool),
    String(String),
    NaiveDate(NaiveDate),
    NaiveDateTime(NaiveDateTime),
    NaiveTime(NaiveTime),
}
