use darling::{ast::Data, FromDeriveInput};
use inflector::Inflector;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Error};

#[proc_macro_derive(Qqx, attributes(qqx, graphql))]
pub fn derive_qqx(input: TokenStream) -> TokenStream {
    let args = match args::Qqx::from_derive_input(&parse_macro_input!(input as DeriveInput)) {
        Ok(args) => args,
        Err(err) => return TokenStream::from(err.write_errors()),
    };
    match qqx_generate(&args) {
        Ok(expanded) => expanded,
        Err(err) => err.write_errors().into(),
    }
}

fn qqx_generate(args: &args::Qqx) -> utils::GeneratorResult<TokenStream> {
    let ident = &args.ident;
    let mod_ident = proc_macro2::Ident::new(&ident.to_string().to_snake_case(), ident.span());
    let filter_ident = proc_macro2::Ident::new(&format!("{}Filter", ident), ident.span());

    let s = match &args.data {
        Data::Struct(s) => s,
        _ => return Err(Error::new_spanned(&ident, "Qqx can only be applied to a struct.").into()),
    };

    let mut filter_fields = Vec::new();
    for field in &s.fields {
        let ident = field.ident.as_ref().unwrap();
        let ty = &field.ty;

        if field.skip {
            continue;
        }

        filter_fields.push(quote! {
            pub #ident: Option<FieldFilter<#ty>>
        });
    }

    Ok(quote! {
        pub mod #mod_ident {
            use qqx::field_filter::FieldFilter;
            use super::*;

            #[derive(Debug, Clone, async_graphql::InputObject)]
            pub struct #filter_ident {
                or: Option<Vec<#filter_ident>>,
                #(#filter_fields),*
            }
        }
    }
    .into())
}

mod utils {
    use proc_macro2::TokenStream;
    #[derive(thiserror::Error, Debug)]
    pub enum GeneratorError {
        #[error("{0}")]
        Syn(#[from] syn::Error),

        #[error("{0}")]
        Darling(#[from] darling::Error),
    }

    impl GeneratorError {
        pub fn write_errors(self) -> TokenStream {
            match self {
                GeneratorError::Syn(err) => err.to_compile_error(),
                GeneratorError::Darling(err) => err.write_errors(),
            }
        }
    }

    pub type GeneratorResult<T> = std::result::Result<T, GeneratorError>;
}

mod args {
    use darling::ast::Data;
    use darling::util::Ignored;
    use darling::{FromDeriveInput, FromField};
    use syn::{
        Attribute, Generics, Ident, Type, Visibility,
    };

    #[derive(FromDeriveInput)]
    #[darling(attributes(qqx))]
    pub struct Qqx {
        pub ident: Ident,
        pub generics: Generics,
        pub attrs: Vec<Attribute>,
        pub data: Data<Ignored, QqxField>,

        #[darling(default)]
        pub name: Option<String>,
    }

    #[derive(FromField)]
    #[darling(attributes(qqx))]
    pub struct QqxField {
        pub ident: Option<Ident>,
        pub ty: Type,
        pub vis: Visibility,
        pub attrs: Vec<Attribute>,

        #[darling(default)]
        pub skip: bool,
    }
}
