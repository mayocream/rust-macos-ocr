#[repr(C)]
struct RecognizedText {
    text: *mut i8,
    confidence: f32,
}

extern "C" {
    fn performOCRWithLanguage(imagePath: *const i8, language: *const i8, result_count: *mut i32) -> *mut RecognizedText;
    fn getSupportedLanguages(recognition_level: i32) -> *mut i8;
}

pub fn ocr_image_with_language(path: &str, lang: &str) -> Vec<(String, f32)> {
    let mut result_count = 0;
    let recognized_texts = unsafe {
        let path_cstring = std::ffi::CString::new(path).unwrap();
        let lang_cstring = std::ffi::CString::new(lang).unwrap();
        performOCRWithLanguage(path_cstring.as_ptr(), lang_cstring.as_ptr(), &mut result_count)
    };

    let mut results = Vec::new();
    for i in 0..result_count {
        unsafe {
            let recognized_text_ref = &*recognized_texts.offset(i as isize);
            let text = std::ffi::CStr::from_ptr(recognized_text_ref.text).to_string_lossy().into_owned();
            results.push((text, recognized_text_ref.confidence));
        }
    }

    // Don't forget to free the array itself after processing
    unsafe {
        libc::free(recognized_texts as *mut libc::c_void);
    }

    results
}

pub fn get_supported_languages(recognition_level: &str) -> Result<Vec<String>, String> {
    let level = match recognition_level {
        "fast" => 1,
        _ => 0,
    };

    let languages_cstring = unsafe {
        let result_ptr = getSupportedLanguages(level);
        let result_str = std::ffi::CStr::from_ptr(result_ptr).to_string_lossy().into_owned();
        libc::free(result_ptr as *mut libc::c_void);
        result_str
    };

    if languages_cstring.contains("Error:") {
        Err(languages_cstring)
    } else {
        Ok(languages_cstring.split(',').map(|s| s.to_string()).collect())
    }
}
