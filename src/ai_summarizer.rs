#[cfg(feature = "llama")]
#[allow(dead_code)]
#[derive(Clone)]
/// An AI summarizer that uses the LLaMA C++ library
/// https://github.com/ggerganov/llama.cpp
/// https://github.com/ArkForgeLabs/llama_cpp-rs
///
/// To use it, you need to download a GGUF LLM model. Dolphin models are preferred.
/// example:
///
/// mistral 7B v0.2
/// https://huggingface.co/bartowski/dolphin-2.8-mistral-7b-v02-GGUF
///
/// mistral 7B
/// https://huggingface.co/TheBloke/dolphin-2.6-mistral-7B-GGUF
///
/// phi2 3B
/// https://huggingface.co/TheBloke/dolphin-2_6-phi-2-GGUF
///
/// any quantization level is ok, usually K_M is medium and should be good enough.
/// every model is also different in their capabilities, e.g. Phi2 is smaller and faster but less capable than Mistral 7B.
pub struct AiSummarizer {
    model: llama_cpp_rs::llama_cpp::LlamaModel,
}
#[cfg(feature = "llama")]
#[allow(dead_code)]
impl AiSummarizer {
    pub fn new(model_path: &str, params: Option<llama_cpp_rs::llama_cpp::LlamaParams>) -> Self {
        Self {
            model: llama_cpp_rs::llama_cpp::LlamaModel::load_from_file(
                model_path,
                params.unwrap_or_else(|| llama_cpp_rs::llama_cpp::LlamaParams::default()),
            )
            .expect("Failed to load model"),
        }
    }

    pub fn summarize(
        &self,
        release_notes: &str,
        session_params: Option<llama_cpp_rs::llama_cpp::SessionParams>,
    ) -> String {
        let mut context = self
            .model
            .create_session(session_params.unwrap_or_else(|| {
                println!("Using default session params");
                llama_cpp_rs::llama_cpp::SessionParams::default()
            }))
            .expect("Failed to create session");

        context.advance_context(format!(r#"
<|im_start|>system
you summarize release notes in a simple language and in one or two paragraphs. Include any web links.
Do not write anything outside of the reference release notes. Do not write unless you are sure. Do not write out of context. Keep it clean.<|im_end|>
<|im_start|>release notes
{release_notes}<|im_end|>
<|im_start|>assistant
"#)).expect("Failed to advance context");

        let max_tokens = 32000;
        context
            .start_completing_with(
                llama_cpp_rs::llama_cpp::standard_sampler::StandardSampler::default(),
                max_tokens,
            )
            .expect("Failed to start completions")
            .into_string()
    }
}
#[cfg(feature = "llama")]
unsafe impl Send for AiSummarizer {}
#[cfg(feature = "llama")]
unsafe impl Sync for AiSummarizer {}
