//! 統一的標量型別詞彙。

use std::fmt;

use serde::{Deserialize, Serialize};

/// 標量型別詞彙
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScalarType {
    Bool,
    Int16,
    Uint16,
    Int32,
    Uint32,
    Int64,
    Uint64,
    Float32,
    Float64,
    /// `string:N`，N 為 1..=256
    Text(u16),
}

/// 標量型別解析錯誤
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ScalarTypeError {
    Unknown(String),
    BadStringLength(String),
}

impl fmt::Display for ScalarTypeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ScalarTypeError::Unknown(s) => write!(
                f,
                "不認得的標量型別 `{s}`（允許：bool, int16, uint16, int32, uint32, int64, uint64, float32, float64, string:N）"
            ),
            ScalarTypeError::BadStringLength(n) => {
                write!(f, "字串長度 `{n}` 無效，必須是 1 到 256 的整數")
            }
        }
    }
}

impl std::error::Error for ScalarTypeError {}

impl ScalarType {
    pub fn parse(raw: &str) -> Result<ScalarType, ScalarTypeError> {
        match raw {
            "bool" => Ok(ScalarType::Bool),
            "int16" => Ok(ScalarType::Int16),
            "uint16" => Ok(ScalarType::Uint16),
            "int32" => Ok(ScalarType::Int32),
            "uint32" => Ok(ScalarType::Uint32),
            "int64" => Ok(ScalarType::Int64),
            "uint64" => Ok(ScalarType::Uint64),
            "float32" => Ok(ScalarType::Float32),
            "float64" => Ok(ScalarType::Float64),
            other => match other.strip_prefix("string:") {
                Some(n) => {
                    let bad = || ScalarTypeError::BadStringLength(n.to_string());
                    if n.is_empty() || !n.bytes().all(|b| b.is_ascii_digit()) {
                        return Err(bad());
                    }
                    let len: u16 = n.parse().map_err(|_| bad())?;
                    if (1..=256).contains(&len) {
                        Ok(ScalarType::Text(len))
                    } else {
                        Err(bad())
                    }
                }
                None => Err(ScalarTypeError::Unknown(other.to_string())),
            },
        }
    }

    pub fn is_numeric(&self) -> bool {
        !matches!(self, ScalarType::Bool | ScalarType::Text(_))
    }

    pub fn is_string(&self) -> bool {
        matches!(self, ScalarType::Text(_))
    }
}

impl fmt::Display for ScalarType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ScalarType::Bool => write!(f, "bool"),
            ScalarType::Int16 => write!(f, "int16"),
            ScalarType::Uint16 => write!(f, "uint16"),
            ScalarType::Int32 => write!(f, "int32"),
            ScalarType::Uint32 => write!(f, "uint32"),
            ScalarType::Int64 => write!(f, "int64"),
            ScalarType::Uint64 => write!(f, "uint64"),
            ScalarType::Float32 => write!(f, "float32"),
            ScalarType::Float64 => write!(f, "float64"),
            ScalarType::Text(n) => write!(f, "string:{n}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn string_bounds() {
        assert_eq!(ScalarType::parse("string:256"), Ok(ScalarType::Text(256)));
        assert_eq!(ScalarType::parse("string:1"), Ok(ScalarType::Text(1)));
        for bad in [
            "string:0",
            "string:257",
            "string:+4",
            "string:",
            "string:65536",
            "string:-1",
            "string: 4",
        ] {
            assert!(
                matches!(
                    ScalarType::parse(bad),
                    Err(ScalarTypeError::BadStringLength(_))
                ),
                "{bad}"
            );
        }
    }

    #[test]
    fn string_roundtrip() {
        let t = ScalarType::parse("string:16").unwrap();
        assert_eq!(t.to_string(), "string:16");
    }

    #[test]
    fn unknown() {
        for bad in ["Bool", "", "int"] {
            assert!(
                matches!(ScalarType::parse(bad), Err(ScalarTypeError::Unknown(_))),
                "{bad}"
            );
        }
    }

    #[test]
    fn vocabulary_roundtrip() {
        for s in [
            "bool", "int16", "uint16", "int32", "uint32", "int64", "uint64", "float32", "float64",
        ] {
            assert_eq!(ScalarType::parse(s).unwrap().to_string(), s);
        }
    }

    #[test]
    fn is_copy() {
        let a = ScalarType::Bool;
        let b = a;
        assert_eq!(a, b);
    }

    #[test]
    fn truth_table() {
        assert!(!ScalarType::Bool.is_numeric() && !ScalarType::Bool.is_string());
        assert!(ScalarType::Int16.is_numeric() && !ScalarType::Int16.is_string());
        assert!(ScalarType::Float64.is_numeric());
        assert!(!ScalarType::Text(4).is_numeric() && ScalarType::Text(4).is_string());
    }

    #[test]
    fn error_display() {
        assert_eq!(
            ScalarTypeError::Unknown("x".into()).to_string(),
            "不認得的標量型別 `x`（允許：bool, int16, uint16, int32, uint32, int64, uint64, float32, float64, string:N）"
        );
        assert_eq!(
            ScalarTypeError::BadStringLength("0".into()).to_string(),
            "字串長度 `0` 無效，必須是 1 到 256 的整數"
        );
    }
}
