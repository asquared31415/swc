// Reexports
pub use swc_common::{chain, plugin::Serialized, DUMMY_SP};
pub mod ast {
    pub use swc_atoms::*;
    pub use swc_ecma_ast::*;
    pub use swc_ecma_visit::*;
}

#[cfg(target_arch = "wasm32")]
pub use swc_plugin_macro::plugin_module;
#[cfg(target_arch = "wasm32")]
mod allocation;
#[cfg(target_arch = "wasm32")]
pub mod memory {
    pub use crate::allocation::*;
}
