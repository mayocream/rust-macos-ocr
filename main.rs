use vision_ocr::ocr_image_with_language;
use vision_ocr::get_supported_languages;

fn main() {
    match get_supported_languages("accurate") {
        Ok(languages) => {
            println!("Supported Languages:");
            for lang in languages {
                println!("{}", lang);
            }
        }
        Err(error) => {
            println!("Error: {}", error);
        }
    }

    let path = "/Users/mayo/Workspaces/tschan/tests/small.png";
    let results = ocr_image_with_language(path, "ja-JP");
    // Display the results
    for (text, confidence) in results {
        println!("Recognized text: {}", text);
        println!("Confidence: {:.2}%", confidence * 100.0);
        println!("-------------------------");
    }
}
