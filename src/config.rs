use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use anyhow::Context;

/// Application configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    /// Server configuration
    pub server: ServerConfig,
    
    /// Vector index configuration
    pub index: IndexConfig,
    
    /// Embedding model configuration
    pub embedding: EmbeddingConfig,
    
    /// Storage paths
    pub storage: StorageConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    /// Server host address
    #[serde(default = "default_host")]
    pub host: String,
    
    /// Server port
    #[serde(default = "default_port")]
    pub port: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexConfig {
    /// Index type: "BKT" (default) or "KDT"
    #[serde(default = "default_index_type")]
    pub index_type: String,
    
    /// Vector dimension (must match embedding model)
    #[serde(default = "default_vector_dim")]
    pub vector_dim: usize,
    
    /// Number of trees (for BKT/KDT)
    #[serde(default = "default_num_trees")]
    pub num_trees: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmbeddingConfig {
    /// Model name (e.g., "all-MiniLM-L6-v2")
    #[serde(default = "default_model_name")]
    pub model_name: String,
    
    /// Maximum sequence length
    #[serde(default = "default_max_length")]
    pub max_length: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageConfig {
    /// Data directory
    #[serde(default = "default_data_dir")]
    pub data_dir: PathBuf,
    
    /// Vector index file path
    #[serde(default = "default_index_path")]
    pub index_path: PathBuf,
    
    /// Metadata JSONL file path
    #[serde(default = "default_metadata_path")]
    pub metadata_path: PathBuf,
}

// Default values
fn default_host() -> String {
    "127.0.0.1".to_string()
}

fn default_port() -> u16 {
    3000
}

fn default_index_type() -> String {
    "BKT".to_string()
}

fn default_vector_dim() -> usize {
    384 // all-MiniLM-L6-v2 dimension
}

fn default_num_trees() -> usize {
    10
}

fn default_model_name() -> String {
    "sentence-transformers/all-MiniLM-L6-v2".to_string()
}

fn default_max_length() -> usize {
    512
}

fn default_data_dir() -> PathBuf {
    PathBuf::from("data")
}

fn default_index_path() -> PathBuf {
    PathBuf::from("data/reviews.index")
}

fn default_metadata_path() -> PathBuf {
    PathBuf::from("data/reviews.jsonl")
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            server: ServerConfig {
                host: default_host(),
                port: default_port(),
            },
            index: IndexConfig {
                index_type: default_index_type(),
                vector_dim: default_vector_dim(),
                num_trees: default_num_trees(),
            },
            embedding: EmbeddingConfig {
                model_name: default_model_name(),
                max_length: default_max_length(),
            },
            storage: StorageConfig {
                data_dir: default_data_dir(),
                index_path: default_index_path(),
                metadata_path: default_metadata_path(),
            },
        }
    }
}

impl AppConfig {
    /// Load configuration from file, or use defaults
    pub fn load() -> anyhow::Result<Self> {
        use std::env;
        use std::fs;

        // Priority:
        // 1. Path from env var VECTOR_CONFIG_PATH
        // 2. ./config.toml
        // 3. ./config.json
        // 4. defaults

        let candidates = vec![
            env::var("VECTOR_CONFIG_PATH").ok(),
            Some("config.toml".to_string()),
            Some("config.json".to_string()),
        ];

        for opt in candidates.into_iter().flatten() {
            let path = std::path::PathBuf::from(&opt);
            if path.exists() {
                let content = fs::read_to_string(&path)
                    .with_context(|| format!("Failed to read config file: {}", path.display()))?;

                // Decide parser by extension
                if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
                    match ext.to_lowercase().as_str() {
                        "toml" => {
                            let cfg: AppConfig = toml::from_str(&content)
                                .with_context(|| format!("Failed to parse TOML config: {}", path.display()))?;
                            return Ok(cfg);
                        }
                        "json" => {
                            let cfg: AppConfig = serde_json::from_str(&content)
                                .with_context(|| format!("Failed to parse JSON config: {}", path.display()))?;
                            return Ok(cfg);
                        }
                        other => {
                            // try JSON first, then TOML fallback
                            let try_json = serde_json::from_str(&content);
                            if let Ok(cfg) = try_json {
                                return Ok(cfg);
                            }
                            let try_toml = toml::from_str(&content);
                            if let Ok(cfg) = try_toml {
                                return Ok(cfg);
                            }
                            // fallthrough to next candidate
                            eprintln!("Unsupported config extension: {}\nAttempted to parse as JSON/TOML", other);
                        }
                    }
                } else {
                    // no extension: try JSON then TOML
                    if let Ok(cfg) = serde_json::from_str(&content) {
                        return Ok(cfg);
                    }
                    if let Ok(cfg) = toml::from_str(&content) {
                        return Ok(cfg);
                    }
                }
            }
        }

        // None found or parse failed -> return defaults
        Ok(Self::default())
    }
}
