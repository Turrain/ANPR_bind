#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(dead_code)] // Optional: suppresses warnings for unused code
#![allow(non_camel_case_types)] // Suppresses warnings for type names that should be in upper camel case
#![allow(unused_variables)]
use std::{
    alloc::Layout, ffi::{c_double, CStr, CString}, os::raw::{c_char, c_int, c_void}, ptr, time::Instant
};
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[derive(Debug)]
pub enum AnprError {
    ImageEmpty,
    ErrorTypePlate,
    ErrorTypeForColor,
    Other(i32),
}

impl AnprError {
    pub fn from_code(code: i32) -> Result<(), AnprError> {
        match code {
            0 => Ok(()),
            1 => Err(AnprError::Other(1)),
            2 => Err(AnprError::Other(2)),
            -2 => Err(AnprError::ImageEmpty),
            -100 => Err(AnprError::ErrorTypePlate),
            -101 => Err(AnprError::ErrorTypeForColor),
            _ => Err(AnprError::Other(code)),
        }
    }

    pub fn description(&self) -> &str {
        match self {
            AnprError::ImageEmpty => "IMAGE_EMPTY: The image is empty.",
            AnprError::ErrorTypePlate => "ERROR_TYPE_PLATE: Unsupported plate type for this configuration.",
            AnprError::ErrorTypeForColor => "ERROR_TYPE_FOR_COLOR: Mismatch between image type and plate type flag in ANPR_OPTIONS.",
            AnprError::Other(code) => match code {
                1 => "No candidates detected for the license plate.",
                2 => "No license plates found.",
                _ => "Unknown error.",
            },
        }
    }
}


pub struct AnprVideoCapture {
    pub cap: *mut CvCapture,
}

impl AnprVideoCapture {
    pub fn from_file(filename: &str) -> Result<Self, String> {
        let c_filename = CString::new(filename).map_err(|e| e.to_string())?;
        let cap = unsafe { cvCreateFileCapture(c_filename.as_ptr()) };
        if cap.is_null() {
            return Err("Failed to open video file".to_string());
        }
        Ok(Self { cap })
    }

    pub fn from_camera(camera_index: i32) -> Result<Self, String> {
        let cap = unsafe { cvCreateCameraCapture(camera_index) };
        if cap.is_null() {
            return Err("Failed to open camera".to_string());
        }
        Ok(Self { cap })
    }
    
    pub fn from_url(url: &str) -> Result<Self, String> {
        let c_url = CString::new(url).map_err(|e| e.to_string())?;
        let cap = unsafe { cvCreateFileCapture(c_url.as_ptr()) };
        if cap.is_null() {
            return Err("Failed to open video URL".to_string());
        }
        Ok(Self { cap })
    }

    pub fn read_frame(&mut self) -> Result<AnprImage, String> {
        let frame = unsafe { cvQueryFrame(self.cap) };
        if frame.is_null() {
            return Err("Failed to capture frame".to_string());
        }
        Ok(AnprImage { ptr: frame })
    }
}

impl Drop for AnprVideoCapture {
    fn drop(&mut self) {
        unsafe {
            cvReleaseCapture(&mut self.cap);
        }
    }
}

pub struct AnprImage {
    pub ptr: *mut _IplImage,
}

impl AnprImage {
    pub fn as_ptr(&self) -> *const IplImage {
        self.ptr as *const IplImage
    }

    pub fn as_mut_ptr(&mut self) -> *mut IplImage {
        self.ptr
    }

    pub fn get_size(&self) -> CvSize {
        unsafe {
            CvSize {
                width: (*self.ptr).width,
                height: (*self.ptr).height,
            }
        }
    }

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
        let result =
            unsafe { cvSaveImage(c_path.as_ptr(), self.ptr as *mut c_void, ptr::null_mut()) };
        if result != 0 {
            Err("Failed to save image".to_string())
        } else {
            Ok(())
        }
    }
}

impl Drop for AnprImage {
    fn drop(&mut self) {
      //  unsafe { println!("{:?} {:?}", self.ptr, *self.ptr) }
        // if !self.ptr.is_null() {
        //     unsafe { cvReleaseImage(&mut self.ptr) }
        // }
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
    pub alpha: c_double,
    pub beta: c_double,
    pub gamma: c_double,
    pub max_threads: c_int,
}
unsafe impl Send for AnprOptions {}

impl AnprOptions {
    pub fn with_min_plate_size(mut self, min_plate_size: c_int) -> Self {
        self.min_plate_size = min_plate_size;
        self
    }

    pub fn with_max_plate_size(mut self, max_plate_size: c_int) -> Self {
        self.max_plate_size = max_plate_size;
        self
    }

    pub fn with_detect_mode(mut self, detect_mode: c_int) -> Self {
        self.detect_mode = detect_mode;
        self
    }

    pub fn with_max_text_size(mut self, max_text_size: c_int) -> Self {
        self.max_text_size = max_text_size;
        self
    }

    pub fn with_type_number(mut self, type_number: c_int) -> Self {
        self.type_number = type_number;
        self
    }

    pub fn with_flags(mut self, flags: c_int) -> Self {
        self.flags = flags;
        self
    }

