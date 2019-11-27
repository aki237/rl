pub enum ValueType {
    Integer,
    Float,
    String,
    List,
    Dictionary,
    Custom,
}

impl std::fmt::Debug for ValueType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ValueType::Integer => write!(f, "Integer"),
            ValueType::Float => write!(f, "Float"),
            ValueType::String => write!(f, "String"),
            ValueType::List => write!(f, "List"),
            ValueType::Dictionary => write!(f, "Dictionary"),
            ValueType::Custom => write!(f, "Custom"),
        }
    }
}
