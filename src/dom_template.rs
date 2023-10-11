use dioxus::core::{Template, TemplateAttribute, TemplateNode};

#[derive(Clone, Debug)]
pub enum DomTemplateAttribute {
    Static {
        name: &'static str,
        value: String,
        namespace: Option<&'static str>,
    },
    Dynamic {
        id: usize,
    },
}

#[derive(Clone, Debug)]
pub enum DomTemplateNode {
    Element {
        tag: &'static str,
        namespace: Option<&'static str>,
        attrs: Vec<DomTemplateAttribute>,
        children: Vec<DomTemplateNode>,
    },
    Text {
        text: String,
    },

    Dynamic {
        id: usize,
    },
    DynamicText {
        id: usize,
    },
}

impl From<&'_ TemplateNode<'_>> for DomTemplateNode {
    fn from(value: &'_ TemplateNode<'_>) -> Self {
        match *value {
            TemplateNode::Element {
                tag,
                namespace,
                attrs,
                children,
            } => DomTemplateNode::Element {
                tag: unsafe { std::mem::transmute::<&str, &'static str>(tag) },
                namespace: namespace
                    .map(|n| unsafe { std::mem::transmute::<&str, &'static str>(n) }),
                attrs: attrs
                    .into_iter()
                    .map(|n| match n {
                        TemplateAttribute::Static {
                            name,
                            value,
                            namespace,
                        } => DomTemplateAttribute::Static {
                            name: unsafe { std::mem::transmute::<&str, &'static str>(name) },
                            namespace: namespace
                                .map(|n| unsafe { std::mem::transmute::<&str, &'static str>(n) }),
                            value: value.to_string(),
                        },
                        TemplateAttribute::Dynamic { id } => {
                            DomTemplateAttribute::Dynamic { id: *id }
                        }
                    })
                    .collect(),
                children: children.iter().map(|n| n.into()).collect(),
            },
            TemplateNode::Text { text } => DomTemplateNode::Text {
                text: text.to_string(),
            },
            TemplateNode::Dynamic { id } => DomTemplateNode::Dynamic { id },
            TemplateNode::DynamicText { id } => DomTemplateNode::DynamicText { id },
        }
    }
}

impl From<Template<'_>> for DomTemplate {
    fn from(value: Template<'_>) -> Self {
        Self {
            name: value.name.to_string(),
            roots: value.roots.into_iter().map(|n| n.into()).collect(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct DomTemplate {
    pub name: String,
    pub roots: Vec<DomTemplateNode>,
    //    pub node_paths: Vec<Vec<u8>>,
    //    pub attr_paths: Vec<Vec<u8>>,
}
