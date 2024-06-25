#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(dead_code)] // Optional: suppresses warnings for unused code
#![allow(non_camel_case_types)] // Suppresses warnings for type names that should be in upper camel case
#![allow(unused_variables)]
use std::{ffi::{CStr, CString}, ptr};

use libc::{c_int, c_char, c_void};


include!(concat!(env!("OUT_DIR"), "/bindings.rs"));



pub struct ANPRCapture {
    handle: iANPRCapture,
}

impl ANPRCapture {
    pub fn new(max_frames: c_int, options: &ANPR_OPTIONS, rect: CvRect) -> Self {
        let handle = unsafe { CreateiANPRCapture(max_frames, *options, rect) };
        ANPRCapture { handle }
    }

    pub fn add_frame(&self, image: &IplImage, all_number: &mut c_int, rects: &mut [CvRect], texts: &mut Vec<String>) -> c_int {
        let mut res: Vec<*mut c_char> = vec![ptr::null_mut(); texts.len()];
        let result = unsafe {
            AddFrameToiANPRCapture(
                self.handle,
                image as *const IplImage as *mut IplImage,
                all_number,
                rects.as_mut_ptr(),
                res.as_mut_ptr(),
            )
        };
        for (i, &ptr) in res.iter().enumerate() {
            if !ptr.is_null() {
                texts[i] = unsafe { CStr::from_ptr(ptr) }.to_string_lossy().into_owned();
                unsafe { libc::free(ptr as *mut c_void) };
            }
        }
        result
    }

    pub fn create_memory(&self, min_frames_with_plate: c_int, frames_without_plate: c_int, max_plates_in_mem: c_int) -> c_int {
        unsafe {
            CreateMemoryForiANPRCapture(self.handle, min_frames_with_plate, frames_without_plate, max_plates_in_mem)
        }
    }

    pub fn create_line_intersection(&self, p1a: CvPoint, p2a: CvPoint, p1b: CvPoint, p2b: CvPoint) -> c_int {
        unsafe { CreateLineIntersection(self.handle, p1a, p2a, p1b, p2b) }
    }
}

impl Drop for ANPRCapture {
    fn drop(&mut self) {
        unsafe {
            ReleaseiANPRCapture(&mut self.handle);
        }
    }
}

pub struct License {
    key: Vec<u8>,
}

impl License {
    pub fn load_from_file(filename: &str) -> Result<Self, std::io::Error> {
        let mut key = vec![0u8; 8001];
        let mut file = std::fs::File::open(filename)?;
        use std::io::Read;
        file.read_exact(&mut key)?;
        unsafe {
            LicenseValue(key.as_ptr() as *mut i8);
        }
        Ok(License { key })
    }
}

pub fn recognize_plate(video_path: Option<&str>, type_number: i32) {
   
    let mut capture = if let Some(path) = video_path {
        let path_cstring = CString::new(path).unwrap();
        unsafe { cvCreateFileCapture(path_cstring.as_ptr()) }
    } else {
        unsafe {  cvCreateCameraCapture(0) }
    };
    
    if capture.is_null() {
        println!("Can't load file or camera");
        return;
    }

    let buffer = "out.avi";
    let  video_write: *mut CvVideoWriter = ptr::null_mut();

    let mut options = ANPR_OPTIONS{
        sign1: b'i' as i8,
        sign2: b'a' as i8,
        sign3: b'1' as i8,
        min_plate_size: 500,
        max_plate_size: 50000,
        Detect_Mode: ANPR_DETECTCOMPLEXMODE as c_int,
        max_text_size: 20,
        type_number: 0,
        flags: 0,
        custom: ptr::null_mut(),
        vers: ptr::null_mut(),
        alpha: 90.0,
        beta: 90.0,
        gamma: 90.0,
        max_threads: 1,
    };
    options.type_number = type_number;
    
    let is_full_type  = true;

    if let Err(e) = License::load_from_file("lic.key") {
        println!("WARNING! File lic.key not found. This may crash the program if you use a licensed version of iANPR SDK. Error: {}", e);
    }

    let mut gray_img: *mut IplImage = ptr::null_mut();
    let mut i_capture = ANPRCapture::new(10, &options, CvRect { x: 0, y: 0, width: 0, height: 0 });

    loop {
        let frame = unsafe { cvQueryFrame(capture) as *mut IplImage };
        if frame.is_null() {
            break;
        }

        if gray_img.is_null() {
            let size = unsafe { cvGetSize(frame as *const CvArr) };
            gray_img = create_image(size, 8, 1);
            i_capture = ANPRCapture::new(10, &options, CvRect { x: 0, y: 0, width: size.width, height: size.height });
        }

        convert_color(frame, gray_img, CV_BGR2GRAY);

        let tick1 = unsafe { cvGetTickCount() };
        let mut all: c_int = 100;
        let mut rects: [CvRect; 100] = [CvRect { x: 0, y: 0, width: 0, height: 0 }; 100];
        let mut texts: Vec<String> = vec![String::new(); all as usize];

        let i1 = if is_full_type {
            i_capture.add_frame(unsafe { &*frame }, &mut all, &mut rects, &mut texts)
        } else {
            i_capture.add_frame(unsafe { &*gray_img }, &mut all, &mut rects, &mut texts)
        };

        let tick2 = unsafe { cvGetTickCount() };
        let processing_time = (tick2 - tick1) as f32 / 1000.0;
        println!("Ret: {}; num: {}; time: {:.3}; cand: {}", i1, 0, processing_time, all);
        let test =  CvScalar{
            val: [1.0,1.0,1.0,1.0]
        };
        for (i, text) in texts.iter().enumerate() {
            if !text.is_empty() {
                unsafe {
                    cvRectangle(frame as *mut c_void, cv_point(rects[i].x, rects[i].y), cv_point(rects[i].x + rects[i].width, rects[i].y + rects[i].height), test, 2, 8, 0);
                    let font = CvFont {
                        nameFont: std::ptr::null(),
                        color: CvScalar { val: [0.0, 0.0, 0.0, 0.0] },
                        font_face: CV_FONT_HERSHEY_SIMPLEX as i32,
                        ascii: std::ptr::null(),
                        greek: std::ptr::null(),
                        cyrillic: std::ptr::null(),
                        hscale: 1.0,
                        vscale: 1.0,
                        shear: 0.0,
                        thickness: 1,
                        dx: 0.0,
                        line_type: 8,
                    };
                  
                    let pp2 = cv_point(rects[i].x, rects[i].y);
                    let pp1 = cv_point(rects[i].x + 1, rects[i].y + 1);
                    let text_cstring = CString::new(text.clone()).unwrap();
                    cvPutText(frame as *mut c_void,  text_cstring.as_ptr(), pp1, &font, test);
                    cvPutText(frame as *mut c_void,  text_cstring.as_ptr(), pp2, &font, test);
                }
            }
        }

        // Additional processing can be added here
    }

    unsafe {
        cvReleaseCapture(&mut capture);
        cvReleaseImage(&mut gray_img);
    }
}

