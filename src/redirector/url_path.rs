use std::fmt::Display;

use once_cell::sync::Lazy;
use regex::Regex;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum UrlPathError {
    /// the provided path is not a valid url path consisting of
    /// letters, digits, and dashes
    #[error("Invalid URL path: {0}")]
    InvalidPath(String),
}

#[derive(Debug, Default, PartialEq, Clone)]
pub(crate) struct UrlPath(String);

impl UrlPath {
    pub(crate) fn new(path: String) -> Result<Self, UrlPathError> {
        static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"^/?[^/;#?]+(?:/[^/;#?]+)*/?$").unwrap());

        if !RE.is_match(&path) {
            return Err(UrlPathError::InvalidPath(path.clone()));
        }

        let mut path = path;
        if !path.starts_with('/') {
            path.insert(0, '/');
        }

        if !path.ends_with('/') {
            path.push('/');
        }

        Ok(UrlPath(path))
    }

    pub(crate) fn encode_utf16(&self) -> Vec<u16> {
        self.0.encode_utf16().collect()
    }
}

impl Display for UrlPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_url_path_new_valid_basic() {
        let path = UrlPath::new("api/v1".to_string()).unwrap();
        assert_eq!(path.0, "/api/v1/");
    }

    #[test]
    fn test_url_path_new_valid_with_leading_slash() {
        let path = UrlPath::new("/api/v1".to_string()).unwrap();
        assert_eq!(path.0, "/api/v1/");
    }

    #[test]
    fn test_url_path_new_valid_with_trailing_slash() {
        let path = UrlPath::new("api/v1/".to_string()).unwrap();
        assert_eq!(path.0, "/api/v1/");
    }

    #[test]
    fn test_url_path_new_valid_with_both_slashes() {
        let path = UrlPath::new("/api/v1/".to_string()).unwrap();
        assert_eq!(path.0, "/api/v1/");
    }

    #[test]
    fn test_url_path_new_valid_complex() {
        let path = UrlPath::new("/api/v2/users/123".to_string()).unwrap();
        assert_eq!(path.0, "/api/v2/users/123/");
    }

    #[test]
    fn test_url_path_new_valid_with_dashes() {
        let path = UrlPath::new("api-v1/user-data".to_string()).unwrap();
        assert_eq!(path.0, "/api-v1/user-data/");
    }

    #[test]
    fn test_url_path_new_valid_with_numbers() {
        let path = UrlPath::new("api123/version456".to_string()).unwrap();
        assert_eq!(path.0, "/api123/version456/");
    }

    #[test]
    fn test_url_path_new_valid_single_segment() {
        let result = UrlPath::new("api".to_string());
        assert!(result.is_ok());
        let path = result.unwrap();
        assert_eq!(path.0, "/api/");
    }

    #[test]
    fn test_url_path_new_invalid_root_only() {
        let result = UrlPath::new("/".to_string());
        assert!(result.is_err());
    }

    #[test]
    fn test_url_path_new_invalid_empty() {
        let result = UrlPath::new("".to_string());
        assert!(result.is_err());
    }

    #[test]
    fn test_url_path_new_invalid_with_query() {
        let result = UrlPath::new("api/v1?param=value".to_string());
        assert!(result.is_err());
    }

    #[test]
    fn test_url_path_new_invalid_with_semicolon() {
        let result = UrlPath::new("api/v1;param=value".to_string());
        assert!(result.is_err());
    }

    #[test]
    fn test_url_path_new_invalid_double_slash() {
        let result = UrlPath::new("api//v1".to_string());
        assert!(result.is_err());
    }

    #[test]
    fn test_url_path_encode_utf16() {
        let path = UrlPath::new("api/v1".to_string()).unwrap();
        let encoded = path.encode_utf16();
        let expected: Vec<u16> = "/api/v1/".encode_utf16().collect();
        assert_eq!(encoded, expected);
    }

    #[test]
    fn test_url_path_encode_utf16_unicode() {
        let path = UrlPath::new("café/müsli".to_string()).unwrap();
        let encoded = path.encode_utf16();
        let expected: Vec<u16> = "/café/müsli/".encode_utf16().collect();
        assert_eq!(encoded, expected);
    }

    #[test]
    fn test_url_path_clone() {
        let path = UrlPath::new("api/v1".to_string()).unwrap();
        let cloned = path.clone();
        assert_eq!(path, cloned);
        assert_eq!(path.0, cloned.0);
    }

    #[test]
    fn test_url_path_partial_eq() {
        let path1 = UrlPath::new("api/v1".to_string()).unwrap();
        let path2 = UrlPath::new("/api/v1/".to_string()).unwrap();
        assert_eq!(path1, path2);
    }

    #[test]
    fn test_url_path_default() {
        let path = UrlPath::default();
        assert_eq!(path.0, "");
    }

    #[test]
    fn test_url_path_debug() {
        let path = UrlPath::new("api/v1".to_string()).unwrap();
        let debug_output = format!("{path:?}");
        assert!(debug_output.contains("UrlPath"));
        assert!(debug_output.contains("/api/v1/"));
    }

    #[test]
    fn test_url_path_error_display() {
        let error = UrlPathError::InvalidPath("invalid-path".to_string());
        let error_message = format!("{error}");
        assert_eq!(error_message, "Invalid URL path: invalid-path");
    }

    #[test]
    fn test_url_path_error_debug() {
        let error = UrlPathError::InvalidPath("invalid-path".to_string());
        let debug_output = format!("{error:?}");
        assert!(debug_output.contains("InvalidPath"));
        assert!(debug_output.contains("invalid-path"));
    }

    #[test]
    fn test_url_path_display() {
        let path = UrlPath::new("api/v1".to_string()).unwrap();
        let display_output = format!("{path}");
        assert_eq!(display_output, "/api/v1/");
    }

    #[test]
    fn test_url_path_display_complex() {
        let path = UrlPath::new("api/v2/users/123".to_string()).unwrap();
        let display_output = format!("{path}");
        assert_eq!(display_output, "/api/v2/users/123/");
    }
}
