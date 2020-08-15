#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Boolean(bool),
    Number(f64),
    Nil,
}

impl Value {
    pub fn is_truthy(&self) -> bool {
        match self {
            Value::Nil | Value::Boolean(false) => false,
            _ => true,
        }
    }

    pub fn is_falsey(&self) -> bool {
        !self.is_truthy()
    }
}

impl From<bool> for Value {
    fn from(value: bool) -> Self {
        Value::Boolean(value)
    }
}

impl From<f64> for Value {
    fn from(value: f64) -> Self {
        Value::Number(value)
    }
}

