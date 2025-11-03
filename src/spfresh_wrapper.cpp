#include "AnnService/inc/Core/VectorIndex.h"
#include "AnnService/inc/Core/Common.h"
#include <cstring>
#include <memory>

using namespace SPTAG;

// C-compatible wrapper for SPFresh VectorIndex
extern "C" {

// Create a new index
void* spfresh_create_index(const char* algo_type, const char* value_type, int dimension) {
    IndexAlgoType algo;
    if (strcmp(algo_type, "BKT") == 0) {
        algo = IndexAlgoType::BKT;
    } else if (strcmp(algo_type, "KDT") == 0) {
        algo = IndexAlgoType::KDT;
    } else {
        return nullptr;
    }

    VectorValueType vtype;
    if (strcmp(value_type, "Float") == 0) {
        vtype = VectorValueType::Float;
    } else {
        return nullptr;
    }

    auto index = VectorIndex::CreateInstance(algo, vtype);
    if (!index) return nullptr;

    // Return raw pointer (caller must manage lifetime)
    return new std::shared_ptr<VectorIndex>(index);
}

// Add a single vector to the index
int spfresh_add_vector(void* index_ptr, const float* vector, int dimension) {
    if (!index_ptr || !vector) return -1;

    auto index = *static_cast<std::shared_ptr<VectorIndex>*>(index_ptr);
    
    // Allocate memory for the vector
    size_t vectorSize = dimension * sizeof(float);
    float* vectorData = new float[dimension];
    memcpy(vectorData, vector, vectorSize);
    
    // Create ByteArray wrapper
    ByteArray byteArray((std::uint8_t*)vectorData, vectorSize, true);  // true = take ownership
    
    // Create VectorSet with single vector
    auto vectorSet = std::make_shared<BasicVectorSet>(
        byteArray,
        VectorValueType::Float,
        dimension,
        1  // single vector
    );

    ErrorCode ret = index->AddIndex(vectorSet, nullptr, false, false);
    
    if (ret != ErrorCode::Success) {
        return -1;
    }

    // Return the new vector ID (current count - 1)
    return index->GetNumSamples() - 1;
}

// Build index from existing vectors
int spfresh_build_index(void* index_ptr, const float* vectors, int num_vectors, int dimension) {
    if (!index_ptr || !vectors) return -1;

    auto index = *static_cast<std::shared_ptr<VectorIndex>*>(index_ptr);
    
    ErrorCode ret = index->BuildIndex(
        (const void*)vectors,
        num_vectors,
        dimension,
        false,  // not normalized
        false   // don't share ownership
    );

    return (ret == ErrorCode::Success) ? 0 : -1;
}

// Search k nearest neighbors
int spfresh_search(
    void* index_ptr,
    const float* query,
    int dimension,
    int k,
    int* result_indices,
    float* result_distances
) {
    if (!index_ptr || !query || !result_indices || !result_distances) return -1;

    auto index = *static_cast<std::shared_ptr<VectorIndex>*>(index_ptr);

    // Create query result structure
    QueryResult query_result((const void*)query, k, false);
    query_result.SetTarget(const_cast<float*>(query));

    ErrorCode ret = index->SearchIndex(query_result, false);
    
    if (ret != ErrorCode::Success) {
        return -1;
    }

    // Copy results
    int count = min(k, (int)query_result.GetResultNum());
    for (int i = 0; i < count; i++) {
        result_indices[i] = query_result.GetResult(i)->VID;
        result_distances[i] = query_result.GetResult(i)->Dist;
    }

    return count;
}

// Save index to directory
int spfresh_save_index(void* index_ptr, const char* folder_path) {
    if (!index_ptr || !folder_path) return -1;

    auto index = *static_cast<std::shared_ptr<VectorIndex>*>(index_ptr);
    
    ErrorCode ret = index->SaveIndex(std::string(folder_path));
    
    return (ret == ErrorCode::Success) ? 0 : -1;
}

// Load index from directory
void* spfresh_load_index(const char* folder_path) {
    if (!folder_path) return nullptr;

    std::shared_ptr<VectorIndex> index;
    ErrorCode ret = VectorIndex::LoadIndex(std::string(folder_path), index);
    
    if (ret != ErrorCode::Success || !index) {
        return nullptr;
    }

    return new std::shared_ptr<VectorIndex>(index);
}

// Get number of vectors in index
int spfresh_get_num_vectors(void* index_ptr) {
    if (!index_ptr) return -1;
    
    auto index = *static_cast<std::shared_ptr<VectorIndex>*>(index_ptr);
    return index->GetNumSamples();
}

// Get dimension of vectors
int spfresh_get_dimension(void* index_ptr) {
    if (!index_ptr) return -1;
    
    auto index = *static_cast<std::shared_ptr<VectorIndex>*>(index_ptr);
    return index->GetFeatureDim();
}

// Set index parameter
int spfresh_set_parameter(void* index_ptr, const char* param_name, const char* param_value) {
    if (!index_ptr || !param_name || !param_value) return -1;
    
    auto index = *static_cast<std::shared_ptr<VectorIndex>*>(index_ptr);
    ErrorCode ret = index->SetParameter(param_name, param_value);
    
    return (ret == ErrorCode::Success) ? 0 : -1;
}

// Destroy index and free memory
void spfresh_destroy_index(void* index_ptr) {
    if (index_ptr) {
        delete static_cast<std::shared_ptr<VectorIndex>*>(index_ptr);
    }
}

} // extern "C"
