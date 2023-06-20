use quote::{format_ident, quote};

#[proc_macro]
pub fn pg_migration(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    impl_pg_migration(item.into()).into()
}

fn impl_pg_migration(item: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    let ident = syn::parse2::<syn::Ident>(item).expect("Must be an ident");

    let up = format_ident!("{ident}_up");
    let down = format_ident!("{ident}_down");

    let migration = &ident.to_string()[1..];
    let path_up = format!("migrations/{ident}/up.sql");
    let path_down = format!("migrations/{ident}/down.sql");

    quote! {
        #[pgrx::pg_extern]
        pub fn #up() -> core::result::Result<(), pgrx::spi::Error> {
            if !Spi::get_one_with_args::<bool>(
                r#"SELECT EXISTS (SELECT 1 FROM "_migration" WHERE "name" = $1 LIMIT 1);"#,
                vec![(
                    PgBuiltInOids::TEXTOID.oid(),
                    #migration.into_datum(),
                )],
            )
            .is_ok_and(|o| !o.is_some_and(|b| !b))
            {
                Spi::run(include_str!(#path_up))?;
            }
            Ok(())
        }

        #[pgrx::pg_extern]
        pub fn #down() -> core::result::Result<(), pgrx::spi::Error> {
            if Spi::get_one_with_args::<bool>(
                r#"SELECT EXISTS (SELECT 1 FROM "_migration" WHERE "name" = $1 LIMIT 1);"#,
                vec![(
                    PgBuiltInOids::TEXTOID.oid(),
                    #migration.into_datum(),
                )],
            )
            .is_ok_and(|o| o.is_some_and(|b| b))
            {
                Spi::run(include_str!(#path_down))?;
            }
            Ok(())
        }
    }
}
