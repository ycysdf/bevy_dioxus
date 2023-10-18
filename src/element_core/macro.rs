#[macro_export]
macro_rules! attr_description_define {
   ($($attr:ident),*) =>{
      $(const $attr: $crate::DioxusAttributeDescription =
            $crate::all_attrs::$attr::ATTRIBUTE_DESCRIPTION;)*
   }
}

#[macro_export]
macro_rules! attrs_trait_define {
   ($name:ident;$index_start:expr;$($attr:ident),*) => {
        count_macro::count! {
            $(
            impl $crate::element_core::HasIndex for $crate::all_attrs::$attr{
               const INDEX: $crate::AttrIndex = $index_start+_int_;
            }
            )*
        }

        #[allow(non_upper_case_globals)]
        #[allow(non_camel_case_types)]
        pub trait $name {
            $crate::attr_description_define!($($attr),*);

            const ATTRS: &'static [&'static dyn $crate::ElementAttrUntyped] = &[
                $(&$crate::all_attrs::$attr,)*
            ];
         }


         paste::paste!{
            pub struct [<$name:camel Impl>];
            impl $name for [<$name:camel Impl>] {}
            pub trait [<_ $name:camel Extension>]<'a,M=()> :dioxus::prelude::HasAttributes<'a> +Sized {
              $(
                 fn $attr(self,value:impl dioxus::prelude::IntoAttributeValue<'a>) -> Self {
                    let d =  <[<$name:camel Impl>] as $name>::$attr;
                    self.push_attribute(d.0,d.1,value,d.2)
                 }
              )*
            }
         }
   }
}
#[macro_export]
macro_rules! composite_attrs_trait_define {
   ($name:ident;$($attr:ident),*) =>{

        #[allow(non_camel_case_types)]
        #[allow(non_upper_case_globals)]
        pub trait $name {
            $crate::attr_description_define!($($attr),*);

            const ATTRS: &'static [&'static dyn $crate::ElementCompositeAttrUntyped] = &[
                $(&$crate::all_attrs::$attr,)*
            ];
        }

        paste::paste!{
           pub struct [<$name:camel Impl>];
           impl $name for [<$name:camel Impl>] {}
           pub trait [<_ $name:camel Extension>]<'a,M=()> :dioxus::prelude::HasAttributes<'a> +Sized {
              $(
                 fn $attr(self,value:impl dioxus::prelude::IntoAttributeValue<'a>) -> Self {
                    let d =  <[<$name:camel Impl>] as $name>::$attr;
                    self.push_attribute(d.0,d.1,value,d.2)
                 }
              )*
           }
        }
   }
}

#[macro_export]
macro_rules! define_elements {
   (
      $(
         $(#[$m_attr:meta])*
            $name:ident {
            [attrs]
            $($attr:ident)*
            [composite_attrs]
            $($comopsite_attr:ident),*
         }
     )*
   ) => {

      impl $crate::BevyDioxusAppExt for bevy::app::App{
         fn register_elements_type(&mut self)-> &mut Self{
            self
                $(
                   .register_type::<$crate::elements::$name>()
                )*
         }
      }

     pub fn try_get_element_type(name: &str) -> Option<&'static dyn ElementTypeUnTyped> {
         match name {
            $(
               stringify!($name) => Some(&$crate::elements::$name),
                )*
                _ => None,
            }
      }

      pub fn get_element_type(name: &str) -> &'static dyn ElementTypeUnTyped {
         try_get_element_type(name).unwrap_or_else(|| panic!("No Found ElementType by {:#?}", name))
      }

      $(
        $crate::define_element!(
            $(#[$m_attr])*
            $name {
               [attrs]
               $($attr)*
               [composite_attrs]
               $($comopsite_attr),*
            }
         );
      )*
   }
}

#[macro_export]
macro_rules! define_element {
    (
         $(#[$m_attr:meta])*
         $name:ident {
            [attrs]
            $($attr:ident)*
            [composite_attrs]
            $($composite_attr:ident),*
         }
    ) => {
       #[allow(non_camel_case_types)]
        $( #[$m_attr] )*
        pub struct $name;

        paste::paste!{
            $crate::attrs_trait_define!([<$name:camel Attrs>];<$name as $crate::CommonAttrs>::ATTRS.len() as u8;
                $($attr),*
            );

            impl $crate::CommonAttrs for $name {}
            impl [<$name:camel Attrs>] for $name {}

            $crate::composite_attrs_trait_define!([<$name:camel CompositeAttrs>];
                $($composite_attr),*
            );

            impl $crate::CommonCompositeAttrs for $name {}
            impl [<$name:camel CompositeAttrs>] for $name {}

            impl $crate::ElementTypeBase for $name {
                const TAG_NAME: &'static str = stringify!($name);
                const ATTRS: &'static [&'static [&'static dyn $crate::ElementAttrUntyped]] = &[
                    <Self as $crate::CommonAttrs>::ATTRS,
                    <Self as $crate::[<$name:camel Attrs>]>::ATTRS,
                ];

                const COMPOSITE_ATTRS: &'static [&'static [&'static dyn $crate::ElementCompositeAttrUntyped]] = &[
                    <Self as $crate::CommonCompositeAttrs>::ATTRS,
                    <Self as $crate::[<$name:camel CompositeAttrs>]>::ATTRS,
                ];
            }

            pub trait [<$name:camel AttrsExtension>]<'a>: $crate::prelude::HasAttributes<'a> {}

            impl<'a, T: [<$name:camel AttrsExtension>]<'a>> $crate::[<_ $name:camel AttrsExtension>]<'a> for T {}
            impl<'a, T: [<$name:camel AttrsExtension>]<'a>> $crate::[<_ $name:camel CompositeAttrsExtension>]<'a> for T {}
            impl<'a, T: [<$name:camel AttrsExtension>]<'a>> $crate::_CommonAttrsExtension<'a,$name> for T {}
            impl<'a, T: [<$name:camel AttrsExtension>]<'a>> $crate::_CommonCompositeAttrsExtension<'a,$name> for T {}
        }
   }
}
