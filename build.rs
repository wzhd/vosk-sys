use std::env;
use std::fs;
use std::path::{Path, PathBuf};

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let out_dir: PathBuf = out_dir.into();
    let kaldi_root = if let Ok(p) = env::var("KALDI_ROOT") {
        PathBuf::from(p)
    } else {
        let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
        let mut p = PathBuf::from(crate_dir);
        p.push("kaldi");
        if p.exists() {
            p.to_path_buf()
        } else {
            println!("Set environment variable KALDI_ROOT to the directory containing kaldi sources and compiled object archives.");
            std::process::exit(1);
        }
    };

    let kaldi_src = kaldi_root.join("src");
    let vosk_src = Path::new("vosk-api").join("src");
    let vosk_files = [
        "kaldi_recognizer.cc",
        "spk_model.cc",
        "model.cc",
        "vosk_api.cc",
        "language_model.cc",
    ];
    let vosk_files = vosk_files
        .iter()
        .map(|&f| vosk_src.join(f))
        .collect::<Vec<_>>();
    vosk_files.iter().for_each(|f| {
        println!("cargo:rerun-if-changed={}", f.display());
    });
    println!("cargo:rerun-if-changed=src/wrap_catching.cc");
    cc::Build::new()
        .cpp(true)
        .static_flag(true)
        .flag("-Wno-unused-result")
        .flag("-Wno-sign-compare")
        .flag("-Wno-unused-variable")
        .flag("-Wno-unused-local-typedefs")
        .flag("-Wno-pessimizing-move")
        .flag("-Wno-unused-parameter")
        .flag("-Wno-deprecated-copy")
        .flag("-Wno-extra")
        .flag("-Wno-ignored-qualifiers")
        .flag("-Wno-type-limits")
        .include(&kaldi_src)
        .include(kaldi_root.join("tools/openfst/include"))
        .files(vosk_files)
        .file("src/wrap_catching.cc")
        .include(&vosk_src)
        .compile("vosk");
    // kaldi
    let kmods = [
        "online2",
        "decoder",
        "ivector",
        "gmm",
        "nnet3",
        "tree",
        "feat",
        "lat",
        "lm",
        "hmm",
        "transform",
        "cudamatrix",
        "matrix",
        "fstext",
        "util",
        "base",
    ];
    let mut aname = String::new();
    for &m in kmods.iter() {
        aname.replace_range(.., "kaldi-");
        aname.push_str(m);
        println!("cargo:rustc-link-lib=static={}", aname);
        let mut from = kaldi_src.clone();
        from.push(m);
        aname.push_str(".a");
        from.push(&aname);
        aname.insert_str(0, "lib");
        let dst = out_dir.join(&aname);
        fs::copy(&from, dst).unwrap();
    }
    // fst
    let fst_dir = kaldi_root.join("tools/openfst/lib/");
    println!("cargo:rustc-link-search={}", fst_dir.display());
    println!("cargo:rustc-link-lib=static=fst");
    println!("cargo:rustc-link-lib=static=fstngram");
    // algebra
    println!("cargo:rustc-link-lib=lapack");
    println!("cargo:rustc-link-lib=atlas");
    println!("cargo:rustc-link-lib=blas");
    println!("cargo:rustc-link-lib=gfortran");
}
