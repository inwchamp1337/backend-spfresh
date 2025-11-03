# Architecture Documentation

## System Overview

Vector Search API for product reviews using semantic similarity.

```
┌─────────────────────────────────────────────────────────────────┐
│                      Client Applications                        │
│                     (curl, Web Apps, etc.)                      │
└────────────────────────────┬────────────────────────────────────┘
                             │ HTTP REST API
                             ▼
┌─────────────────────────────────────────────────────────────────┐
│                    Axum Web Server (Rust)                       │
├─────────────────────────────────────────────────────────────────┤
│  Endpoints:                                                     │
│    • GET  /health                                               │
│    • POST /reviews/add                                          │
│    • POST /reviews/search                                       │
└────────┬──────────────────────────────────┬─────────────────────┘
         │                                  │
         ▼                                  ▼
┌──────────────────────┐          ┌──────────────────────┐
│  Embedding Service   │          │  Storage Layer       │
│  (fastembed-rs)      │          │                      │
├──────────────────────┤          ├──────────────────────┤
│ • Model: MiniLM-L6   │          │ • VectorIndex        │
│ • Dimension: 384     │          │   (SPFresh FFI)      │
│ • Text → Vector      │          │ • JsonlStorage       │
└──────────────────────┘          │   (Metadata)         │
                                  └──────────┬───────────┘
                                             │
                                             ▼
                                  ┌─────────────────────┐
                                  │   File System       │
                                  ├─────────────────────┤
                                  │ data/reviews.index  │
                                  │ data/reviews.jsonl  │
                                  └─────────────────────┘
```

---

## Component Details

### 1. Web Server (Axum)

**Location:** `src/main.rs`, `src/api/`

**Responsibilities:**
- HTTP request handling
- Request validation
- Response serialization
- Logging and tracing
- Graceful shutdown

**Key Features:**
- Async/await with Tokio runtime
- JSON request/response
- HTTP tracing middleware
- CORS support (configurable)

---

### 2. Embedding Service

**Location:** `src/embedding/mod.rs`

**Responsibilities:**
- Load embedding model
- Generate vector embeddings from text
- Batch embedding support

**Model Details:**
- **Default Model**: `sentence-transformers/all-MiniLM-L6-v2`
- **Dimension**: 384
- **Max Length**: 512 tokens
- **Input**: Review title + body (concatenated)
- **Output**: Float32 vector

**Text Preparation:**
```rust
fn prepare_review_text(title: &str, body: &str) -> String {
    format!("{} {}", title, body)
}
```

---

### 3. Vector Index (SPFresh)

**Location:** `src/storage/spfresh.rs`

**Responsibilities:**
- Store vector embeddings
- Perform k-NN search
- Persist index to disk

**Configuration:**
- **Index Type**: BKT (default) or KDT
- **Number of Trees**: 10 (configurable)
- **Metric**: Cosine similarity (via distance)

**Current Status:** ⚠️ **Stub Implementation**
- Rust interface defined
- C++ FFI binding needed
- See TODOs in `spfresh.rs`

**Required C++ Functions:**
```cpp
void* spfresh_create_index(const char* type, int dim, int trees);
int spfresh_add_vector(void* index, const float* vector, int dim);
int spfresh_search(void* index, const float* query, int dim, int k, 
                   int* result_ids, float* result_distances);
void spfresh_save_index(void* index, const char* path);
void* spfresh_load_index(const char* path);
void spfresh_destroy_index(void* index);
```

---

### 4. Metadata Storage (JSONL)

**Location:** `src/storage/jsonl.rs`

**Responsibilities:**
- Store review metadata
- Map vector IDs to metadata
- Efficient line-by-line reading

**Format:**
```jsonl
{"review_title":"Great phone","review_body":"Battery lasts long","product_id":"P123","review_rating":5}
{"review_title":"Decent phone","review_body":"Camera is okay","product_id":"P456","review_rating":3}
```

**Key Operations:**
- `append()` - Add new review (O(1))
- `read_by_id()` - Read by line number (O(n))
- `read_batch()` - Read multiple lines (O(n))

**ID Mapping:**
```
Line Number (0-indexed) = Vector ID in index
```

---

## Data Flow Diagrams

### Add Review

```
┌──────────┐
│  Client  │
└─────┬────┘
      │ POST /reviews/add
      │ { title, body, product_id, rating }
      ▼
┌─────────────────┐
│  Axum Handler   │
└────┬───────┬────┘
     │       │
     │       └──────────────────────┐
     │                              │
     ▼                              ▼
┌──────────────┐          ┌─────────────────┐
│  Embedding   │          │  Validation     │
│  Service     │          └─────────────────┘
└──────┬───────┘
       │ vector (384-dim)
       ▼
┌──────────────────┐
│  Vector Index    │────── append ──────┐
│  (SPFresh)       │                    │
└──────┬───────────┘                    │
       │ vector_id                      │
       │                                ▼
       │                     ┌──────────────────┐
       │                     │  JSONL Storage   │
       │                     └────────┬─────────┘
       │                              │ line_id
       │                              │
       └──────── verify IDs ──────────┘
                     │
                     ▼
              ┌─────────────┐
              │  Response   │
              │  vector_id  │
              └─────────────┘
```

