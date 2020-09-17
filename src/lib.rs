/** Model stores all the data required for recognition
 *  it contains static data and can be shared across processing
 *  threads. */
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VoskModel {
    _unused: [u8; 0],
}

/** Speaker model is the same as model but contains the data
 *  for speaker identification. */
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VoskSpkModel {
    _unused: [u8; 0],
}

/** Recognizer object is the main object which processes data.
 *  Each recognizer usually runs in own thread and takes audio as input.
 *  Once audio is processed recognizer returns JSON object as a string
 *  which represent decoded information - words, confidences, times, n-best lists,
 *  speaker information and so on */
#[repr(C)]
#[derive(Debug)]
pub struct VoskRecognizer {
    _unused: [u8; 0],
}

extern "C" {
    /// Loads model data from the path and returns the model object
    /// throws `kaldi::KaldiFatalError` on failure
    pub fn vosk_model_new(model_path: *const ::std::os::raw::c_char) -> *mut VoskModel;

    /// Loads model data from the path and returns the model object
    /// or null if there was an error
    pub fn vosk_model_new_or_null(model_path: *const ::std::os::raw::c_char) -> *mut VoskModel;

    /// Releases the model memory
    ///
    ///  The model object is reference-counted so if some recognizer
    ///  depends on this model, model might still stay alive. When
    ///  last recognizer is released, model will be released too.
    pub fn vosk_model_free(model: *mut VoskModel);

    /// Loads speaker model data from the path and returns the model object
    pub fn vosk_spk_model_new(model_path: *const ::std::os::raw::c_char) -> *mut VoskSpkModel;

    /// Loads speaker model data from the path and returns the model object
    /// or null if there was an error
    pub fn vosk_spk_model_new_or_null(
        model_path: *const ::std::os::raw::c_char,
    ) -> *mut VoskSpkModel;

    /// Releases the model memory
    ///
    ///  The model object is reference-counted so if some recognizer
    ///  depends on this model, model might still stay alive. When
    ///  last recognizer is released, model will be released too.
    pub fn vosk_spk_model_free(model: *mut VoskSpkModel);

    /// Creates the recognizer object
    ///
    ///  The recognizers process the speech and return text using shared model data
    ///
    ///   `sample_rate`: The sample rate of the audio you going to feed into the recognizer
    pub fn vosk_recognizer_new(model: *mut VoskModel, sample_rate: f32) -> *mut VoskRecognizer;

    /// Creates the recognizer object with speaker recognition
    ///
    ///  With the speaker recognition mode the recognizer not just recognize
    ///  text but also return speaker vectors one can use for speaker identification
    ///
    ///   `spk_model`: speaker model for speaker identification
    ///
    ///   `sample_rate`: The sample rate of the audio you going to feed into the recognizer
    pub fn vosk_recognizer_new_spk(
        model: *mut VoskModel,
        spk_model: *mut VoskSpkModel,
        sample_rate: f32,
    ) -> *mut VoskRecognizer;

    /// Creates the recognizer object with the grammar
    ///
    ///  Sometimes when you want to improve recognition accuracy and when you don't need
    ///  to recognize large vocabulary you can specify a list of words to recognize. This
    ///  will improve recognizer speed and accuracy but might return `[unk]` if user said
    ///  something different.
    ///
    ///  Only recognizers with lookahead models support this type of quick configuration.
    ///  Precompiled HCLG graph models are not supported.
    ///
    ///   `sample_rate` The sample rate of the audio you going to feed into the recognizer
    ///   `grammar` The string with the list of words to recognize, for example "one two three four five [unk]"
    pub fn vosk_recognizer_new_grm(
        model: *mut VoskModel,
        sample_rate: f32,
        grammar: *const ::std::os::raw::c_char,
    ) -> *mut VoskRecognizer;

    /// Accept voice data
    ///
    ///  accept and process new chunk of voice data
    ///
    ///   `data` - audio data in PCM 16-bit mono format
    ///   `length` - length of the audio data
    ///
    ///  returns true if silence has occurred and you can retrieve a new utterance with result method
    // 16-bit data needs to be converted to bytes in little endian
    pub fn vosk_recognizer_accept_waveform(
        recognizer: *mut VoskRecognizer,
        data: *const ::std::os::raw::c_char,
        length: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;

    /// Same as above but the version with the short data for language bindings where you have
    ///  audio as array of shorts
    pub fn vosk_recognizer_accept_waveform_s(
        recognizer: *mut VoskRecognizer,
        data: *const ::std::os::raw::c_short,
        length: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;

    /// Same as above but the version with the float data for language bindings where you have
    ///  audio as array of floats
    pub fn vosk_recognizer_accept_waveform_f(
        recognizer: *mut VoskRecognizer,
        data: *const f32,
        length: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;

    /// Returns speech recognition result
    ///
    /// returns the result in JSON format which contains decoded line, decoded
    ///          words, times in seconds and confidences. You can parse this result
    ///          with any json parser
    /// ```json
    /// {
    ///   "result" : [{
    ///       "conf" : 1.000000,
    ///       "end" : 1.110000,
    ///       "start" : 0.870000,
    ///       "word" : "what"
    ///     }, {
    ///       "conf" : 1.000000,
    ///       "end" : 1.530000,
    ///       "start" : 1.110000,
    ///       "word" : "zero"
    ///     }, {
    ///       "conf" : 1.000000,
    ///       "end" : 1.950000,
    ///       "start" : 1.530000,
    ///       "word" : "zero"
    ///     }, {
    ///       "conf" : 1.000000,
    ///       "end" : 2.340000,
    ///       "start" : 1.950000,
    ///       "word" : "zero"
    ///     }, {
    ///       "conf" : 1.000000,
    ///      "end" : 2.610000,
    ///       "start" : 2.340000,
    ///       "word" : "one"
    ///     }],
    ///   "text" : "what zero zero zero one"
    ///  }
    /// ```
    pub fn vosk_recognizer_result(recognizer: *mut VoskRecognizer)
        -> *const ::std::os::raw::c_char;

    /// Returns partial speech recognition
    ///
    /// returns partial speech recognition text which is not yet finalized.
    ///          result may change as recognizer process more data.
    /// ```json
    /// {
    ///  "partial" : "cyril one eight zero"
    /// }
    /// ```
    pub fn vosk_recognizer_partial_result(
        recognizer: *mut VoskRecognizer,
    ) -> *const ::std::os::raw::c_char;

    /// Returns speech recognition result.
    ///
    ///  Same as result, but doesn't wait for silence
    ///  You usually call it in the end of the stream to get final bits of audio. It
    ///  flushes the feature pipeline, so all remaining audio chunks got processed.
    ///  returns speech result in JSON format.
    pub fn vosk_recognizer_final_result(
        recognizer: *mut VoskRecognizer,
    ) -> *const ::std::os::raw::c_char;

    /// Releases recognizer object
    ///
    ///  Underlying model is also unreferenced and if needed released
    pub fn vosk_recognizer_free(recognizer: *mut VoskRecognizer);

    /// Set log level for Kaldi messages
    ///
    ///   log_level the level
    ///     0 - default value to print info and error messages but no debug
    ///     less than 0 - don't print info messages
    ///     greather than 0 - more verbose mode
    pub fn vosk_set_log_level(log_level: ::std::os::raw::c_int);
}
