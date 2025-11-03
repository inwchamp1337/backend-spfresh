# Vector Search API for Product Reviews

Semantic search API for product reviews using vector embeddings. Built with Rust, fastembed-rs, and SPFresh vector index.

## Features

- 🚀 **Fast Semantic Search**: Find similar reviews using vector embeddings
- 📝 **Append-Only Storage**: Simple, reliable data persistence
- 🧠 **Local ML Models**: No external API calls, runs entirely on your machine
- 💾 **No Database Required**: Uses local files for storage
- 🔧 **Configurable Index**: Choose between BKT or KDT index types
- 📊 **Structured Logging**: Detailed operation logs

## Tech Stack

- **Backend**: Rust (Axum web framework)
- **Embeddings**: fastembed-rs (sentence-transformers/all-MiniLM-L6-v2)
- **Vector Index**: SPFresh (BKT/KDT)
- **Storage**: JSONL (metadata) + Binary index (vectors)

## Quick Start

### Prerequisites

```bash
# Rust toolchain (1.70+)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# C++ compiler (for SPFresh integration, future)
sudo apt-get install build-essential cmake
```

### Build & Run

```bash
# Clone and navigate to project
cd /home/csy/lastz/rust

# Build
cargo build --release

# Run server
cargo run

# Server starts at http://127.0.0.1:3000
```

## API Usage

### Health Check

```bash
curl http://localhost:3000/health
```

### Add Review

```bash
curl -X POST http://localhost:3000/reviews/add \
  -H "Content-Type: application/json" \
  -d '{
    "review_title": "Excellent product",
    "review_body": "Battery lasts 2 days, camera quality is superb",
    "product_id": "P12345",
    "review_rating": 5
  }'
```

Response:
```json
{
  "vector_id": 0,
  "status": "success",
  "message": "Review added with ID 0"
}
```

### Search Reviews

```bash
curl -X POST http://localhost:3000/reviews/search \
  -H "Content-Type: application/json" \
  -d '{
    "query": "good battery and camera",
    "top_k": 5
  }'
```

Response:
```json
{
  "query": "good battery and camera",
  "total_found": 2,
  "results": [
    {
      "review_title": "Excellent product",
      "review_body": "Battery lasts 2 days, camera quality is superb",
      "product_id": "P12345",
      "review_rating": 5,
      "similarity_score": 0.89,
      "vector_id": 0
    }
  ]
}
```

## Project Structure

```
rust/
├── Cargo.toml              # Dependencies
├── src/
│   ├── main.rs            # Server entry point
│   ├── config.rs          # Configuration
│   ├── api/               # API endpoints
│   ├── embedding/         # Embedding service
│   └── storage/           # Data storage layer
├── data/                  # Runtime data (created on first run)
│   ├── reviews.index      # Vector index
│   └── reviews.jsonl      # Metadata
└── docs/
    ├── API.md             # API documentation
    ├── ARCHITECTURE.md    # System design
    └── DEV_LOG.md         # Development log
```

## Configuration

Default configuration (hardcoded, future: load from file):

```rust
Server: 127.0.0.1:3000
Index Type: BKT
Vector Dimension: 384
Embedding Model: all-MiniLM-L6-v2
Data Directory: ./data/
```

## Data Storage

### Vector Index
- **File**: `data/reviews.index`
- **Format**: Binary (SPFresh format)
- **Type**: BKT (Balanced K-means Tree) or KDT (KD-Tree)
- **Operations**: Append-only

### Metadata
- **File**: `data/reviews.jsonl`
- **Format**: JSON Lines (one review per line)
- **Mapping**: Line number (0-indexed) = Vector ID

Example JSONL:
```jsonl
{"review_title":"Great phone","review_body":"Battery lasts long","product_id":"P123","review_rating":5}
{"review_title":"Decent camera","review_body":"Photos are clear","product_id":"P456","review_rating":4}
```

## Development

### Run Tests

```bash
cargo test
```

### Build Release

```bash
cargo build --release
./target/release/vector-search-api
```

### Check Code

```bash
cargo clippy
cargo fmt
```

## Current Status

✅ **Working**:
- Configuration system
- JSONL metadata storage
- Embedding generation (fastembed-rs)
- API endpoints (Axum)
- Logging and tracing

⚠️ **Stub/TODO**:
- **SPFresh C++ FFI binding** (critical)
  - Interface defined
  - C++ wrapper needed
  - See `src/storage/spfresh.rs` for TODOs

## Next Steps

1. ✅ Initial project setup
2. ✅ Core modules implementation
3. ✅ API endpoints
4. ✅ Documentation
5. ⏳ **SPFresh C++ integration** (in progress)
6. ⏳ Integration testing
7. ⏳ Performance optimization

## Documentation

- **[API Reference](docs/API.md)**: Complete API documentation
- **[Architecture](docs/ARCHITECTURE.md)**: System design and data flows
- **[Dev Log](docs/DEV_LOG.md)**: Development progress and decisions

## Performance

Expected performance (with SPFresh):
- **Add Review**: <100ms (including embedding generation)
- **Search (k=10)**: <100ms
- **Scalability**: 100K+ reviews

Current performance (stub):
- Embedding: ~50-100ms per review
- Search: Dummy results (no real index yet)

## License

(Add your license here)

## Contributing

(Add contribution guidelines)

## Contact

(Add contact information)

---


cd /home/csy/lastz/rust && source "$HOME/.cargo/env" && LD_LIBRARY_PATH=SPFresh/SPFresh/Release:$LD_LIBRARY_PATH cargo run 2>&1