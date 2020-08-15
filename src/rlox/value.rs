#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Boolean(bool),
    Number(f64),
    Nil,
    Obj(Obj),
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

impl From<&str> for Value {
    fn from(value: &str) -> Self {
        Value::Obj(Obj {
            value: ObjValue::String(String::from(value)),
        })
    }
}

impl From<String> for Value {
    fn from(value: String) -> Self {
        Value::Obj(Obj {
            value: ObjValue::String(value),
        })
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum ObjValue {
    String(String),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Obj {
    pub value: ObjValue,
}
