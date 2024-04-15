#[derive(Debug, Clone)]
pub enum ValidationResultType {
    Ok,
    Warning,
    Error,
}

#[derive(Debug, Clone)]
pub struct ValidationResult {
    pub result: ValidationResultType,
    pub code: u32,
    pub message: Option<String>,
}

impl ValidationResult {
    pub fn ok() -> Self {
        Self {
            result: ValidationResultType::Ok,
            code: 0,
            message: None,
        }
    }

    pub fn error(code: u32, message: &str) -> Self {
        Self {
            result: ValidationResultType::Error,
            code,
            message: Some(message.to_string()),
        }
    }
    pub fn warning(code: u32, message: &str) -> Self {
        Self {
            result: ValidationResultType::Warning,
            code,
            message: Some(message.to_string()),
        }
    }
}
