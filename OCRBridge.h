#ifndef OCRBridge_h
#define OCRBridge_h

typedef struct {
    char* text;
    float confidence;
} RecognizedText;

RecognizedText* performOCRWithLanguage(const char* imagePath, const char* language, int *result_count);
const char* getSupportedLanguages(int recognition_level);

#endif /* OCRBridge_h */