### Search Reviews

```
┌──────────┐
│  Client  │
└─────┬────┘
      │ POST /reviews/search
      │ { query, top_k }
      ▼
┌─────────────────┐
│  Axum Handler   │
└────┬────────────┘
     │
     ▼
┌──────────────┐
│  Embedding   │
│  Service     │
└──────┬───────┘
       │ query_vector (384-dim)
       ▼
┌──────────────────┐
│  Vector Index    │
│  (SPFresh)       │
└──────┬───────────┘
       │ [(vector_id, distance), ...]
       ▼
┌──────────────────┐
│  JSONL Storage   │
│  read_batch()    │
└──────┬───────────┘
       │ [metadata, ...]
       ▼
┌─────────────────┐
│  Combine        │
│  Results        │
└────┬────────────┘
     │
     ▼
┌─────────────────┐
│  Response       │
│  [results]      │
└─────────────────┘
```

---

## File Structure

```
/home/csy/lastz/rust/
├── Cargo.toml              # Dependencies
├── src/
│   ├── main.rs            # Server entry point
│   ├── config.rs          # Configuration
│   ├── api/
│   │   ├── mod.rs
│   │   ├── models.rs      # Request/Response DTOs
│   │   └── handlers.rs    # Endpoint handlers
│   ├── embedding/
│   │   └── mod.rs         # Embedding service
│   └── storage/
│       ├── mod.rs
│       ├── jsonl.rs       # Metadata storage
│       └── spfresh.rs     # Vector index (stub)
├── data/                  # Runtime data (gitignored)
│   ├── reviews.index      # Binary vector index
│   └── reviews.jsonl      # Metadata (JSON Lines)
└── docs/
    ├── API.md             # API documentation
    └── ARCHITECTURE.md    # This file
```

---

## Configuration

**Location:** `src/config.rs`

```rust
AppConfig {
    server: {
        host: "127.0.0.1",
        port: 3000
    },
    index: {
        index_type: "BKT",      // or "KDT"
        vector_dim: 384,
        num_trees: 10
    },
    embedding: {
        model_name: "sentence-transformers/all-MiniLM-L6-v2",
        max_length: 512
    },
    storage: {
        data_dir: "data",
        index_path: "data/reviews.index",
        metadata_path: "data/reviews.jsonl"
    }
}
```

---

## Logging

Uses `tracing` crate for structured logging:

```
2024-11-03T12:00:00 INFO  🚀 Starting Vector Search API Server
2024-11-03T12:00:01 INFO  📋 Configuration loaded
2024-11-03T12:00:02 INFO  🧠 Initializing embedding model...
2024-11-03T12:00:05 INFO  ✅ Embedding model ready (dim: 384)
2024-11-03T12:00:05 INFO  💾 Initializing metadata storage...
2024-11-03T12:00:05 INFO  ✅ Metadata storage ready (0 reviews)
2024-11-03T12:00:05 INFO  🔍 Initializing vector index...
2024-11-03T12:00:05 INFO  ✅ Vector index ready
2024-11-03T12:00:05 INFO  🌐 Server listening on http://127.0.0.1:3000
```

---

## Performance Considerations

### Scalability

- **Current Design**: Single-threaded vector operations (RwLock)
- **Bottleneck**: Embedding generation (~50-100ms per text)
- **Optimization**: Batch embedding for multiple reviews

### Storage

- **JSONL Reading**: O(n) for line access
- **Optimization Ideas**:
  - Use `memmap2` for large files
  - Build line offset index
  - Cache frequently accessed reviews

### Search

- **k-NN Complexity**: Depends on SPFresh implementation
- **Expected**: Sub-linear with BKT/KDT trees
- **Response Time**: Target <100ms for k=10

---

## TODO: SPFresh Integration

### Phase 1: C++ Wrapper
1. Create `src/spfresh_wrapper.cpp`
2. Implement C-compatible functions
3. Link against SPFresh library

### Phase 2: Build Configuration
1. Create `build.rs`
2. Use `cc` crate to compile wrapper
3. Link SPFresh static/shared library

### Phase 3: Rust FFI
1. Define `extern "C"` bindings
2. Implement safe Rust wrappers
3. Handle memory safety (pointers, lifetimes)

### Phase 4: Testing
1. Unit tests for FFI layer
2. Integration tests for index operations
3. Benchmark search performance

---

## Security Considerations

- **Input Validation**: All requests validated before processing
- **File Access**: Limited to data/ directory
- **Resource Limits**: top_k capped at 100
- **Error Handling**: Internal errors don't leak sensitive info

---

## Future Enhancements

1. **Filtering**: Search by product_id, rating range
2. **Pagination**: For large result sets
3. **Batch Operations**: Add multiple reviews at once
4. **Index Rebuild**: Periodic optimization
5. **Monitoring**: Metrics (Prometheus)
6. **Rate Limiting**: Prevent abuse
7. **Authentication**: API keys
8. **Backup/Restore**: Data persistence strategy
