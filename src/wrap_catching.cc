#include "vosk_api.h"
#include "model.h"
#include "spk_model.h"

using namespace kaldi;

extern "C" {

VoskModel *vosk_model_new_or_null(const char *model_path) {
  try {
    return (VoskModel *)new Model(model_path);
  } catch (...) {
    return NULL;
  }
}

VoskSpkModel *vosk_spk_model_new_or_null(const char *model_path){
  try {
    return (VoskSpkModel *)new SpkModel(model_path);
  } catch (...) {
    return NULL;
  }
}

}
