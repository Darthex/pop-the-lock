use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input};

/*
This macro adds both the `Component` and `Reflect` traits to a struct,
and also adds a `register` function that registers the type with the Bevy app.
This is useful for making it easy to create components that can be used in
Bevy's ECS system and also be reflected for use in editor tools or serialization.
*/

#[proc_macro_attribute]
pub fn serialize(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let name = &input.ident;

    quote! {
        #[derive(bevy::prelude::Component, bevy::reflect::Reflect, Default)]
        #[reflect(Component)]  // ← just Component, not the full path
        #input

        impl #name {
            pub fn register(app: &mut bevy::prelude::App) {
                app.register_type::<#name>();
            }
        }
    }
        .into()
}
