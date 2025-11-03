use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::Path;
use tracing::{info, warn};

/// Review metadata stored in JSONL format
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReviewMetadata {
    pub review_title: String,
    pub review_body: String,
    pub product_id: String,
    pub review_rating: u8,
}

/// JSONL storage for review metadata
/// Each line corresponds to one vector in the index (line number = vector ID)
pub struct JsonlStorage {
    path: std::path::PathBuf,
}

impl JsonlStorage {
    /// Create a new JSONL storage instance
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        Self {
            path: path.as_ref().to_path_buf(),
        }
    }

    /// Initialize storage (create file if not exists)
    pub fn initialize(&self) -> Result<()> {
        if !self.path.exists() {
            if let Some(parent) = self.path.parent() {
                std::fs::create_dir_all(parent)
                    .context("Failed to create data directory")?;
            }
            File::create(&self.path)
                .context("Failed to create metadata file")?;
            info!("Created new metadata file: {:?}", self.path);
        } else {
            info!("Using existing metadata file: {:?}", self.path);
        }
        Ok(())
    }

    /// Append a review to the JSONL file
    /// Returns the line number (0-indexed) which corresponds to vector ID
    pub fn append(&self, metadata: &ReviewMetadata) -> Result<usize> {
        // Get current line count before appending
        let vector_id = self.count_lines()?;

        // Serialize to JSON
        let json = serde_json::to_string(metadata)
            .context("Failed to serialize metadata")?;

        // Append to file
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.path)
            .context("Failed to open metadata file for appending")?;

        writeln!(file, "{}", json)
            .context("Failed to write metadata to file")?;

        info!(
            vector_id = vector_id,
            product_id = %metadata.product_id,
            "Appended review metadata"
        );

        Ok(vector_id)
    }

    /// Read a review by line number (vector ID)
    pub fn read_by_id(&self, vector_id: usize) -> Result<ReviewMetadata> {
        let file = File::open(&self.path)
            .context("Failed to open metadata file")?;
        let reader = BufReader::new(file);

        // Read line at specific index
        let line = reader
            .lines()
            .nth(vector_id)
            .context(format!("Vector ID {} not found", vector_id))?
            .context("Failed to read line")?;

        serde_json::from_str(&line)
            .context("Failed to deserialize metadata")
    }

    /// Read multiple reviews by their vector IDs
    pub fn read_batch(&self, vector_ids: &[usize]) -> Result<Vec<ReviewMetadata>> {
        let file = File::open(&self.path)
            .context("Failed to open metadata file")?;
        let reader = BufReader::new(file);

        // Collect all lines
        let lines: Vec<String> = reader
            .lines()
            .collect::<std::io::Result<_>>()
            .context("Failed to read lines")?;

        // Extract requested lines
        let mut results = Vec::with_capacity(vector_ids.len());
        for &id in vector_ids {
            if id < lines.len() {
                let metadata: ReviewMetadata = serde_json::from_str(&lines[id])
                    .context(format!("Failed to parse line {}", id))?;
                results.push(metadata);
            } else {
                warn!("Vector ID {} out of bounds (total lines: {})", id, lines.len());
            }
        }

        Ok(results)
    }

    /// Count total number of lines (reviews)
    pub fn count_lines(&self) -> Result<usize> {
        if !self.path.exists() {
            return Ok(0);
        }

        let file = File::open(&self.path)
            .context("Failed to open metadata file")?;
        let reader = BufReader::new(file);
        Ok(reader.lines().count())
    }

    /// Get all reviews (for debugging/admin purposes)
    pub fn read_all(&self) -> Result<Vec<ReviewMetadata>> {
        if !self.path.exists() {
            return Ok(vec![]);
        }

        let file = File::open(&self.path)
            .context("Failed to open metadata file")?;
        let reader = BufReader::new(file);

        reader
            .lines()
            .enumerate()
            .map(|(idx, line)| {
                let line = line.context("Failed to read line")?;
                serde_json::from_str(&line)
                    .context(format!("Failed to parse line {}", idx))
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_append_and_read() {
        let temp_dir = TempDir::new().unwrap();
        let path = temp_dir.path().join("test.jsonl");
        let storage = JsonlStorage::new(&path);
        storage.initialize().unwrap();

        let review = ReviewMetadata {
            review_title: "Great product".to_string(),
            review_body: "Very satisfied".to_string(),
            product_id: "P123".to_string(),
            review_rating: 5,
        };

        let id = storage.append(&review).unwrap();
        assert_eq!(id, 0);

        let retrieved = storage.read_by_id(id).unwrap();
        assert_eq!(retrieved.review_title, review.review_title);
        assert_eq!(retrieved.product_id, review.product_id);
    }
}
