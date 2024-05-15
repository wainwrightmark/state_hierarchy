use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{self};

#[proc_macro_derive(MavericRoot)]
pub fn maveric_root_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_maveric_root(&ast)
}

fn impl_maveric_root(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;

    quote!(
        #[automatically_derived]
        impl MavericRoot for #name {
            type ContextParam<'w,'s> = <<Self as maveric::prelude::MavericRootChildren>::Context as maveric::prelude::MavericContext>::Wrapper<'w, 's>;

            fn get_context<'w1, 's1, 'w, 's>(
                param: bevy::ecs::system::StaticSystemParam<'w, 's, Self::ContextParam<'w1, 's1>>,
            ) -> <<Self as MavericRootChildren>::Context as maveric::prelude::MavericContext>::Wrapper<'w,'s> {
                param.into_inner()
            }
        }

    ).into()
}

#[proc_macro_derive(MavericContextCompound)]
pub fn maveric_context_derive(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();

    let name = &ast.ident;

    let data_struct: &syn::DataStruct = match &ast.data {
        syn::Data::Struct(s) => s,
        syn::Data::Enum(_) => panic!("`MavericContext` can only be derived for structs"),
        syn::Data::Union(_) => panic!("`MavericContext` can only be derived for unions"),
    };

    let fields_named = match &data_struct.fields {
        syn::Fields::Named(fd) => fd,
        syn::Fields::Unnamed(_) => {
            panic!("`MavericContext` can only be derived for structs with named fields")
        }
        syn::Fields::Unit => {
            panic!("`MavericContext` can only be derived for structs with named fields")
        }
    };

    let wrapper_name = format_ident!("{name}Wrapper");
    let visibility = &ast.vis;

    let wrapper_fields = fields_named.named.iter().map(|field| {
        let field_name = field.ident.clone().unwrap();
        let field_type = &field.ty;
        quote!(pub #field_name: <#field_type as maveric::maveric_context::MavericContext>::Wrapper<'w,'s> )
    });

    let has_changed = fields_named.named.iter().map(|field| {
        let field_name = field.ident.clone().unwrap();
        quote!(maveric::has_changed::HasChanged::has_changed(&self.#field_name) )
    });

    quote!(

        #[derive(bevy::ecs::system::SystemParam)]
        #visibility struct #wrapper_name<'w, 's>{
            #(#wrapper_fields),*
        }


        #[automatically_derived]
        impl maveric::maveric_context::MavericContext for #name {
            type Wrapper<'w, 's> = #wrapper_name<'w, 's>;
        }

        #[automatically_derived]
        impl<'w, 's> maveric::has_changed::HasChanged for #wrapper_name<'w, 's>
        {
            fn has_changed(&self) -> bool {
                #(#has_changed)||*
            }
        }

    )
    .into()
}

#[proc_macro_derive(MavericContextResource)]
pub fn maveric_context_resource_derive(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();

    let name = &ast.ident;

    quote!(

        #[automatically_derived]
        impl MavericContext for #name {
            type Wrapper<'w, 's> = Res<'w, #name>;
        }
    )
    .into()
}

