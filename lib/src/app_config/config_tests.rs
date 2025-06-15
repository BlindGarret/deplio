#[cfg(test)]
mod tests {
    use super::super::config::{
        CURRENT_VERSION, DeserializationError, build_current_version_regex, deserialize_app_config,
        write_app_config_template,
    };
    use super::super::templates::APP_CONFIG_TEMPLATE;

    #[test]
    fn test_current_version_is_valid_semver() {
        // Ensure CURRENT_VERSION is a valid semantic version
        let version = semver::Version::parse(CURRENT_VERSION);
        assert!(version.is_ok(), "CURRENT_VERSION should be valid semver");
    }

    #[test]
    fn test_write_app_config_template_basic() {
        let result = write_app_config_template("test-app", "https://api.example.com", "test-owner");

        assert!(result.contains("test-app"));
        assert!(result.contains("https://api.example.com"));
        assert!(result.contains("test-owner"));
        assert!(result.contains(CURRENT_VERSION));
    }

    #[test]
    fn test_write_app_config_template_replaces_all_placeholders() {
        let result = write_app_config_template(
            "my-application",
            "https://deplio.example.org",
            "my-organization",
        );

        // Verify all placeholders are replaced
        assert!(!result.contains("{{app_name}}"));
        assert!(!result.contains("{{deplio_server}}"));
        assert!(!result.contains("{{owner}}"));
        assert!(!result.contains("{{version}}"));

        // Verify actual values are present
        assert!(result.contains("my-application"));
        assert!(result.contains("https://deplio.example.org"));
        assert!(result.contains("my-organization"));
        assert!(result.contains(CURRENT_VERSION));
    }

    #[test]
    fn test_write_app_config_template_with_empty_strings() {
        let result = write_app_config_template("", "", "");

        // Should still be valid TOML structure even with empty values
        assert!(!result.contains("{{app_name}}"));
        assert!(!result.contains("{{deplio_server}}"));
        assert!(!result.contains("{{owner}}"));
        assert!(!result.contains("{{version}}"));
        assert!(result.contains(CURRENT_VERSION));
    }

    #[test]
    fn test_write_app_config_template_with_special_characters() {
        let result = write_app_config_template(
            "app-with-dashes_and_underscores",
            "https://api.example.com/v1/endpoint?param=value&other=123",
            "owner@example.com",
        );

        assert!(result.contains("app-with-dashes_and_underscores"));
        assert!(result.contains("https://api.example.com/v1/endpoint?param=value&other=123"));
        assert!(result.contains("owner@example.com"));
    }

    #[test]
    fn test_write_app_config_template_with_unicode() {
        let result = write_app_config_template("测试应用", "https://api.例え.com", "用户名");

        assert!(result.contains("测试应用"));
        assert!(result.contains("https://api.例え.com"));
        assert!(result.contains("用户名"));
    }

    #[test]
    fn test_write_app_config_template_with_quotes_and_escapes() {
        let result = write_app_config_template(
            "app\"with'quotes",
            "https://api.example.com/path\\with\\backslashes",
            "owner\"with'mixed\"quotes",
        );

        assert!(result.contains("app\"with'quotes"));
        assert!(result.contains("https://api.example.com/path\\with\\backslashes"));
        assert!(result.contains("owner\"with'mixed\"quotes"));
    }

    #[test]
    fn test_write_app_config_template_returns_valid_toml() {
        let result = write_app_config_template("test-app", "https://api.example.com", "test-owner");

        // Try to parse the result as TOML to ensure it's valid
        let parsed = toml::from_str::<toml::Value>(&result);
        assert!(
            parsed.is_ok(),
            "Generated template should be valid TOML: {:?}",
            parsed.err()
        );
    }

