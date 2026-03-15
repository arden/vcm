//! 单元测试

#[cfg(test)]
mod tests {
    use vcm::models::*;

    #[test]
    fn test_package_manager_to_string() {
        assert_eq!(PackageManager::Npm.to_string(), "npm");
        assert_eq!(PackageManager::Pip.to_string(), "pip");
        assert_eq!(PackageManager::Cargo.to_string(), "cargo");
        assert_eq!(PackageManager::Brew.to_string(), "brew");
    }

    #[test]
    fn test_platform_to_string() {
        assert_eq!(Platform::Linux.to_string(), "linux");
        assert_eq!(Platform::Macos.to_string(), "macos");
        assert_eq!(Platform::Windows.to_string(), "windows");
    }

    #[test]
    fn test_health_status() {
        assert_eq!(HealthStatus::Healthy.to_string(), "✓ 正常");
        assert_eq!(HealthStatus::Warning.to_string(), "⚠ 警告");
        assert_eq!(HealthStatus::Error.to_string(), "✗ 错误");
        assert_eq!(HealthStatus::Unknown.to_string(), "? 未知");
    }

    #[test]
    fn test_tool_serialization() {
        let tool = Tool {
            id: "test-tool".to_string(),
            name: "Test Tool".to_string(),
            vendor: "Test Vendor".to_string(),
            description: "A test tool".to_string(),
            website: Some("https://example.com".to_string()),
            repository: None,
            executables: vec!["test".to_string()],
            install_methods: vec![InstallMethod {
                manager: PackageManager::Npm,
                package: "test-tool".to_string(),
                version: None,
                platforms: None,
            }],
            config_paths: vec!["~/.test".to_string()],
            env_vars: vec![EnvVar {
                name: "TEST_API_KEY".to_string(),
                description: "Test API Key".to_string(),
                required: true,
                get_url: None,
            }],
            tags: vec!["test".to_string()],
            is_cli: true,
            is_gui: false,
            featured: false,
            pricing: None,
        };

        // 序列化
        let yaml = serde_yaml::to_string(&tool).unwrap();
        assert!(yaml.contains("test-tool"));

        // 反序列化
        let deserialized: Tool = serde_yaml::from_str(&yaml).unwrap();
        assert_eq!(deserialized.id, "test-tool");
        assert_eq!(deserialized.name, "Test Tool");
    }

    #[test]
    fn test_installed_tool() {
        let tool = InstalledTool {
            tool_id: "test".to_string(),
            tool_name: "Test Tool".to_string(),
            executable: "test".to_string(),
            version: Some("1.0.0".to_string()),
            path: "/usr/bin/test".to_string(),
            install_method: Some(PackageManager::Npm),
            config_exists: true,
            is_configured: true,
            missing_env_vars: vec![],
        };

        assert_eq!(tool.tool_id, "test");
        assert!(tool.is_configured);
    }

    #[test]
    fn test_tool_registry_serialization() {
        let yaml_content = r#"
version: 1
updated: "2025-03-14"
tools:
  - id: claude-code
    name: Claude Code
    vendor: Anthropic
    description: Test tool
    executables:
      - claude
    install_methods:
      - manager: npm
        package: "@anthropic-ai/claude-code"
    config_paths:
      - ~/.claude
    env_vars:
      - name: ANTHROPIC_API_KEY
        description: API Key
        required: true
    tags:
      - ai
    is_cli: true
"#;

        let registry: ToolRegistry = serde_yaml::from_str(yaml_content).unwrap();
        assert_eq!(registry.tools.len(), 1);
        assert_eq!(registry.tools[0].id, "claude-code");
        assert_eq!(registry.tools[0].vendor, "Anthropic");
    }
}
