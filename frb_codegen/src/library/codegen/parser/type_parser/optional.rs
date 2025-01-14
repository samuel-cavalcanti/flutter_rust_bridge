use crate::codegen::ir::ty::delegate::IrTypeDelegate;
use crate::codegen::ir::ty::optional::IrTypeOptional;
use crate::codegen::ir::ty::IrType;
use crate::codegen::ir::ty::IrType::{
    Boxed, DartFn, DartOpaque, Delegate, Dynamic, EnumRef, GeneralList, Optional, Ownership,
    Primitive, PrimitiveList, Record, RustAutoOpaque, RustOpaque, StructRef, Unencodable,
};
use crate::codegen::parser::type_parser::unencodable::ArgsRefs::Generic;
use crate::codegen::parser::type_parser::unencodable::SplayedSegment;
use crate::codegen::parser::type_parser::TypeParserWithContext;
use anyhow::bail;
use quote::ToTokens;
use syn::TypePath;

impl<'a, 'b, 'c> TypeParserWithContext<'a, 'b, 'c> {
    pub(crate) fn parse_type_path_data_optional(
        &mut self,
        type_path: &TypePath,
        last_segment: &SplayedSegment,
    ) -> anyhow::Result<Option<IrType>> {
        Ok(Some(match last_segment {
            // This will stop the whole generator and tell the users, so we do not care about testing it
            // frb-coverage:ignore-start
            ("Option", Some(Generic([Optional(_)]))) => bail!(
                "Nested optionals without indirection are not supported. {}",
                type_path.to_token_stream()
            ),
            // frb-coverage:ignore-end
            ("Option", Some(Generic([inner]))) => Optional(match inner {
                StructRef(..)
                | EnumRef(..)
                | RustAutoOpaque(..)
                | RustOpaque(..)
                | DartOpaque(..)
                | DartFn(..)
                | Primitive(..)
                | Record(..)
                | Delegate(IrTypeDelegate::PrimitiveEnum(..)) => {
                    IrTypeOptional::new_with_boxed_wrapper(inner.clone())
                }
                Delegate(IrTypeDelegate::Time(..)) => {
                    IrTypeOptional::new_with_boxed_wrapper(inner.clone())
                }
                PrimitiveList(_) | GeneralList(_) | Boxed(_) | Dynamic(_) | Ownership(_)
                | Unencodable(_) | Delegate(_) => IrTypeOptional::new(inner.clone()),
                // frb-coverage:ignore-start
                Optional(_) => unreachable!(),
                // frb-coverage:ignore-end
            }),

            _ => return Ok(None),
        }))
    }
}