// Additional helper functions

pub fn create_image(size: CvSize, depth: c_int, channels: c_int) -> *mut IplImage {
    unsafe { cvCreateImage(size, depth, channels) }
}

pub fn convert_color(src: *const IplImage, dst: *mut IplImage, code: c_int) {
    unsafe { cvCvtColor(src as *const c_void, dst as *mut c_void, code) }
}
pub fn cv_point(x: c_int, y: c_int) -> CvPoint {
    CvPoint { x, y }
}





// fn cv_size(width: i32, height: i32) -> CvSize {
//     CvSize { width, height }
// }


// fn print_help(program_name: &str) {
//     println!("Use: {} <type_number> <path to image>\n", program_name);
//     println!("type_number: 7 for Russian, 104 for Kazakhstan, 203 for Turkmenistan, 300 for Belarus vehicle registration plates");
//     println!("For more type_numbers please refer to iANPR SDK documentation\n");
//     println!("Example: {} 7 C:\\test.jpg - recognition of russian vehicle registration plates from file test.jpg\n", program_name);
// }



// fn main() {
    
//     let args: Vec<String> = std::env::args().collect();

//     if args.len() < 2 {
//         println!("Too few arguments. For help print {} /?", args[0]);
//         return;
//     } else if args[1] == "help" || args[1] == "-help" || args[1] == "--help" || args[1] == "/?" {
//         print_help(&args[0]);
//         return;
//     } else if args.len() < 3 {
//         println!("Too few arguments. For help print {} /?", args[0]);
//         return;
//     }
    
//     let img_path = &args[2];
//     let img = unsafe { cvLoadImage(CString::new(img_path.clone()).unwrap().as_ptr(), CV_LOAD_IMAGE_COLOR) };

//     if img.is_null() {
//         println!("Can't load file!");
//         return;
//     }

//     let mut rects: [CvRect; 100] = [CvRect { x: 0, y: 0, width: 0, height: 0 }; 100];
//     let mut all: i32 = 100; // Change usize to i32
//     let mut res: Vec<*mut i8> = vec![ptr::null_mut(); all as usize];

//     for j in 0..all as usize {
//         res[j] = CString::new(vec![' '; 20].into_iter().collect::<String>()).unwrap().into_raw();
//     }

//     let vers_cstring = CString::new("1.7").unwrap();
//     let vers_ptr = vers_cstring.into_raw();

//     let mut options = ANPR_OPTIONS {
//         sign1: 'i' as i8,
//         sign2: 'a' as i8,
//         sign3: '1' as i8,
//         min_plate_size: 500,
//         max_plate_size: 50000,
//         Detect_Mode: ANPR_DETECTCOMPLEXMODE as i32, // Ensure correct type
//         max_text_size: 20,
//         type_number: args[1].parse().expect("Invalid type number"),
//         flags: 0,
//         custom: ptr::null_mut(),
//         vers: vers_ptr,
//         alpha: 90.0,
//         beta: 90.0,
//         gamma: 90.0,
//         max_threads: 1,
//     };


//     let key_file = "lic.key";
//     let mut key = vec![0u8; 8001];
//     let key_slice = &mut key[..];


//     let mut i = -9999;
   
//         unsafe {
//             i = anprPlate(img, options, &mut all, rects.as_mut_ptr(), res.as_mut_ptr(), ptr::null_mut());
//         }
    

//     if i == 0 {
//         for j in 0..all as usize {
//             unsafe {
//                 println!("{}", CStr::from_ptr(res[j]).to_str().unwrap());
//             }
//         }
//     } else {
//         println!("Error: {}", i);
//     }

//     for j in res {
//         unsafe {
//             CString::from_raw(j);
//         }
//     }

//     unsafe {
//         cvReleaseImage(&img as *const _ as *mut _);
//     }

//     unsafe {
//         CString::from_raw(vers_ptr);
//     }
// }