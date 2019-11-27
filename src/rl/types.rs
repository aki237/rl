use crate::value;
use crate::value_types;

pub struct Wrapper<T:std::fmt::Debug> {
  value: T,
  value_type: value_types::ValueType,
}

impl<T:std::fmt::Debug> value::Value for Wrapper<T> {
  fn display(&self) -> String {
    format!("{:?}<{:?}>", self.value_type, self.value)
  }

  fn value_type(&self) -> value_types::ValueType {
    value_types::ValueType::Integer
  }
}


pub type RLInt = Wrapper<i64>;

impl RLInt {
  pub fn new() -> RLInt {
    RLInt::from(0)
  }

  pub fn from(init: i64) -> RLInt {
    Wrapper{
      value: init,
      value_type: value_types::ValueType::Integer,
    }
  }
}