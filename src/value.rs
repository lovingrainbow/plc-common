//! 標量預設值／範圍值。

use serde::{Deserialize, Serialize};

/// 標量預設值／範圍值
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ScalarValue {
    Bool(bool),
    Number(f64),
    Text(String),
}

impl ScalarValue {
    pub fn as_f64(&self) -> Option<f64> {
        match self {
            ScalarValue::Number(n) => Some(*n),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn yaml_variants() {
        assert_eq!(
            serde_yaml_ng::from_str::<ScalarValue>("true").unwrap(),
            ScalarValue::Bool(true)
        );
        assert_eq!(
            serde_yaml_ng::from_str::<ScalarValue>("1.5").unwrap(),
            ScalarValue::Number(1.5)
        );
        assert_eq!(
            serde_yaml_ng::from_str::<ScalarValue>("x").unwrap(),
            ScalarValue::Text("x".into())
        );
    }

    #[test]
    fn as_f64_only_number() {
        assert_eq!(ScalarValue::Number(2.0).as_f64(), Some(2.0));
        assert_eq!(ScalarValue::Bool(true).as_f64(), None);
        assert_eq!(ScalarValue::Text("1".into()).as_f64(), None);
    }
}
