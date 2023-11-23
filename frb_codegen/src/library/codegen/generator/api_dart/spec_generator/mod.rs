use crate::codegen::generator::api_dart::internal_config::GeneratorApiDartInternalConfig;
use crate::codegen::generator::api_dart::spec_generator::base::{
    ApiDartGenerator, ApiDartGeneratorContext,
};
use crate::codegen::generator::api_dart::spec_generator::class::ApiDartGeneratedClass;
use crate::codegen::generator::api_dart::spec_generator::function::ApiDartGeneratedFunction;
use crate::codegen::ir::func::IrFuncOwnerInfo;
use crate::codegen::ir::pack::{IrPack, IrPackComputedCache};
use crate::library::codegen::generator::api_dart::spec_generator::class::ty::ApiDartGeneratorClassTrait;
use anyhow::Result;
use itertools::Itertools;
use serde::Serialize;

pub(crate) mod base;
pub(crate) mod class;
pub(crate) mod function;
pub(crate) mod info;
pub(crate) mod misc;

#[derive(Serialize)]
pub(crate) struct ApiDartOutputSpec {
    pub funcs: Vec<ApiDartGeneratedFunction>,
    pub classes: Vec<ApiDartGeneratedClass>,
    pub needs_freezed: bool,
}

pub(crate) fn generate(
    ir_pack: &IrPack,
    config: &GeneratorApiDartInternalConfig,
) -> Result<ApiDartOutputSpec> {
    let cache = IrPackComputedCache::compute(ir_pack);
    let context = ApiDartGeneratorContext { ir_pack, config };

    let funcs = (ir_pack.funcs.iter())
        .filter(|f| f.owner == IrFuncOwnerInfo::Function)
        .map(|f| function::generate(f, context))
        .collect_vec();

    let classes = cache
        .distinct_types
        .iter()
        .filter_map(|ty| ApiDartGenerator::new(ty.clone(), context).generate_class())
        .collect_vec();

    let needs_freezed = classes.iter().any(|c| c.needs_freezed);

    Ok(ApiDartOutputSpec {
        funcs,
        classes,
        needs_freezed,
    })
}