    pub fn with_custom(mut self, custom: *mut c_void) -> Self {
        self.custom = custom;
        self
    }

    pub fn with_vers(mut self, vers: &str) -> Self {
        self.vers = CString::new(vers).unwrap();
        self
    }

    pub fn with_alpha(mut self, alpha: c_double) -> Self {
        self.alpha = alpha;
        self
    }

    pub fn with_beta(mut self, beta: c_double) -> Self {
        self.beta = beta;
        self
    }

    pub fn with_gamma(mut self, gamma: c_double) -> Self {
        self.gamma = gamma;
        self
    }

    pub fn with_max_threads(mut self, max_threads: c_int) -> Self {
        self.max_threads = max_threads;
        self
    }

    pub fn is_full_type(&self, full_types: &[c_int]) -> bool {
        full_types.contains(&self.type_number)
    }
}
impl Default for AnprOptions {
    fn default() -> Self {
        Self {
            min_plate_size: 500,
            max_plate_size: 50000,
            detect_mode: ANPR_DETECTCOMPLEXMODE as c_int,
            max_text_size: 20,
            type_number: 104,
            flags: 0,
            custom: ptr::null_mut(),
            vers: CString::new("1.6.0").unwrap(),
            alpha: 90.0,
            beta: 90.0,
            gamma: 90.0,
            max_threads: 1,
        }
    }
}

pub fn anpr_video(video_path: Option<String>, type_number: i32) -> Result<(), String> {
    let mut frame_capture = match video_path {
        Some(path) => AnprVideoCapture::from_file(&path)?,
        None => AnprVideoCapture::from_camera(0)?,
    };

    let mut gray_frame = AnprImage {
        ptr: ptr::null_mut(),
    };

 
    let anpr_options = AnprOptions::default().with_type_number(type_number);
 
    let full_types = [4, 7, 9, 310, 311, 911];
    let is_full_type = anpr_options.is_full_type(&full_types);
    loop {
        let mut frame = frame_capture.read_frame()?;
        if frame.ptr.is_null() {
            break;
        }
        frame.save_image("C:/Users/Debuger/ANPR_Test/Test.jpg");
        let start = Instant::now();
       
        match anpr_plate(&frame, &anpr_options) {
            Ok(e) => {
                println!("{:?}",e)
            },
            Err(e) => {
                eprintln!("{}",e)
            }
        };
        let duration = start.elapsed();

        println!(
            "time: {:.3}; ",
            duration.as_secs_f32(),
        
        );
  
    }

    Ok(())
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
            let mut gray = cvCreateImage(img.get_size(), 8, 1);
            cvCvtColor(img.ptr as *mut c_void, gray as *mut c_void, CV_BGR2GRAY);
         
            cvSaveImage(
                CString::new("C:/Users/Debuger/ANPR_Test/Test.jpg").unwrap().as_ptr(),
                gray as *mut c_void,
                ptr::null_mut(),
            );
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
            for j in 0..all {
                let ptr = *res.add(j as usize);
                if !ptr.is_null() {
                    let c_str = CStr::from_ptr(ptr);
                    match c_str.to_str() {
                        Ok(s) => plate_numbers.push(s.to_string()),
                        Err(e) => eprintln!("Failed to convert to string: {}", e),
                    }
                }
            }
        }
      

        deallocate_array(res, all as usize);

        Ok(plate_numbers)
    } else {

        let t:Vec<String> = vec![String::from("Error")];
        deallocate_array(res, all as usize);
     //   Ok(t)
     
        let s = match result {
            1 => "No candidates detected for the license plate.",
            2 => "No license plates found.",
            -2 => "IMAGE_EMPTY: The image is empty.",
            -100 => "ERROR_TYPE_PLATE: Unsupported plate type for this configuration.",
            -101 => "ERROR_TYPE_FOR_COLOR: Mismatch between image type and plate type flag in ANPR_OPTIONS.",
            _ => "Unknown error."

        };
         Err(format!("Error: {}", s))
    }
}
// fn allocate_array(all: usize) -> Result<Vec<String>, String> {
//     let mut res = Vec::with_capacity(all);
//     for _ in 0..all {
//         res.push(String::with_capacity(20));
//     }
//     Ok(res)
// }

// fn deallocate_array(res: Vec<String>, _all: usize) -> Result<(), String> {
//     drop(res);
//     Ok(())
// }
fn allocate_array(all: usize) -> Result<*mut *mut c_char, String> {
    unsafe {
        let res = libc::malloc(all * std::mem::size_of::<*mut c_char>()) as *mut *mut c_char;
        if res.is_null() {
            return Err("Failed to allocate memory".to_string());
        }
        
        for j in 0..all {
            let inner_array = libc::malloc(20 * std::mem::size_of::<c_char>()) as *mut c_char;
            if inner_array.is_null() {
                deallocate_array(res, j); // Reuse deallocation function for cleanup
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
            let inner_ptr = *res.add(j);
            if !inner_ptr.is_null() {
                libc::free(inner_ptr as *mut libc::c_void);
            }
        }
        libc::free(res as *mut libc::c_void);
    }
}
