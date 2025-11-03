# Vector Search API Documentation

## Overview

REST API for semantic search on product reviews using vector embeddings.

**Tech Stack:**
- **Backend**: Rust (axum framework)
- **Embedding**: fastembed-rs (all-MiniLM-L6-v2)
- **Vector Index**: SPFresh (BKT/KDT, append-only)
- **Storage**: Local files (no database)

## Base URL

```
http://localhost:3000
```

## Endpoints

### 1. Health Check

Check server status and get statistics.

**Endpoint:** `GET /health`

**Response:**
```json
{
  "status": "healthy",
  "version": "0.1.0",
  "total_reviews": 42
}
```

**Example:**
```bash
curl http://localhost:3000/health
```

---

### 2. Add Review

Add a new product review to the index.

**Endpoint:** `POST /reviews/add`

**Request Body:**
```json
{
  "review_title": "Great phone",
  "review_body": "Battery lasts long and camera is excellent",
  "product_id": "P123",
  "review_rating": 5
}
```

**Field Validation:**
- `review_title`: Required, non-empty string
- `review_body`: Required, non-empty string
- `product_id`: Required, non-empty string
- `review_rating`: Required, integer 1-5

**Response:**
```json
{
  "vector_id": 42,
  "status": "success",
  "message": "Review added with ID 42"
}
```

**Example:**
```bash
curl -X POST http://localhost:3000/reviews/add \
  -H "Content-Type: application/json" \
  -d '{
    "review_title": "Amazing quality",
    "review_body": "Exceeded my expectations. Highly recommend!",
    "product_id": "P456",
    "review_rating": 5
  }'
```

---

### 3. Search Reviews

Search for reviews similar to a query using semantic similarity.

**Endpoint:** `POST /reviews/search`

**Request Body:**
```json
{
  "query": "good battery life",
  "top_k": 10
}
```

**Field Validation:**
- `query`: Required, non-empty string
- `top_k`: Optional (default: 10), integer 1-100

**Response:**
```json
{
  "query": "good battery life",
  "total_found": 3,
  "results": [
    {
      "review_title": "Great phone",
      "review_body": "Battery lasts long and camera is excellent",
      "product_id": "P123",
      "review_rating": 5,
      "similarity_score": 0.87,
      "vector_id": 42
    },
    {
      "review_title": "Decent phone",
      "review_body": "Battery is okay, could be better",
      "product_id": "P789",
      "review_rating": 3,
      "similarity_score": 0.72,
      "vector_id": 15
    }
  ]
}
```

**Example:**
```bash
curl -X POST http://localhost:3000/reviews/search \
  -H "Content-Type: application/json" \
  -d '{
    "query": "excellent camera quality",
    "top_k": 5
  }'
```

---

## Error Responses

All endpoints may return error responses:

**400 Bad Request:**
```json
{
  "error": "400 Bad Request",
  "message": "Review title cannot be empty"
}
```

**500 Internal Server Error:**
```json
{
  "error": "500 Internal Server Error",
  "message": "Failed to generate embedding: ..."
}
```

---

## Data Flow

### Add Review Flow

```
1. Client sends review data
2. Server combines title + body → text
3. Generate embedding vector (384-dim)
4. Append vector → data/reviews.index
5. Append metadata → data/reviews.jsonl
6. Return vector_id
```

### Search Flow

```
1. Client sends query text
2. Generate query embedding (384-dim)
3. Search k-NN in vector index
4. Retrieve metadata by vector IDs
5. Return ranked results with similarity scores
```

---

## Configuration

Default configuration (can be customized):

```rust
Server: 127.0.0.1:3000
Index Type: BKT
Vector Dimension: 384
Embedding Model: all-MiniLM-L6-v2
Data Directory: ./data/
```

---

## Notes

- **Append-Only**: No update/delete operations supported
- **No Database**: All data stored in local files
- **File Structure**:
  - `data/reviews.index` - Binary vector index (SPFresh)
  - `data/reviews.jsonl` - JSON Lines metadata (1 review per line)
- **Vector ID Mapping**: Line number in JSONL = Vector ID in index
