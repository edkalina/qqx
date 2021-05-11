use std::collections::BTreeMap;

use crate::filter_ops::FilterOps;
use async_graphql::{registry, InputType, InputValueError, Type, Value};

#[derive(Debug, Clone)]
pub struct FieldFilter<T: InputType + WithFilterOps> {
    ops: T::Ops,
}

impl<T> FieldFilter<T>
where
    Self: Type,
    T: InputType + WithFilterOps,
{
    pub fn generic_create_type_info(registry: &mut registry::Registry, name: &str) -> String {
        registry.create_type::<Self, _>(|registry| registry::MetaType::InputObject {
            name: name.to_owned(),
            description: None,
            input_fields: {
                let mut fields = async_graphql::indexmap::IndexMap::new();
                T::Ops::creat_fields_info(registry, &mut fields);
                fields
            },
            visible: None,
        })
    }
}

impl<T> InputType for FieldFilter<T>
where
    Self: Type,
    T: InputType + WithFilterOps,
{
    fn parse(value: Option<Value>) -> async_graphql::InputValueResult<Self> {
        if let Some(Value::Object(obj)) = value {
            Ok(Self {
                ops: T::Ops::parse::<Self>(&obj)?,
            })
        } else {
            Err(InputValueError::expected_type(value.unwrap_or_default()))
        }
    }

    fn to_value(&self) -> Value {
        let mut map = BTreeMap::new();
        self.ops.to_value(&mut map);
        Value::Object(map)
    }

    fn federation_fields() -> Option<String> {
        let mut res = Vec::new();
        T::Ops::add_federation_fields(&mut res);
        Some(format!("{{ {} }}", res.join(" ")))
    }
}

pub trait WithFilterOps {
    type Ops: std::fmt::Debug + Clone + Send + Sync + FilterOps;
}
