#import "OCRBridge.h"
#import <Foundation/Foundation.h>
#import <Vision/Vision.h>

RecognizedText* performOCRWithLanguage(const char* imagePath, const char* language, int *result_count) {
    @autoreleasepool {
        NSMutableArray *results = [NSMutableArray array];

        NSError *error = nil;
        VNRecognizeTextRequest *request = [[VNRecognizeTextRequest alloc] initWithCompletionHandler:^(VNRequest * _Nonnull request, NSError * _Nullable error) {
            if (error == nil) {
                for (VNRecognizedTextObservation *observation in request.results) {
                    VNRecognizedText *topText = [[observation topCandidates:1] firstObject];
                    RecognizedText recognizedText;
                    recognizedText.text = strdup([topText.string UTF8String]);
                    recognizedText.confidence = topText.confidence;
                    [results addObject:[NSValue valueWithBytes:&recognizedText objCType:@encode(RecognizedText)]];
                }
            }
        }];

        // Set recognition languages and level
        request.recognitionLanguages = @[[NSString stringWithUTF8String:language]];
        request.recognitionLevel = VNRecognizeTextRequestRevision3; // For accurate recognition

        NSURL *url = [NSURL fileURLWithPath:[NSString stringWithUTF8String:imagePath]];
        VNImageRequestHandler *handler = [[VNImageRequestHandler alloc] initWithURL:url options:@{}];
        [handler performRequests:@[request] error:&error];

        *result_count = (int)[results count];
        RecognizedText* recognizedTexts = (RecognizedText*)malloc(sizeof(RecognizedText) * [results count]);
        for (NSUInteger i = 0; i < [results count]; i++) {
            RecognizedText text;
            [[results objectAtIndex:i] getValue:&text];
            recognizedTexts[i] = text;
        }

        return recognizedTexts;
    }
}

const char* getSupportedLanguages(int recognition_level) {
    @autoreleasepool {
        NSError *error = nil;
        NSArray<NSString *> *languages = [VNRecognizeTextRequest supportedRecognitionLanguagesForTextRecognitionLevel:recognition_level
                                                                                                           revision:VNRecognizeTextRequestRevision3
                                                                                                              error:&error];
        if (error) {
            return strdup([[error localizedDescription] UTF8String]);
        }
        return strdup([[languages componentsJoinedByString:@","] UTF8String]);
    }
}
