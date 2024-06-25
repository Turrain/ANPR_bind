extern crate bindgen;

use std::{env, fs};
use std::path::{Path, PathBuf};

fn main() {
    // Determine the target architecture
    let target = env::var("TARGET").unwrap();
    let is_x86_64 = target.contains("x86_64");
    // Paths to the DLLs
    let current_dir = env::current_dir().unwrap();
    println!("Current directory: {}", current_dir.display());
    let anpr_dlls = if is_x86_64 {
        vec![
            current_dir.join("dll/x64/iANPR_vc14_x64.dll"),
            current_dir.join("dll/x64/iANPRcapture_vc14_x64.dll"),
            current_dir.join("dll/x64/iANPRinterface_vc14_x64.dll"),
            current_dir.join("dll/x64/opencv_world340.dll"),
            current_dir.join("dll/x64/opencv_ffmpeg340_64.dll"),
        ]
    } else {
        vec![
            current_dir.join("dll/x86/iANPR_vc14_x86.dll"),
            current_dir.join("dll/x86/iANPRcapture_vc14_x86.dll"),
            current_dir.join("dll/x86/iANPRinterface_vc14_x86.dll"),
            current_dir.join("dll/x86/opencv_world340.dll"),
            current_dir.join("dll/x86/opencv_ffmpeg340.dll"),
        ]
    };

    // Copy the DLLs to the output directory
    let out_dir = env::var("OUT_DIR").unwrap();
    let target_dir = Path::new(&out_dir)
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join("deps");
    println!("Target directory: {}", target_dir.display());
    // Function to copy a file
   fn copy_dll(src: &Path, dest: &Path) {
        println!("Copying from {} to {}", src.display(), dest.display());
        if !src.exists() {
            panic!("DLL file not found: {}", src.display());
        }
        if let Err(e) = fs::copy(src, dest.join(src.file_name().unwrap())) {
            panic!("Failed to copy file {}: {}", src.display(), e);
        }
    }
    // Copy the ANPR DLLs
    for dll in &anpr_dlls {
        copy_dll(dll, &target_dir);
    }

    // Link the appropriate libraries based on the target architecture
    if is_x86_64 {
        println!("cargo:rustc-link-search=native=lib/x64");
        println!("cargo:rustc-link-lib=static=iANPR_vc14_x64");
        println!("cargo:rustc-link-lib=static=iANPRcapture_vc14_x64");
        println!("cargo:rustc-link-lib=static=iANPRinterface_vc14_x64");
        println!("cargo:rustc-link-lib=opencv_world340"); // Change this to the actual OpenCV library name
    } else {
        println!("cargo:rustc-link-search=native=lib/x86");
        println!("cargo:rustc-link-lib=static=iANPR_vc14_x86");
        println!("cargo:rustc-link-lib=static=iANPRcapture_vc14_x86");
        println!("cargo:rustc-link-lib=static=iANPRinterface_vc14_x86");
        println!("cargo:rustc-link-lib=opencv_world340"); // Change this to the actual OpenCV library name
    }

    // Print the LIBCLANG_PATH for debugging
    if let Ok(libclang_path) = env::var("LIBCLANG_PATH") {
        println!("Found LIBCLANG_PATH: {}", libclang_path);
    } else {
        println!("LIBCLANG_PATH is not set");
    }

    // The list of header files
    let headers = [
        "include/opencv2/core/core_c.h",
        "include/opencv2/highgui/highgui_c.h",
        "include/opencv2/imgproc/imgproc_c.h",
        "include/iANPR.h",
        "include/iANPRcapture.h",
        "include/iANPRCustom.h",
        "include/iANPRerror.h",
        "include/iANPRinterface.h",
    ];

    // Create the bindgen::Builder and add each header file
    let mut builder = bindgen::Builder::default();

    // Specify the include paths for the headers and necessary compiler flags
    builder = builder
        .time_phases(true)
        .clang_arg("-Iinclude")
        .clang_arg("-I/include") // Change this path to where your OpenCV headers are located
        .clang_arg("-x")
        .clang_arg("c++");

    for header in headers.iter() {
        builder = builder.header(header.to_string());
    }

    // Generate the bindings
    let bindings = builder.generate().expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
