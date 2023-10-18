pub use attr_value::*;
pub use element_attr::*;
pub use element_composite_attr::*;
pub use element_event::*;
pub use element_type::*;
pub use r#macro::*;

use crate::{attrs_trait_define, composite_attrs_trait_define};

mod attr_value;
mod element_attr;
mod element_composite_attr;
mod element_event;
mod element_type;
mod r#macro;

pub trait BevyDioxusAppExt {
    fn register_elements_type(&mut self) -> &mut Self;
}

pub type DioxusAttributeDescription = (&'static str, Option<&'static str>, bool);
