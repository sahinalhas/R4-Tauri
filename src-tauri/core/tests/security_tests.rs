#[cfg(test)]
mod security_tests {
    use rehber360_core::security;

    #[test]
    fn test_validate_email_valid() {
        assert!(security::validate_email("user@example.com").is_ok());
        assert!(security::validate_email("test.user@school.edu.tr").is_ok());
        assert!(security::validate_email("admin+test@domain.com").is_ok());
    }

    #[test]
    fn test_validate_email_invalid() {
        assert!(security::validate_email("").is_err());
        assert!(security::validate_email("invalid").is_err());
        assert!(security::validate_email("@example.com").is_err());
        assert!(security::validate_email("user@").is_err());
        assert!(security::validate_email("user @example.com").is_err());
    }

    #[test]
    fn test_validate_student_number_valid() {
        assert!(security::validate_student_number("123456").is_ok());
        assert!(security::validate_student_number("123456789012").is_ok());
    }

    #[test]
    fn test_validate_student_number_invalid() {
        assert!(security::validate_student_number("12345").is_err()); // Too short
        assert!(security::validate_student_number("1234567890123").is_err()); // Too long
        assert!(security::validate_student_number("12345a").is_err()); // Contains letter
        assert!(security::validate_student_number("").is_err());
    }

    #[test]
    fn test_validate_turkish_phone_valid() {
        assert!(security::validate_turkish_phone("+905551234567").is_ok());
        assert!(security::validate_turkish_phone("05551234567").is_ok());
        assert!(security::validate_turkish_phone("+90 555 123 45 67").is_ok());
        assert!(security::validate_turkish_phone("0 555 123 45 67").is_ok());
    }

    #[test]
    fn test_validate_turkish_phone_invalid() {
        assert!(security::validate_turkish_phone("123456").is_err());
        assert!(security::validate_turkish_phone("+1234567890").is_err());
        assert!(security::validate_turkish_phone("").is_err());
    }

    #[test]
    fn test_validate_filename_safe() {
        assert!(security::validate_filename("document.pdf").is_ok());
        assert!(security::validate_filename("image_2025.jpg").is_ok());
        assert!(security::validate_filename("report-2025-01.xlsx").is_ok());
    }

    #[test]
    fn test_validate_filename_unsafe() {
        assert!(security::validate_filename("../../../etc/passwd").is_err());
        assert!(security::validate_filename("..\\windows\\system32\\file.dll").is_err());
        assert!(security::validate_filename("file<script>.pdf").is_err());
        assert!(security::validate_filename("").is_err());
    }

    #[test]
    fn test_validate_name_valid() {
        assert!(security::validate_name("Ahmet").is_ok());
        assert!(security::validate_name("Şebnem").is_ok());
        assert!(security::validate_name("Ümit Çağlar").is_ok());
        assert!(security::validate_name("İbrahim Öztürk").is_ok());
    }

    #[test]
    fn test_validate_name_invalid() {
        assert!(security::validate_name("").is_err());
        assert!(security::validate_name("A").is_err()); // Too short
        assert!(security::validate_name("123").is_err()); // Only numbers
        assert!(security::validate_name("User@Name").is_err()); // Special chars
        assert!(security::validate_name("<script>alert('xss')</script>").is_err());
    }

    #[test]
    fn test_sanitize_html() {
        let input = "<script>alert('XSS')</script>Hello World";
        let output = security::sanitize_html(input);
        assert!(!output.contains("<script>"));
        assert!(output.contains("Hello World"));
    }

    #[test]
    fn test_check_xss() {
        assert!(security::check_xss("<script>alert('xss')</script>").is_err());
        assert!(security::check_xss("javascript:alert('xss')").is_err());
        assert!(security::check_xss("<img src=x onerror=alert('xss')>").is_err());
        assert!(security::check_xss("Hello World").is_ok());
        assert!(security::check_xss("Normal text with <strong>HTML</strong>").is_ok());
    }

    #[test]
    fn test_check_sql_injection() {
        assert!(security::check_sql_injection("'; DROP TABLE users--").is_err());
        assert!(security::check_sql_injection("1' OR '1'='1").is_err());
        assert!(security::check_sql_injection("admin'--").is_err());
        assert!(security::check_sql_injection("UNION SELECT * FROM users").is_err());
        assert!(security::check_sql_injection("Normal query text").is_ok());
        assert!(security::check_sql_injection("User's name").is_ok());
    }

    #[test]
    fn test_validate_id_valid() {
        assert!(security::validate_id("550e8400-e29b-41d4-a716-446655440000").is_ok()); // UUID
        assert!(security::validate_id("12345").is_ok()); // Numeric ID
    }

    #[test]
    fn test_validate_id_invalid() {
        assert!(security::validate_id("").is_err());
        assert!(security::validate_id("not-a-valid-uuid").is_err());
        assert!(security::validate_id("<script>").is_err());
    }

    #[test]
    fn test_validate_length() {
        assert!(security::validate_length("Hello", 1, 10).is_ok());
        assert!(security::validate_length("Test", 4, 4).is_ok());
        assert!(security::validate_length("Short", 1, 100).is_ok());
        
        assert!(security::validate_length("", 1, 10).is_err()); // Too short
        assert!(security::validate_length("Too long text here", 1, 10).is_err()); // Too long
    }
}
