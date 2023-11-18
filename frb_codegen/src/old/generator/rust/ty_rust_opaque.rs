use std::borrow::Cow;

use crate::generator::rust::ty::*;
use crate::ir::*;
use crate::target::Acc;
use crate::type_rust_generator_struct;
use crate::utils::misc::BlockIndex;

use super::{ExternFuncCollector, NO_PARAMS};

type_rust_generator_struct!(TypeRustOpaqueGenerator, IrTypeRustOpaque);

impl TypeRustGeneratorTrait for TypeRustOpaqueGenerator<'_> {
    fn generate_allocate_funcs(&self) -> Acc<Option<CodeWithExternFunc>> {
        let rust_wire = self.ir.rust_wire_type(crate::target::Target::Io);

        Acc {
            io: Some(collector.generate(
                &format!("new_{}", self.ir.safe_ident()),
                NO_PARAMS,
                Some(&format!(
                    "{}{rust_wire}",
                    self.ir.rust_wire_modifier(crate::target::Target::Io),
                )),
                &format!("{rust_wire}::new_with_null_ptr()"),
                crate::target::Target::Io,
            )),
            ..Default::default()
        }
    }

    fn generate_related_funcs(&self) -> Acc<Option<CodeWithExternFunc>> {
        let mut generate_impl = |target| {
            [
                collector.generate(
                    &format!("drop_opaque_{}", self.ir.safe_ident()),
                    [("ptr: *const c_void", "")],
                    None,
                    &format!(
                        "unsafe {{Arc::<{}>::decrement_strong_count(ptr as _);}}",
                        self.ir.inner_rust
                    ),
                    target,
                ),
                collector.generate(
                    &format!("share_opaque_{}", self.ir.safe_ident()),
                    [("ptr: *const c_void", "")],
                    Some("*const c_void"),
                    &format!(
                        "unsafe {{Arc::<{}>::increment_strong_count(ptr as _); ptr}}",
                        self.ir.inner_rust
                    ),
                    target,
                ),
            ]
            .join("\n")
        };
        Acc {
            io: Some(generate_impl(crate::target::Target::Io)),
            wasm: Some(generate_impl(crate::target::Target::Wasm)),
            ..Default::default()
        }
    }
}
