pub use attr_value::*;
pub use element_attr::*;
pub use element_composite_attr::*;
pub use element_type::*;
pub use element_event::*;

mod element_composite_attr;
mod element_attr;
mod element_type;
mod attr_value;
mod element_event;

pub type DioxusAttributeDescription = (&'static str, Option<&'static str>, bool);
