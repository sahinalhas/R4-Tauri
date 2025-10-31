// Security utilities for input validation and sanitization

use thiserror::Error;

#[derive(Error, Debug)]
pub enum ValidationError {
    #[error("Input too long: maximum {max} characters, got {actual}")]
    InputTooLong { max: usize, actual: usize },
    
    #[error("Input contains forbidden characters")]
    ForbiddenCharacters,
    
    #[error("Email format is invalid")]
    InvalidEmail,
    
    #[error("Invalid identifier format")]
    InvalidIdentifier,
    
    #[error("Path traversal attempt detected")]
    PathTraversal,
    
    #[error("SQL injection pattern detected")]
    SqlInjectionAttempt,
    
    #[error("XSS pattern detected")]
    XssAttempt,
}

/// Validate string length
pub fn validate_length(input: &str, max_length: usize) -> Result<(), ValidationError> {
    let len = input.len();
    if len > max_length {
        return Err(ValidationError::InputTooLong {
            max: max_length,
            actual: len,
        });
    }
    Ok(())
}

/// Validate email format
pub fn validate_email(email: &str) -> Result<(), ValidationError> {
    let email_regex = regex::Regex::new(
        r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$"
    ).unwrap();
    
    if !email_regex.is_match(email) {
        return Err(ValidationError::InvalidEmail);
    }
    
    Ok(())
}

/// Sanitize user input to prevent XSS
pub fn sanitize_html(input: &str) -> String {
    input
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#x27;")
        .replace('&', "&amp;")
}

/// Check for path traversal attempts
pub fn validate_filename(filename: &str) -> Result<(), ValidationError> {
    // Check for path traversal patterns
    if filename.contains("..") 
        || filename.contains('/') 
        || filename.contains('\\') 
        || filename.starts_with('.')
    {
        return Err(ValidationError::PathTraversal);
    }
    
    // Check for forbidden characters
    let forbidden_chars = ['<', '>', ':', '"', '|', '?', '*'];
    if filename.chars().any(|c| forbidden_chars.contains(&c)) {
        return Err(ValidationError::ForbiddenCharacters);
    }
    
    validate_length(filename, 255)?;
    
    Ok(())
}

/// Validate student number format (Turkish student ID)
pub fn validate_student_number(number: &str) -> Result<(), ValidationError> {
    // Turkish student numbers are typically numeric, 6-12 digits
    if !number.chars().all(|c| c.is_numeric()) {
        return Err(ValidationError::InvalidIdentifier);
    }
    
    let len = number.len();
    if len < 6 || len > 12 {
        return Err(ValidationError::InputTooLong { max: 12, actual: len });
    }
    
    Ok(())
}

/// Validate Turkish phone number
pub fn validate_turkish_phone(phone: &str) -> Result<(), ValidationError> {
    // Remove common separators
    let cleaned = phone.replace(' ', "")
        .replace('-', "")
        .replace('(', "")
        .replace(')', "");
    
    // Turkish phone: starts with 0 or +90, followed by 10 digits
    let phone_regex = regex::Regex::new(
        r"^(\+90|0)?[1-9][0-9]{9}$"
    ).unwrap();
    
    if !phone_regex.is_match(&cleaned) {
        return Err(ValidationError::InvalidIdentifier);
    }
    
    Ok(())
}

/// Check for potential SQL injection patterns
pub fn check_sql_injection(input: &str) -> Result<(), ValidationError> {
    let suspicious_patterns = [
        "'; DROP",
        "'; DELETE",
        "'; UPDATE",
        "'; INSERT",
        "' OR '1'='1",
        "' OR 1=1",
        "UNION SELECT",
        "exec(",
        "execute(",
    ];
    
    let uppercase_input = input.to_uppercase();
    for pattern in suspicious_patterns {
        if uppercase_input.contains(pattern) {
            return Err(ValidationError::SqlInjectionAttempt);
        }
    }
    
    Ok(())
}

/// Check for potential XSS patterns
pub fn check_xss(input: &str) -> Result<(), ValidationError> {
    let suspicious_patterns = [
        "<script",
        "javascript:",
        "onerror=",
        "onload=",
        "onclick=",
        "<iframe",
        "eval(",
    ];
    
    let lowercase_input = input.to_lowercase();
    for pattern in suspicious_patterns {
        if lowercase_input.contains(pattern) {
            return Err(ValidationError::XssAttempt);
        }
    }
    
    Ok(())
}

/// Comprehensive input validation
pub fn validate_user_input(input: &str, max_length: usize) -> Result<String, ValidationError> {
    // Check length
    validate_length(input, max_length)?;
    
    // Check for SQL injection
    check_sql_injection(input)?;
    
    // Check for XSS
    check_xss(input)?;
    
    // Sanitize and return
    Ok(sanitize_html(input))
}

/// Validate and sanitize name fields (first name, last name)
pub fn validate_name(name: &str) -> Result<String, ValidationError> {
    validate_length(name, 100)?;
    
    // Names should only contain letters, spaces, and Turkish characters
    let name_regex = regex::Regex::new(
        r"^[a-zA-ZğüşıöçĞÜŞİÖÇ\s'-]+$"
    ).unwrap();
    
    if !name_regex.is_match(name) {
        return Err(ValidationError::ForbiddenCharacters);
    }
    
    Ok(name.trim().to_string())
}

/// Validate ID (UUID format)
pub fn validate_id(id: &str) -> Result<(), ValidationError> {
    // Check if it's a valid i64 (for database IDs)
    if id.parse::<i64>().is_ok() {
        return Ok(());
    }
    
    // Or check if it's a valid UUID
    if uuid::Uuid::parse_str(id).is_ok() {
        return Ok(());
    }
    
    Err(ValidationError::InvalidIdentifier)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_length() {
        assert!(validate_length("short", 10).is_ok());
        assert!(validate_length("very long text", 5).is_err());
    }

    #[test]
    fn test_validate_email() {
        assert!(validate_email("user@example.com").is_ok());
        assert!(validate_email("invalid.email").is_err());
        assert!(validate_email("@example.com").is_err());
    }

    #[test]
    fn test_sanitize_html() {
        assert_eq!(sanitize_html("<script>alert('xss')</script>"), 
                   "&lt;script&gt;alert(&#x27;xss&#x27;)&lt;/script&gt;");
    }

    #[test]
    fn test_validate_filename() {
        assert!(validate_filename("document.pdf").is_ok());
        assert!(validate_filename("../../../etc/passwd").is_err());
        assert!(validate_filename("file<name>.txt").is_err());
    }

    #[test]
    fn test_sql_injection_detection() {
        assert!(check_sql_injection("normal text").is_ok());
        assert!(check_sql_injection("'; DROP TABLE students; --").is_err());
        assert!(check_sql_injection("' OR 1=1 --").is_err());
    }

    #[test]
    fn test_xss_detection() {
        assert!(check_xss("normal text").is_ok());
        assert!(check_xss("<script>alert('xss')</script>").is_err());
        assert!(check_xss("javascript:alert(1)").is_err());
    }
}