    #[test]
    fn test_write_app_config_template_structure() {
        let result = write_app_config_template("test-app", "https://api.example.com", "test-owner");

        // Parse as TOML and verify structure
        let parsed: toml::Value = toml::from_str(&result).expect("Should be valid TOML");

        // Check that the template contains the version somewhere in the structure
        // It might be at the top level or nested in a section
        let result_contains_version = result.contains(CURRENT_VERSION);
        assert!(
            result_contains_version,
            "Template should contain the current version"
        );

        // Verify the parsed TOML has some structure (not empty)
        assert!(
            !parsed.as_table().unwrap().is_empty(),
            "Parsed TOML should not be empty"
        );
    }

    #[test]
    fn test_write_app_config_template_idempotent() {
        let app_name = "test-app";
        let server = "https://api.example.com";
        let owner = "test-owner";

        let result1 = write_app_config_template(app_name, server, owner);
        let result2 = write_app_config_template(app_name, server, owner);

        assert_eq!(result1, result2, "Function should be idempotent");
    }

    #[test]
    fn test_write_app_config_template_different_inputs_different_outputs() {
        let result1 = write_app_config_template("app1", "server1", "owner1");
        let result2 = write_app_config_template("app2", "server2", "owner2");

        assert_ne!(
            result1, result2,
            "Different inputs should produce different outputs"
        );
    }

    #[test]
    fn test_write_app_config_template_preserves_template_structure() {
        let result = write_app_config_template("test-app", "https://api.example.com", "test-owner");

        // The result should still look like a configuration file
        // Check for common TOML patterns
        assert!(
            result.contains("="),
            "Should contain TOML key-value assignments"
        );
        assert!(result.contains("["), "Should contain TOML section headers");
    }

    #[test]
    fn test_write_app_config_template_with_very_long_strings() {
        let long_app_name = "a".repeat(1000);
        let long_server = format!("https://{}.example.com", "b".repeat(500));
        let long_owner = "c".repeat(800);

        let result = write_app_config_template(&long_app_name, &long_server, &long_owner);

        assert!(result.contains(&long_app_name));
        assert!(result.contains(&long_server));
        assert!(result.contains(&long_owner));
    }

    #[test]
    fn test_write_app_config_template_with_newlines() {
        let result = write_app_config_template(
            "app\nwith\nnewlines",
            "https://api.example.com\nwith\nnewlines",
            "owner\nwith\nnewlines",
        );

        assert!(result.contains("app\nwith\nnewlines"));
        assert!(result.contains("https://api.example.com\nwith\nnewlines"));
        assert!(result.contains("owner\nwith\nnewlines"));
    }

    #[test]
    fn test_current_version_format() {
        // Test that CURRENT_VERSION follows expected format
        assert!(
            !CURRENT_VERSION.is_empty(),
            "CURRENT_VERSION should not be empty"
        );
        assert!(
            !CURRENT_VERSION.starts_with('v'),
            "CURRENT_VERSION should not start with 'v'"
        );
        assert!(
            CURRENT_VERSION.contains('.'),
            "CURRENT_VERSION should contain dots"
        );

        // Should be parseable as semver
        let version = semver::Version::parse(CURRENT_VERSION).expect("Should be valid semver");
        assert!(version.major >= 1, "Should be at least version 1.x.x");
    }

    #[test]
    fn test_deserialize_app_config_valid_config() {
        let valid_config =
            write_app_config_template("test-app", "https://api.example.com", "test-owner");
        let result = deserialize_app_config(&valid_config);

        assert!(result.is_ok());
        let config = result.unwrap();
        assert_eq!(config.app.name, "test-app");
        assert_eq!(config.server.deplio_server, "https://api.example.com");
        assert_eq!(config.server.owner, "test-owner");
    }

    #[test]
    fn test_deserialize_app_config_version_mismatch() {
        let config_with_wrong_version = r#"
[deplio]
version = "0.9.0"

[server]
deplio_server = "https://api.example.com"
owner = "test-owner"

[app]
name = "test-app"
charts = []
sdlc = "dev"
"#;

        let result = deserialize_app_config(config_with_wrong_version);
        assert!(result.is_err());

        match result.unwrap_err() {
            DeserializationError::VersionMismatch(version) => {
                assert_eq!(version, CURRENT_VERSION);
            }
            _ => panic!("Expected VersionMismatch error"),
        }
    }

