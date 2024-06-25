#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(dead_code)] // Optional: suppresses warnings for unused code
#![allow(non_camel_case_types)] // Suppresses warnings for type names that should be in upper camel case
#![allow(unused_variables)]
use std::{
    ffi::{CStr, CString},
    os::raw::{c_char, c_int, c_void},
    ptr,
};
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));


pub struct AnprImage {
    pub ptr: *mut _IplImage,
}


impl AnprImage {
    pub fn load_image(path: &str) -> Result<Self, String> {
        let c_path = CString::new(path).map_err(|_| "Failed to convert path to CString")?;
        let img = unsafe { cvLoadImage(c_path.as_ptr(), CV_LOAD_IMAGE_COLOR) };
        if img.is_null() {
            Err("Failed to load image".to_string())
        } else {
            Ok(Self { ptr: img })
        }
    }

    pub fn save_image(&self, path: &str) -> Result<(), String> {
        let c_path = CString::new(path).map_err(|_| "Failed to convert path to CString")?;
        let result = unsafe { cvSaveImage(c_path.as_ptr(), self.ptr as *mut c_void, ptr::null_mut()) };
        if result != 0 {
            Err("Failed to save image".to_string())
        } else {
            Ok(())
        }
    }
}

impl Drop for AnprImage {
    fn drop(&mut self) {
        unsafe { cvReleaseImage(&mut self.ptr) }
    }
}


pub struct AnprOptions {
    pub min_plate_size: c_int,
    pub max_plate_size: c_int,
    pub detect_mode: c_int,
    pub max_text_size: c_int,
    pub type_number: c_int,
    pub flags: c_int,
    pub custom: *mut c_void,
    pub vers: CString,
    pub alpha: f64,
    pub beta: f64,
    pub gamma: f64,
    pub max_threads: c_int,
}


impl AnprOptions {
    pub fn new(version: &str) -> Self {
        Self {
            min_plate_size: 500,
            max_plate_size: 50000,
            detect_mode: ANPR_DETECTCOMPLEXMODE as c_int,
            max_text_size: 20,
            type_number: 104,
            flags: 0,
            custom: ptr::null_mut(),
            vers: CString::new(version).unwrap(),
            alpha: 90.0,
            beta: 90.0,
            gamma: 90.0,
            max_threads: 1,
        }
    }
    pub fn is_full_type(&self, full_types: &[c_int]) -> bool {
        full_types.contains(&self.type_number)
    }
}


pub fn anpr_plate(img: &AnprImage, options: &AnprOptions) -> Result<Vec<String>, String> {
    let mut all: c_int = 100;
    let mut rects = vec![
        CvRect {
            x: 0,
            y: 0,
            width: 0,
            height: 0
        };
        100
    ];
    let rects_ptr: *mut CvRect = rects.as_mut_ptr();
    let res = allocate_array(all as usize)?;

    let full_types = [4, 7, 9, 310, 311, 911];
    let is_full_type = options.is_full_type(&full_types);

    let result = if is_full_type {
        unsafe {
            anprPlate(
                img.ptr,
                ANPR_OPTIONS {
                    sign1: b'i' as c_char,
                    sign2: b'a' as c_char,
                    sign3: b'1' as c_char,
                    min_plate_size: options.min_plate_size,
                    max_plate_size: options.max_plate_size,
                    Detect_Mode: options.detect_mode,
                    max_text_size: options.max_text_size,
                    type_number: options.type_number,
                    flags: options.flags,
                    custom: options.custom,
                    vers: options.vers.as_ptr() as *mut c_char,
                    alpha: options.alpha,
                    beta: options.beta,
                    gamma: options.gamma,
                    max_threads: options.max_threads,
                },
                &mut all,
                rects_ptr,
                res,
                ptr::null_mut(),
            )
        }
    } else {
        unsafe {
            let size = CvSize { width: 1193, height: 671 };
            let gray = cvCreateImage(size, 8, 1);
            cvCvtColor(img.ptr as *mut CvArr, gray as *mut CvArr, CV_BGR2GRAY);
            cvSaveImage(CString::new("gray.jpg").unwrap().as_ptr(), gray as *mut c_void, ptr::null_mut());
            anprPlate(
                gray,
                ANPR_OPTIONS {
                    sign1: b'i' as c_char,
                    sign2: b'a' as c_char,
                    sign3: b'1' as c_char,
                    min_plate_size: options.min_plate_size,
                    max_plate_size: options.max_plate_size,
                    Detect_Mode: options.detect_mode,
                    max_text_size: options.max_text_size,
                    type_number: options.type_number,
                    flags: options.flags,
                    custom: options.custom,
                    vers: options.vers.as_ptr() as *mut c_char,
                    alpha: options.alpha,
                    beta: options.beta,
                    gamma: options.gamma,
                    max_threads: options.max_threads,
                },
                &mut all,
                rects_ptr,
                res,
                ptr::null_mut(),
            )
        }
    };

    if result == 0 {
        let mut plate_numbers = Vec::new();
        unsafe {
            for j in 0..all as usize {
                if !(*res.add(j)).is_null() {
                    let c_str = CStr::from_ptr(*res.add(j));
                    plate_numbers.push(c_str.to_str().unwrap().to_string());
                }
            }
        }
        deallocate_array(res, all as usize);
        Ok(plate_numbers)
    } else {
        deallocate_array(res, all as usize);
        Err(format!("Error: {}", result))
    }
}

fn allocate_array(all: usize) -> Result<*mut *mut c_char, String> {
    unsafe {
        let res = libc::malloc(all * std::mem::size_of::<*mut c_char>()) as *mut *mut c_char;
        if res.is_null() {
            return Err("Failed to allocate memory".to_string());
        }
        for j in 0..all {
            let inner_array = libc::malloc(20 * std::mem::size_of::<c_char>()) as *mut c_char;
            if inner_array.is_null() {
                for k in 0..j {
                    libc::free(*res.add(k) as *mut libc::c_void);
                }
                libc::free(res as *mut libc::c_void);
                return Err("Failed to allocate memory".to_string());
            }
            *res.add(j) = inner_array;
        }
        Ok(res)
    }
}

fn deallocate_array(res: *mut *mut c_char, all: usize) {
    unsafe {
        for j in 0..all {
            libc::free(*res.add(j) as *mut libc::c_void);
        }
        libc::free(res as *mut libc::c_void);
    }
}