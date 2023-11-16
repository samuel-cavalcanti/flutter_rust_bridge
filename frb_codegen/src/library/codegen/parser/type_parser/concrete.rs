use crate::codegen::ir::ty::dart_opaque::IrTypeDartOpaque;
use crate::codegen::ir::ty::delegate::{IrTypeDelegate, IrTypeDelegateTime};
use crate::codegen::ir::ty::dynamic::IrTypeDynamic;
use crate::codegen::ir::ty::IrType;
use crate::codegen::ir::ty::IrType::{DartOpaque, Delegate, Dynamic};
use crate::codegen::parser::type_parser::TypeParser;
use crate::codegen::parser::unencodable::ArgsRefs;
use syn::TypePath;

impl<'a> TypeParser<'a> {
    pub(crate) fn parse_type_path_data_concrete(
        &mut self,
        splayed_segments: &[(&str, Option<ArgsRefs>)],
    ) -> anyhow::Result<Option<IrType>> {
        Ok(Some(match splayed_segments {
            [("chrono", None), ("Duration", None)] => {
                Delegate(IrTypeDelegate::Time(IrTypeDelegateTime::Duration))
            }
            [("chrono", None), ("NaiveDateTime", None)] => {
                Delegate(IrTypeDelegate::Time(IrTypeDelegateTime::Naive))
            }
            [("uuid", None), ("Uuid", None)] => Delegate(IrTypeDelegate::Uuid),
            [("String", None)] => Delegate(IrTypeDelegate::String),
            [("Backtrace", None)] => Delegate(IrTypeDelegate::Backtrace),

            [("flutter_rust_bridge", None), ("DartAbi", None)] => Dynamic(IrTypeDynamic),

            [("flutter_rust_bridge", None), ("DartOpaque", None)] => {
                DartOpaque(IrTypeDartOpaque {})
            }

            _ => return Ok(None),
        }))
    }
}
