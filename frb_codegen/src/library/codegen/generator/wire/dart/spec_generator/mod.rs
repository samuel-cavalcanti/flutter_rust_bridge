use crate::codegen::generator::wire::dart::spec_generator::api2wire::WireDartOutputSpecApi2wire;
use crate::codegen::generator::wire::dart::spec_generator::base::WireDartGeneratorContext;
use crate::codegen::generator::wire::dart::spec_generator::misc::WireDartOutputSpecMisc;
use crate::codegen::generator::wire::dart::spec_generator::wire2api::WireDartOutputSpecWire2api;
use crate::codegen::ir::pack::IrPackComputedCache;
use serde::Serialize;
use std::path::PathBuf;

pub mod api2wire;
pub(crate) mod base;
mod function;
pub(crate) mod misc;
pub(crate) mod output_code;
pub mod wire2api;

#[derive(Clone, Serialize)]
pub(crate) struct WireDartOutputSpec {
    pub(super) misc: WireDartOutputSpecMisc,
    pub(super) wire2api: WireDartOutputSpecWire2api,
    pub(super) api2wire: WireDartOutputSpecApi2wire,
}

pub(crate) fn generate(
    context: WireDartGeneratorContext,
    c_file_content: &str,
    api_dart_actual_output_paths: &[PathBuf],
) -> anyhow::Result<WireDartOutputSpec> {
    let cache = IrPackComputedCache::compute(context.ir_pack);
    Ok(WireDartOutputSpec {
        misc: misc::generate(
            context,
            &cache,
            c_file_content,
            api_dart_actual_output_paths,
        )?,
        wire2api: wire2api::generate(context, &cache),
        api2wire: api2wire::generate(context, &cache),
    })
}
