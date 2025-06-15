#[cfg(test)]
mod tests {
    use super::super::traits::Upgrader;
    use super::super::versioning::upgrade_data;

    // Mock upgrader for testing
    struct MockUpgrader {
        version: &'static str,
        can_upgrade_from: &'static str,
        breaking_change: Option<&'static str>,
        upgrade_result: &'static str,
    }

    impl Upgrader for MockUpgrader {
        fn version(&self) -> &str {
            self.version
        }

        fn can_upgrade(&self, from_version: &str) -> bool {
            from_version == self.can_upgrade_from
        }

        fn upgrade(&self, data: &str) -> String {
            format!("{}-{}", data, self.upgrade_result)
        }

        fn breaking_change_message(&self) -> Option<String> {
            self.breaking_change.map(|s| s.to_string())
        }
    }

    #[test]
    fn test_upgrade_data_same_version() {
        let result = upgrade_data("1.0.0", "1.0.0", "test-data", None);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "test-data");
    }

    #[test]
    fn test_upgrade_data_invalid_from_version() {
        let result = upgrade_data("invalid-version", "1.0.0", "test-data", None);
        assert!(result.is_err());
        match result.unwrap_err() {
            super::super::versioning::UpgradeError::InvalidVersionFormat(msg) => {
                assert!(msg.contains("Invalid from_version"));
            }
            _ => panic!("Expected InvalidVersionFormat error"),
        }
    }

    #[test]
    fn test_upgrade_data_invalid_to_version() {
        let result = upgrade_data("1.0.0", "invalid-version", "test-data", None);
        assert!(result.is_err());
        match result.unwrap_err() {
            super::super::versioning::UpgradeError::InvalidVersionFormat(msg) => {
                assert!(msg.contains("Invalid to_version"));
            }
            _ => panic!("Expected InvalidVersionFormat error"),
        }
    }

    #[test]
    fn test_upgrade_data_from_version_greater_than_to_version() {
        let result = upgrade_data("2.0.0", "1.0.0", "test-data", None);
        assert!(result.is_err());
        match result.unwrap_err() {
            super::super::versioning::UpgradeError::DowngradeNotSupported { from, to } => {
                assert_eq!(from, "2.0.0");
                assert_eq!(to, "1.0.0");
            }
            _ => panic!("Expected DowngradeNotSupported error"),
        }
    }

    #[test]
    fn test_upgrade_data_unsupported_target_version() {
        let result = upgrade_data("1.0.0", "90000.0.0", "test-data", None);
        assert!(result.is_err());
        match result.unwrap_err() {
            super::super::versioning::UpgradeError::UnsupportedTargetVersion(version) => {
                assert_eq!(version, "90000.0.0");
            }
            _ => panic!("Expected UnsupportedTargetVersion error"),
        }
    }

    #[test]
    fn test_same_version_data_empty() {
        let result = upgrade_data("1.0.0", "1.0.0", "", None);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "");
    }

    #[test]
    fn test_same_version_data_whitespace() {
        let expected = "   ";
        let result = upgrade_data("1.0.0", "1.0.0", expected, None);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), expected);
    }

    #[test]
    fn test_upgrade_data_version_with_leading_v() {
        let result = upgrade_data("v1.0.0", "1.0.0", "test-data", None);
        assert!(result.is_err());
        match result.unwrap_err() {
            super::super::versioning::UpgradeError::InvalidVersionFormat(msg) => {
                assert!(msg.contains("Invalid from_version"));
            }
            _ => panic!("Expected InvalidVersionFormat error"),
        }
    }

    #[test]
    fn test_upgrade_data_version_with_spaces() {
        let result = upgrade_data(" 1.0.0 ", "1.0.0", "test-data", None);
        assert!(result.is_err());
        match result.unwrap_err() {
            super::super::versioning::UpgradeError::InvalidVersionFormat(msg) => {
                assert!(msg.contains("Invalid from_version"));
            }
            _ => panic!("Expected InvalidVersionFormat error"),
        }
    }

    #[test]
    fn test_upgrade_data_zero_versions() {
        let result = upgrade_data("0.0.0", "0.0.0", "test-data", None);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "test-data");
    }

    #[test]
    fn test_upgrade_data_large_version_numbers() {
        let result = upgrade_data("999.999.999", "999.999.999", "test-data", None);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "test-data");
    }

    #[test]
    fn test_upgrade_data_successful_single_upgrade() {
        let upgrader = MockUpgrader {
            version: "1.1.0",
            can_upgrade_from: "1.0.0",
            breaking_change: None,
            upgrade_result: "upgraded",
        };
        let upgraders: &[&dyn Upgrader] = &[&upgrader];

        let result = upgrade_data("1.0.0", "1.1.0", "test-data", Some(upgraders));
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "test-data-upgraded");
    }

    #[test]
    fn test_upgrade_data_successful_multiple_upgrades() {
        let upgrader1 = MockUpgrader {
            version: "1.1.0",
            can_upgrade_from: "1.0.0",
            breaking_change: None,
            upgrade_result: "v1.1",
        };
        let upgrader2 = MockUpgrader {
            version: "1.2.0",
            can_upgrade_from: "1.1.0",
            breaking_change: None,
            upgrade_result: "v1.2",
        };
        let upgraders: &[&dyn Upgrader] = &[&upgrader1, &upgrader2];

        let result = upgrade_data("1.0.0", "1.2.0", "data", Some(upgraders));
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "data-v1.1-v1.2");
    }

    #[test]
    fn test_upgrade_data_breaking_change_detected() {
        let upgrader = MockUpgrader {
            version: "2.0.0",
            can_upgrade_from: "1.0.0",
            breaking_change: Some("This is a breaking change"),
            upgrade_result: "upgraded",
        };
        let upgraders: &[&dyn Upgrader] = &[&upgrader];

        let result = upgrade_data("1.0.0", "2.0.0", "test-data", Some(upgraders));
        assert!(result.is_err());
        match result.unwrap_err() {
            super::super::versioning::UpgradeError::BreakingChange { from, to, message } => {
                assert_eq!(from, "1.0.0");
                assert_eq!(to, "2.0.0");
                assert_eq!(message, "This is a breaking change");
            }
            _ => panic!("Expected BreakingChange error"),
        }
    }

    #[test]
    fn test_upgrade_data_stops_at_target_version() {
        let upgrader1 = MockUpgrader {
            version: "1.1.0",
            can_upgrade_from: "1.0.0",
            breaking_change: None,
            upgrade_result: "v1.1",
        };
        let upgrader2 = MockUpgrader {
            version: "1.2.0",
            can_upgrade_from: "1.1.0",
            breaking_change: None,
            upgrade_result: "v1.2",
        };
        let upgraders: &[&dyn Upgrader] = &[&upgrader1, &upgrader2];

        // Should stop at 1.1.0 and not continue to 1.2.0
        let result = upgrade_data("1.0.0", "1.1.0", "data", Some(upgraders));
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "data-v1.1");
    }

    #[test]
    fn test_upgrade_data_no_applicable_upgrader() {
        let upgrader = MockUpgrader {
            version: "1.1.0",
            can_upgrade_from: "0.9.0", // Can't upgrade from 1.0.0
            breaking_change: None,
            upgrade_result: "upgraded",
        };
        let upgraders: &[&dyn Upgrader] = &[&upgrader];

        let result = upgrade_data("1.0.0", "1.1.0", "test-data", Some(upgraders));
        assert!(result.is_err());
        // Should fail since no upgrader can handle 1.0.0 to reach target 1.1.0
        match result.unwrap_err() {
            super::super::versioning::UpgradeError::NoRouteFound(version) => {
                assert_eq!(version, "1.1.0");
            }
            _ => panic!("Expected NoRouteFound error"),
        }
    }

    #[test]
    fn test_upgrade_data_target_version_not_in_upgraders() {
        let upgrader = MockUpgrader {
            version: "1.1.0",
            can_upgrade_from: "1.0.0",
            breaking_change: None,
            upgrade_result: "upgraded",
        };
        let upgraders: &[&dyn Upgrader] = &[&upgrader];

        // Target version 1.2.0 is not available in upgraders
        let result = upgrade_data("1.0.0", "1.2.0", "test-data", Some(upgraders));
        assert!(result.is_err());
        match result.unwrap_err() {
            super::super::versioning::UpgradeError::UnsupportedTargetVersion(version) => {
                assert_eq!(version, "1.2.0");
            }
            _ => panic!("Expected UnsupportedTargetVersion error"),
        }
    }

    #[test]
    fn test_upgrade_data_empty_upgraders_list() {
        let upgraders: &[&dyn Upgrader] = &[];

        let result = upgrade_data("1.0.0", "1.1.0", "test-data", Some(upgraders));
        assert!(result.is_err());
        match result.unwrap_err() {
            super::super::versioning::UpgradeError::UnsupportedTargetVersion(version) => {
                assert_eq!(version, "1.1.0");
            }
            _ => panic!("Expected UnsupportedTargetVersion error"),
        }
    }

    #[test]
    fn test_upgrade_data_complex_upgrade_chain() {
        let upgrader1 = MockUpgrader {
            version: "1.0.1",
            can_upgrade_from: "1.0.0",
            breaking_change: None,
            upgrade_result: "patch",
        };
        let upgrader2 = MockUpgrader {
            version: "1.1.0",
            can_upgrade_from: "1.0.1",
            breaking_change: None,
            upgrade_result: "minor",
        };
        let upgrader3 = MockUpgrader {
            version: "2.0.0",
            can_upgrade_from: "1.1.0",
            breaking_change: None,
            upgrade_result: "major",
        };
        let upgraders: &[&dyn Upgrader] = &[&upgrader1, &upgrader2, &upgrader3];

        let result = upgrade_data("1.0.0", "2.0.0", "base", Some(upgraders));
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "base-patch-minor-major");
    }

    #[test]
    fn test_upgrade_data_with_multiline_data() {
        let upgrader = MockUpgrader {
            version: "1.1.0",
            can_upgrade_from: "1.0.0",
            breaking_change: None,
            upgrade_result: "newline",
        };
        let upgraders: &[&dyn Upgrader] = &[&upgrader];

        let multiline_data = "line1\nline2\nline3";
        let result = upgrade_data("1.0.0", "1.1.0", multiline_data, Some(upgraders));
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "line1\nline2\nline3-newline");
    }

    #[test]
    fn test_upgrade_data_with_unicode_data() {
        let upgrader = MockUpgrader {
            version: "1.1.0",
            can_upgrade_from: "1.0.0",
            breaking_change: None,
            upgrade_result: "🚀",
        };
        let upgraders: &[&dyn Upgrader] = &[&upgrader];

        let unicode_data = "测试数据 émojis";
        let result = upgrade_data("1.0.0", "1.1.0", unicode_data, Some(upgraders));
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "测试数据 émojis-🚀");
    }

    #[test]
    fn test_upgrade_data_with_json_like_data() {
        let upgrader = MockUpgrader {
            version: "1.1.0",
            can_upgrade_from: "1.0.0",
            breaking_change: None,
            upgrade_result: "updated",
        };
        let upgraders: &[&dyn Upgrader] = &[&upgrader];

        let json_data = r#"{"key": "value", "number": 42}"#;
        let result = upgrade_data("1.0.0", "1.1.0", json_data, Some(upgraders));
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), r#"{"key": "value", "number": 42}-updated"#);
    }

    #[test]
    fn test_upgrade_data_prerelease_versions() {
        let upgrader = MockUpgrader {
            version: "1.1.0-beta",
            can_upgrade_from: "1.0.0-alpha",
            breaking_change: None,
            upgrade_result: "beta",
        };
        let upgraders: &[&dyn Upgrader] = &[&upgrader];

        let result = upgrade_data("1.0.0-alpha", "1.1.0-beta", "test-data", Some(upgraders));
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "test-data-beta");
    }

    #[test]
    fn test_upgrade_data_build_metadata_ignored() {
        let upgrader = MockUpgrader {
            version: "1.1.0+build2",
            can_upgrade_from: "1.0.0+build1",
            breaking_change: None,
            upgrade_result: "build",
        };
        let upgraders: &[&dyn Upgrader] = &[&upgrader];

        let result = upgrade_data("1.0.0+build1", "1.1.0+build2", "test-data", Some(upgraders));
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "test-data-build");
    }

    #[test]
    fn test_upgrade_data_incomplete_upgrade_chain() {
        let upgrader1 = MockUpgrader {
            version: "1.1.0",
            can_upgrade_from: "1.0.0",
            breaking_change: None,
            upgrade_result: "v1.1",
        };
        let upgrader2 = MockUpgrader {
            version: "1.3.0",
            can_upgrade_from: "1.2.0", // Gap: can't upgrade from 1.1.0
            breaking_change: None,
            upgrade_result: "v1.3",
        };
        let upgraders: &[&dyn Upgrader] = &[&upgrader1, &upgrader2];

        // Should fail because there's no upgrade path from 1.1.0 to 1.3.0
        let result = upgrade_data("1.0.0", "1.3.0", "data", Some(upgraders));
        assert!(result.is_err());
        match result.unwrap_err() {
            super::super::versioning::UpgradeError::NoRouteFound(version) => {
                assert_eq!(version, "1.3.0");
            }
            _ => panic!("Expected NoRouteFound error"),
        }
    }

    #[test]
    fn test_upgrade_data_complete_upgrade_chain_validation() {
        let upgrader1 = MockUpgrader {
            version: "1.1.0",
            can_upgrade_from: "1.0.0",
            breaking_change: None,
            upgrade_result: "v1.1",
        };
        let upgrader2 = MockUpgrader {
            version: "1.2.0",
            can_upgrade_from: "1.1.0",
            breaking_change: None,
            upgrade_result: "v1.2",
        };
        let upgrader3 = MockUpgrader {
            version: "1.3.0",
            can_upgrade_from: "1.2.0",
            breaking_change: None,
            upgrade_result: "v1.3",
        };
        let upgraders: &[&dyn Upgrader] = &[&upgrader1, &upgrader2, &upgrader3];

        // Should successfully upgrade through the complete chain
        let result = upgrade_data("1.0.0", "1.3.0", "data", Some(upgraders));
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "data-v1.1-v1.2-v1.3");
    }

    #[test]
    fn test_upgrade_data_missing_intermediate_version() {
        let upgrader1 = MockUpgrader {
            version: "1.1.0",
            can_upgrade_from: "1.0.0",
            breaking_change: None,
            upgrade_result: "v1.1",
        };
        // Missing 1.2.0 upgrader
        let upgrader3 = MockUpgrader {
            version: "1.3.0",
            can_upgrade_from: "1.2.0",
            breaking_change: None,
            upgrade_result: "v1.3",
        };
        let upgraders: &[&dyn Upgrader] = &[&upgrader1, &upgrader3];

        // Should fail because we can't get from 1.1.0 to 1.2.0
        let result = upgrade_data("1.0.0", "1.3.0", "data", Some(upgraders));
        assert!(result.is_err());
        match result.unwrap_err() {
            super::super::versioning::UpgradeError::NoRouteFound(version) => {
                assert_eq!(version, "1.3.0");
            }
            _ => panic!("Expected NoRouteFound error"),
        }
    }
}
