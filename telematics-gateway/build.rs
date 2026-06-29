use dbc_codegen2::{CodegenPipeline, CodegenConfig, Language};
fn main() -> anyhow::Result<()> {
    CodegenPipeline::run(CodegenConfig {
        inputs: vec!["../network.dbc".to_string()],
        output: "src/can_matrix.rs".to_string(),
        lang: Language::Rust,
        no_enum_other: false, no_enum_dedup: false,
        zero_zero_range_allows_all: false,
        rust_code_injections: std::collections::HashMap::new(),
        cpp_code_injections: std::collections::HashMap::new(),
        generate_tests: false, separate: false,
    })
}