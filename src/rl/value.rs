use crate::value_types;

pub trait Value {
  fn display(&self) -> String;
  fn value_type(&self) -> value_types::ValueType;
}
