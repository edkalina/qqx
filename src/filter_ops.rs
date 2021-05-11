use std::collections::BTreeMap;

use async_graphql::{indexmap::IndexMap, registry, InputType, InputValueError, Name, Type, Value};

pub trait FilterOps: Sized {
    fn to_value(&self, map: &mut BTreeMap<Name, Value>);
    fn creat_fields_info(
        registry: &mut registry::Registry,
        fields: &mut IndexMap<String, registry::MetaInputValue>,
    );
    fn parse<U: InputType>(obj: &BTreeMap<Name, Value>) -> Result<Self, InputValueError<U>>;
    fn add_federation_fields(fields: &mut Vec<String>);
}

#[derive(Debug, Clone)]
pub struct BaseFilterOps<T: InputType> {
    eq: Option<T>,
    neq: Option<T>,
}

impl<T: InputType> FilterOps for BaseFilterOps<T> {
    fn to_value(&self, map: &mut BTreeMap<Name, Value>) {
        if let Some(ref value) = self.eq {
            map.insert(Name::new("eq"), InputType::to_value(value));
        }
        if let Some(ref value) = self.neq {
            map.insert(Name::new("neq"), InputType::to_value(value));
        }
    }

    fn creat_fields_info(
        registry: &mut registry::Registry,
        fields: &mut IndexMap<String, registry::MetaInputValue>,
    ) {
        fields.insert(
            "eq".to_owned(),
            registry::MetaInputValue {
                name: "eq",
                description: None,
                ty: <Option<T> as Type>::create_type_info(registry),
                default_value: None,
                validator: None,
                visible: None,
                is_secret: false,
            },
        );
        fields.insert(
            "neq".to_owned(),
            registry::MetaInputValue {
                name: "neq",
                description: None,
                ty: <Option<T> as Type>::create_type_info(registry),
                default_value: None,
                validator: None,
                visible: None,
                is_secret: false,
            },
        );
    }

    fn parse<U: InputType>(obj: &BTreeMap<Name, Value>) -> Result<Self, InputValueError<U>> {
        Ok(Self {
            eq: InputType::parse(obj.get("eq").cloned()).map_err(InputValueError::propagate)?,
            neq: InputType::parse(obj.get("neq").cloned()).map_err(InputValueError::propagate)?,
        })
    }

    fn add_federation_fields(fields: &mut Vec<String>) {
        fields.push("eq".to_owned());
        fields.push("neq".to_owned());
    }
}

#[derive(Debug, Clone)]
pub struct StringFilterOps {
    ops: BaseFilterOps<String>,
    like: Option<String>,
}

impl FilterOps for StringFilterOps {
    fn to_value(&self, map: &mut BTreeMap<Name, Value>) {
        self.ops.to_value(map);
        if let Some(ref value) = self.like {
            map.insert(Name::new("like"), InputType::to_value(value));
        }
    }

    fn creat_fields_info(
        registry: &mut registry::Registry,
        fields: &mut IndexMap<String, registry::MetaInputValue>,
    ) {
        BaseFilterOps::<String>::creat_fields_info(registry, fields);
        fields.insert(
            "like".to_owned(),
            registry::MetaInputValue {
                name: "like",
                description: None,
                ty: <Option<String> as Type>::create_type_info(registry),
                default_value: None,
                validator: None,
                visible: None,
                is_secret: false,
            },
        );
    }

    fn parse<U: InputType>(obj: &BTreeMap<Name, Value>) -> Result<Self, InputValueError<U>> {
        Ok(Self {
            ops: BaseFilterOps::<String>::parse::<U>(obj)?,
            like: InputType::parse(obj.get("like").cloned()).map_err(InputValueError::propagate)?,
        })
    }

    fn add_federation_fields(fields: &mut Vec<String>) {
        BaseFilterOps::<String>::add_federation_fields(fields);
        fields.push("like".to_owned());
    }
}

#[derive(Debug, Clone)]
pub struct OptionFilterOps<T: FilterOps> {
    ops: T,
    is_null: Option<bool>,
}

impl<T: FilterOps> FilterOps for OptionFilterOps<T> {
    fn to_value(&self, map: &mut BTreeMap<Name, Value>) {
        self.ops.to_value(map);
        if let Some(ref value) = self.is_null {
            map.insert(Name::new("isNull"), InputType::to_value(value));
        }
    }

    fn creat_fields_info(
        registry: &mut registry::Registry,
        fields: &mut IndexMap<String, registry::MetaInputValue>,
    ) {
        T::creat_fields_info(registry, fields);
        fields.insert(
            "isNull".to_owned(),
            registry::MetaInputValue {
                name: "isNull",
                description: None,
                ty: <Option<bool> as Type>::create_type_info(registry),
                default_value: None,
                validator: None,
                visible: None,
                is_secret: false,
            },
        );
    }

    fn parse<U: InputType>(obj: &BTreeMap<Name, Value>) -> Result<Self, InputValueError<U>> {
        Ok(Self {
            ops: T::parse::<U>(obj)?,
            is_null: InputType::parse(obj.get("isNull").cloned())
                .map_err(InputValueError::propagate)?,
        })
    }

    fn add_federation_fields(fields: &mut Vec<String>) {
        T::add_federation_fields(fields);
        fields.push("isNull".to_owned());
    }
}
