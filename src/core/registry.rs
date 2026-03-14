//! 工具注册表

use crate::models::*;
use anyhow::{bail, Context, Result};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::time::{Duration, SystemTime};

/// 默认远程注册表 URL
const DEFAULT_REGISTRY_URL: &str = "https://raw.githubusercontent.com/arden/vcm/main/registry/tools.yaml";

/// 注册表缓存有效期（小时）
const CACHE_VALIDITY_HOURS: u64 = 24;

/// 工具注册表
pub struct Registry {
    /// 工具列表
    pub tools: Vec<Tool>,
    /// ID索引
    index: HashMap<String, usize>,
    /// 标签索引
    by_tag: HashMap<String, Vec<usize>>,
}

impl Registry {
    /// 加载注册表
    pub fn load() -> Result<Self> {
        // 1. 尝试加载缓存（如果有效）
        let cache_path = Self::cache_path();
        if cache_path.exists() && Self::is_cache_valid(&cache_path) {
            if let Ok(registry) = Self::load_from_file(&cache_path) {
                return Ok(registry);
            }
        }

        // 2. 加载内置注册表
        let builtin = Self::builtin_registry()?;
        Self::load_from_str(&builtin)
    }

    /// 从远程更新注册表
    pub async fn update_from_remote(url: Option<&str>) -> Result<()> {
        let registry_url = url.unwrap_or(DEFAULT_REGISTRY_URL);
        
        println!("{} 从远程获取注册表...", console::style("📡").dim());
        println!("URL: {}", console::style(registry_url).dim());

        // 获取远程内容
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(30))
            .user_agent("vcm")
            .build()
            .context("创建 HTTP 客户端失败")?;

        let response = client
            .get(registry_url)
            .send()
            .await
            .context("请求远程注册表失败")?;

        if !response.status().is_success() {
            bail!("远程注册表返回错误: {}", response.status());
        }

        let content = response
            .text()
            .await
            .context("读取远程注册表内容失败")?;

        // 验证 YAML 格式
        let _: ToolRegistry = serde_yaml::from_str(&content)
            .context("远程注册表格式无效")?;

        // 保存到缓存
        let cache_path = Self::cache_path();
        if let Some(parent) = cache_path.parent() {
            fs::create_dir_all(parent)
                .with_context(|| format!("无法创建目录: {:?}", parent))?;
        }

        fs::write(&cache_path, &content)
            .with_context(|| format!("无法写入缓存: {:?}", cache_path))?;

        println!("{} 注册表更新完成", console::style("✓").green());

        Ok(())
    }

    /// 检查缓存是否有效
    fn is_cache_valid(path: &PathBuf) -> bool {
        let metadata = match fs::metadata(path) {
            Ok(m) => m,
            Err(_) => return false,
        };

        let modified = match metadata.modified() {
            Ok(m) => m,
            Err(_) => return false,
        };

        let elapsed = match SystemTime::now().duration_since(modified) {
            Ok(d) => d,
            Err(_) => return false,
        };

        elapsed < Duration::from_secs(CACHE_VALIDITY_HOURS * 3600)
    }

    /// 从文件加载
    fn load_from_file(path: &PathBuf) -> Result<Self> {
        let content = fs::read_to_string(path)
            .with_context(|| format!("无法读取文件: {:?}", path))?;
        Self::load_from_str(&content)
    }

    /// 从字符串加载
    fn load_from_str(content: &str) -> Result<Self> {
        let registry: ToolRegistry = serde_yaml::from_str(content)
            .with_context(|| "解析注册表失败")?;

        let mut index = HashMap::new();
        let mut by_tag = HashMap::new();

        for (i, tool) in registry.tools.iter().enumerate() {
            // 建立ID索引
            index.insert(tool.id.clone(), i);

            // 建立标签索引
            for tag in &tool.tags {
                by_tag.entry(tag.clone())
                    .or_insert_with(Vec::new)
                    .push(i);
            }
        }

        Ok(Self {
            tools: registry.tools,
            index,
            by_tag,
        })
    }

    /// 内置注册表
    fn builtin_registry() -> Result<String> {
        // 尝试从 registry/tools.yaml 读取
        let registry_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("registry")
            .join("tools.yaml");

        if registry_path.exists() {
            return fs::read_to_string(&registry_path)
                .with_context(|| format!("无法读取内置注册表: {:?}", registry_path));
        }

        // 返回最小内置数据
        Ok(Self::minimal_registry())
    }

    /// 最小注册表（当文件不存在时的后备）
    fn minimal_registry() -> String {
        r#"
version: 1
tools:
  - id: claude-code
    name: Claude Code
    vendor: Anthropic
    description: 终端AI编程助手，具备强大的推理能力
    website: https://claude.ai
    executables:
      - claude
    config_paths:
      - ~/.claude
    env_vars:
      - name: ANTHROPIC_API_KEY
        description: Anthropic API Key
        required: true
    tags:
      - ai
      - coding
      - cli
    is_cli: true
    featured: true
"#.to_string()
    }

    /// 缓存路径
    fn cache_path() -> PathBuf {
        dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("vcm")
            .join("registry.yaml")
    }

    /// 按ID查找
    pub fn find_by_id(&self, id: &str) -> Option<&Tool> {
        self.index.get(id).map(|&i| &self.tools[i])
    }

    /// 按名称模糊查找
    pub fn find_by_name(&self, name: &str) -> Vec<&Tool> {
        let name_lower = name.to_lowercase();
        self.tools.iter()
            .filter(|t| {
                t.id.to_lowercase().contains(&name_lower) ||
                t.name.to_lowercase().contains(&name_lower)
            })
            .collect()
    }

    /// 搜索
    pub fn search(&self, query: &str) -> Vec<&Tool> {
        let query = query.to_lowercase();
        self.tools.iter()
            .filter(|t| {
                t.id.to_lowercase().contains(&query) ||
                t.name.to_lowercase().contains(&query) ||
                t.description.to_lowercase().contains(&query) ||
                t.tags.iter().any(|tag| tag.to_lowercase().contains(&query))
            })
            .collect()
    }

    /// 按标签筛选
    pub fn by_tag(&self, tag: &str) -> Vec<&Tool> {
        self.by_tag.get(tag)
            .map(|indices| indices.iter().map(|&i| &self.tools[i]).collect())
            .unwrap_or_default()
    }

    /// 获取推荐工具
    pub fn featured(&self) -> Vec<&Tool> {
        self.tools.iter().filter(|t| t.featured).collect()
    }

    /// 工具数量
    pub fn len(&self) -> usize {
        self.tools.len()
    }

    /// 是否为空
    pub fn is_empty(&self) -> bool {
        self.tools.is_empty()
    }

    /// 获取缓存状态
    pub fn cache_status() -> (bool, Option<String>) {
        let cache_path = Self::cache_path();
        if !cache_path.exists() {
            return (false, None);
        }

        let metadata = match fs::metadata(&cache_path) {
            Ok(m) => m,
            Err(_) => return (false, None),
        };

        let modified = match metadata.modified() {
            Ok(m) => m,
            Err(_) => return (true, None),
        };

        let elapsed = match SystemTime::now().duration_since(modified) {
            Ok(d) => d,
            Err(_) => return (true, None),
        };

        let hours = elapsed.as_secs() / 3600;
        let minutes = (elapsed.as_secs() % 3600) / 60;

        let age = if hours > 0 {
            format!("{}小时前", hours)
        } else {
            format!("{}分钟前", minutes)
        };

        (true, Some(age))
    }
}