    #[test]
    fn test_deserialize_app_config_invalid_toml() {
        let invalid_toml = r#"
[deplio]
version = "1.0.0"
invalid toml structure
"#;

        let result = deserialize_app_config(invalid_toml);
        assert!(result.is_err());

        match result.unwrap_err() {
            DeserializationError::TomlError(_) => {
                // Expected error type
            }
            _ => panic!("Expected TomlError"),
        }
    }

    #[test]
    fn test_deserialize_app_config_missing_version() {
        let config_without_version = r#"
[server]
deplio_server = "https://api.example.com"
owner = "test-owner"

[app]
name = "test-app"
charts = []
sdlc = "dev"
"#;

        let result = deserialize_app_config(config_without_version);
        assert!(result.is_err());

        match result.unwrap_err() {
            DeserializationError::VersionMismatch(_) => {
                // Expected error type
            }
            _ => panic!("Expected VersionMismatch error"),
        }
    }

    #[test]
    fn test_deserialize_app_config_empty_string() {
        let result = deserialize_app_config("");
        assert!(result.is_err());

        match result.unwrap_err() {
            DeserializationError::VersionMismatch(_) => {
                // Expected error type
            }
            _ => panic!("Expected VersionMismatch error"),
        }
    }

    #[test]
    fn test_build_current_version_regex_basic() {
        let regex = build_current_version_regex();

        // Test that the regex is created successfully and contains escaped version
        let escaped_version = regex::escape(CURRENT_VERSION);
        assert!(regex.as_str().contains(&escaped_version));
    }

    #[test]
    fn test_build_current_version_regex_matches_valid_version() {
        let regex = build_current_version_regex();
        let test_config = format!(r#"version = "{}""#, CURRENT_VERSION);

        let captures = regex.captures(&test_config);
        assert!(
            captures.is_some(),
            "Regex should match valid version format"
        );
    }

    #[test]
    fn test_build_current_version_regex_no_match_wrong_version() {
        let regex = build_current_version_regex();
        let test_config = r#"version = "0.9.0""#;

        let captures = regex.captures(test_config);
        assert!(captures.is_none(), "Regex should not match wrong version");
    }

    #[test]
    fn test_build_current_version_regex_no_match_malformed() {
        let regex = build_current_version_regex();
        let test_cases = vec![
            "version = 1.0.0",   // Missing quotes
            "ver = \"1.0.0\"",   // Wrong key name
            "version = \"1.0.0", // Missing closing quote
        ];

        for test_case in test_cases {
            let captures = regex.captures(test_case);
            assert!(
                captures.is_none(),
                "Regex should not match malformed version: {}",
                test_case
            );
        }
    }

    #[test]
    fn test_build_current_version_regex_with_whitespace() {
        let regex = build_current_version_regex();
        let test_cases = vec![
            format!(r#"  version  =  "{}"  "#, CURRENT_VERSION),
            format!(r#"version= "{}""#, CURRENT_VERSION),
            format!(r#"version ="{}""#, CURRENT_VERSION),
        ];

        for test_case in test_cases {
            let captures = regex.captures(&test_case);
            assert!(
                captures.is_some(),
                "Regex should handle whitespace variations: {}",
                test_case
            );
        }
    }

    #[test]
    fn test_deserialize_app_config_with_template() {
        // Use the actual template to ensure it works with real template structure
        let template_config = APP_CONFIG_TEMPLATE
            .replace("{{app_name}}", "template-test")
            .replace("{{deplio_server}}", "https://template.example.com")
            .replace("{{owner}}", "template-owner")
            .replace("{{version}}", CURRENT_VERSION);

        let result = deserialize_app_config(&template_config);
        assert!(
            result.is_ok(),
            "Should successfully deserialize template-based config"
        );

        let config = result.unwrap();
        assert_eq!(config.app.name, "template-test");
        assert_eq!(config.server.deplio_server, "https://template.example.com");
        assert_eq!(config.server.owner, "template-owner");
        assert_eq!(config.deplio.version, CURRENT_VERSION);
    }
}
