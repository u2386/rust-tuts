use core::panic;

use proc_macro2::{Ident, TokenStream};
use syn::{
    Data, DataStruct, DeriveInput, Field, Fields, FieldsNamed, GenericArgument, Path, Type,
    TypePath, __private::quote::quote,
};

pub struct BuilderContext {
    name: Ident,
    fields: Vec<Fd>,
}

pub struct Fd {
    name: Ident,
    ty: Type,
    optional: bool,
}

impl From<Field> for Fd {
    fn from(f: Field) -> Self {
        let (optional, ty) = get_option_inner(&f.ty);
        Self {
            name: f.ident.unwrap(), 
            ty: ty.to_owned(),
            optional: optional,
        }
    }
}

impl From<DeriveInput> for BuilderContext {
    fn from(input: DeriveInput) -> Self {
        let name = input.ident;
        let feilds = if let Data::Struct(DataStruct {
            fields: Fields::Named(FieldsNamed { named, .. }),
            ..
        }) = input.data
        {
            named
        } else {
            panic!("unsuppported data type");
        };

        let fds = feilds.into_iter().map(Fd::from).collect();
        Self { name, fields: fds }
    }
}

impl BuilderContext {
    pub fn render(&self) -> TokenStream {
        let name = &self.name;
        let builder_name = Ident::new(&format!("{}Builder", name), name.span());

        let optionized_fields = self.gen_optionized_fields();
        let methods = self.gen_methods();
        let assignments = self.gen_assigns();

        quote! {
            #[derive(Debug, Default)]
            struct #builder_name {
                #(#optionized_fields,)*
            }

            impl #builder_name {
                #(#methods)*

                pub fn build(mut self) -> Result<#name, &'static str> {
                    Ok(#name {
                        #(#assignments,)*
                    })
                }
            }

            impl #name {
                fn builder() -> #builder_name {
                    Default::default()
                }
            }
        }
    }

    pub fn gen_optionized_fields(&self) -> Vec<TokenStream> {
        self.fields
            .iter()
            .map(|Fd { name, ty, .. }| quote! { #name: std::option::Option<#ty> })
            .collect()
    }

    pub fn gen_methods(&self) -> Vec<TokenStream> {
        self.fields
            .iter()
            .map(|Fd { name, ty, .. }| {
                quote! {
                    pub fn #name(mut self, v: impl Into<#ty>) -> Self {
                        self.#name = Some(v.into());
                        self
                    }
                }
            })
            .collect()
    }

    pub fn gen_assigns(&self) -> Vec<TokenStream> {
        self.fields
            .iter()
            .map(|Fd { name, optional, .. }| {
                if *optional {
                    return quote! {
                        #name: self.#name.take()
                    };
                }

                quote! {
                    #name: self.#name.take().ok_or(concat!(stringify!(#name), " needs to be set!"))?
                }
            })
            .collect()
    }
}

fn get_option_inner(ty: &Type) -> (bool, &Type) {
    if let Type::Path(TypePath {
        path: Path { segments, .. },
        ..
    }) = ty
    {
        if let Some(v) = segments.iter().next() {
            if v.ident == "Option" {
                let t = match &v.arguments {
                    syn::PathArguments::AngleBracketed(a) => match a.args.iter().next() {
                        Some(GenericArgument::Type(t)) => t,
                        _ => panic!("not sure what to do with other GenericArgument"),
                    },
                    _ => panic!("not sure what to do with PathArguments"),
                };
                return (true, t);
            }
        }
    }
    (false, ty)
}
