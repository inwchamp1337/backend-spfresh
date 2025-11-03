fn main() {
    println!("cargo:rerun-if-changed=src/spfresh_wrapper.cpp");
    println!("cargo:rerun-if-changed=SPFresh/");

    let sptag_path = std::path::Path::new("SPFresh/SPFresh");
    let release_path = sptag_path.join("Release");
    
    // Link pre-built SPFresh libraries (use shared library to avoid SPDK dependencies)
    println!("cargo:rustc-link-search=native={}", release_path.display());
    println!("cargo:rustc-link-lib=dylib=SPTAGLib");     // Use shared library instead of static
    println!("cargo:rustc-link-lib=static=DistanceUtils");
    
    // Link system libraries
    println!("cargo:rustc-link-lib=dylib=stdc++");
    println!("cargo:rustc-link-lib=dylib=pthread");
    println!("cargo:rustc-link-lib=dylib=gomp");     // OpenMP
    
    // Compile our C++ wrapper
    cc::Build::new()
        .cpp(true)
        .file("src/spfresh_wrapper.cpp")
        .include(sptag_path)
        .include(sptag_path.join("AnnService"))
        .include(sptag_path.join("AnnService/inc"))
        .flag("-std=c++14")
        .flag("-O3")
        .flag("-fopenmp")
        .warnings(false)
        .compile("spfresh_wrapper");
}
