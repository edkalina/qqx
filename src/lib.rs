pub mod field_filter;
pub mod filter_ops;

use std::borrow::Cow;

use async_graphql::{registry, Type};
pub use qqx_macros::Qqx;

use field_filter::{FieldFilter, WithFilterOps};
use filter_ops::{BaseFilterOps, OptionFilterOps, StringFilterOps};

macro_rules! impl_field_filter {
    ($name: ident, $ty: ty, $ops: ty) => {
        impl Type for FieldFilter<$ty> {
            fn type_name() -> Cow<'static, str> {
                ::std::borrow::Cow::Borrowed(stringify!($name))
            }

            fn create_type_info(registry: &mut registry::Registry) -> String {
                Self::generic_create_type_info(registry, stringify!($name))
            }
        }

        impl WithFilterOps for $ty {
            type Ops = $ops;
        }

        impl Type for FieldFilter<Option<$ty>> {
            fn type_name() -> Cow<'static, str> {
                ::std::borrow::Cow::Borrowed(concat!("Optional", stringify!($name)))
            }

            fn create_type_info(registry: &mut registry::Registry) -> String {
                Self::generic_create_type_info(registry, concat!("Optional", stringify!($name)))
            }
        }

        impl WithFilterOps for Option<$ty> {
            type Ops = OptionFilterOps<$ops>;
        }
    };
}

impl_field_filter!(StringFieldFilter, String, StringFilterOps);
impl_field_filter!(UuidFieldFilter, uuid::Uuid, BaseFilterOps<uuid::Uuid>);
impl_field_filter!(
    DateFieldFilter,
    chrono::NaiveDate,
    BaseFilterOps<chrono::NaiveDate>
);
