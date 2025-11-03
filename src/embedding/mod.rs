use anyhow::{Context, Result};
use fastembed::{EmbeddingModel, InitOptions, TextEmbedding};
use tracing::{info, warn};

/// Embedding service using fastembed-rs
pub struct EmbeddingService {
    model: TextEmbedding,
    dimension: usize,
}

impl EmbeddingService {
    /// Create a new embedding service
    pub fn new(model_name: &str, max_length: usize) -> Result<Self> {
        info!(
            model_name = %model_name,
            max_length = max_length,
            "Initializing embedding model"
        );

        // Parse model enum from name
        let model_type = Self::parse_model_name(model_name);

        // Get model dimension first (before moving model_type)
        let dimension = match model_type {
            EmbeddingModel::AllMiniLML6V2 => 384,
            EmbeddingModel::BGESmallENV15 => 384,
            EmbeddingModel::AllMiniLML12V2 => 384,
            _ => {
                warn!("Unknown model dimension, defaulting to 384");
                384
            }
        };

        // Initialize the model
        let model = TextEmbedding::try_new(
            InitOptions::new(model_type).with_show_download_progress(true)
        )
        .context("Failed to initialize embedding model")?;

        info!(dimension = dimension, "Embedding model ready");

        Ok(Self { model, dimension })
    }

    /// Parse model name string to EmbeddingModel enum
    fn parse_model_name(name: &str) -> EmbeddingModel {
        match name.to_lowercase().as_str() {
            "sentence-transformers/all-minilm-l6-v2" | "all-minilm-l6-v2" => {
                EmbeddingModel::AllMiniLML6V2
            }
            "baai/bge-small-en-v1.5" | "bge-small-en-v1.5" => {
                EmbeddingModel::BGESmallENV15
            }
            "sentence-transformers/all-minilm-l12-v2" | "all-minilm-l12-v2" => {
                EmbeddingModel::AllMiniLML12V2
            }
            _ => {
                warn!(
                    "Unknown model '{}', defaulting to AllMiniLML6V2",
                    name
                );
                EmbeddingModel::AllMiniLML6V2
            }
        }
    }

    /// Generate embedding for a single text
    pub fn embed(&self, text: &str) -> Result<Vec<f32>> {
        let documents = vec![text];
        let embeddings = self
            .model
            .embed(documents, None)
            .context("Failed to generate embedding")?;

        embeddings
            .into_iter()
            .next()
            .context("No embedding returned")
    }

    /// Generate embeddings for multiple texts (batch)
    pub fn embed_batch(&self, texts: Vec<&str>) -> Result<Vec<Vec<f32>>> {
        self.model
            .embed(texts, None)
            .context("Failed to generate embeddings")
    }

    /// Get the embedding dimension
    pub fn dimension(&self) -> usize {
        self.dimension
    }

    /// Combine review title and body for embedding
    pub fn prepare_review_text(title: &str, body: &str) -> String {
        format!("{} {}", title, body)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore] // Run with: cargo test -- --ignored
    fn test_embedding_service() {
        let service = EmbeddingService::new(
            "sentence-transformers/all-MiniLM-L6-v2",
            512,
        )
        .unwrap();

        let text = "This is a test sentence";
        let embedding = service.embed(text).unwrap();

        assert_eq!(embedding.len(), 384);
        assert!(embedding.iter().any(|&v| v != 0.0));
    }

    #[test]
    fn test_prepare_review_text() {
        let title = "Great product";
        let body = "I love it";
        let combined = EmbeddingService::prepare_review_text(title, body);
        assert_eq!(combined, "Great product I love it");
    }
}
