use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::misc::text_generator_utils::{
    generate_text_respecting_web_flag, section_header_comment,
};
use crate::codegen::generator::wire::dart::internal_config::GeneratorWireDartInternalConfig;
use crate::codegen::generator::wire::dart::spec_generator::output_code::WireDartOutputCode;
use crate::codegen::generator::wire::dart::spec_generator::WireDartOutputSpec;

pub(super) struct WireDartOutputText {
    pub(super) text: Acc<Option<String>>,
}

pub(super) fn generate(
    spec: &WireDartOutputSpec,
    config: &GeneratorWireDartInternalConfig,
) -> anyhow::Result<WireDartOutputText> {
    let merged_code = generate_merged_code(spec);
    let text = generate_text_from_merged_code(
        config,
        merged_code.map(|code, target| code.all_code(target, &config.dart_output_class_name_pack)),
    )?;
    Ok(WireDartOutputText { text })
}

fn generate_merged_code(spec: &WireDartOutputSpec) -> Acc<WireDartOutputCode> {
    let mut merged_code = Acc::<Vec<WireDartOutputCode>>::default();
    let mut add = |section_name: &str, item: &Acc<Vec<WireDartOutputCode>>| {
        merged_code += section_header_comment(section_name, item);
        merged_code += item.clone();
    };

    add("boilerplate", &spec.misc.boilerplate);
    add(
        "api_impl_normal_functions",
        &Acc::new_common(spec.misc.api_impl_normal_functions.clone()),
    );
    add("extra_functions", &spec.misc.extra_functions);
    add("wire_class", &spec.misc.wire_class);
    add("rust2dart", &spec.rust2dart.inner);
    add("dart2rust", &spec.dart2rust.inner);

    merged_code.map(|code, _| code.into_iter().fold(Default::default(), |a, b| a + b))
}

fn generate_text_from_merged_code(
    config: &GeneratorWireDartInternalConfig,
    core_code: Acc<String>,
) -> anyhow::Result<Acc<Option<String>>> {
    Ok(generate_text_respecting_web_flag(
        core_code,
        config.web_enabled,
    ))
}
