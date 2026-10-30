// REAL FIXED CODE
// RustSec ID: RUSTSEC-2023-0042 (FIXED)
// Crate: ouroboros
// Type: unknown (FIXED)
// Extraction Method: repo_search
// GitHub Repo: joshua-maros/ouroboros
// Commit: 7c08e05
// File: ouroboros_macro/src/generate/with_mut.rs


let mut mut_field_assignments = Vec::new();
     let mut lifetime_idents = Vec::new();
     // I don't think the reverse is necessary but it does make the expanded code more uniform.
    for (index, field) in info.fields.iter().rev().enumerate() {
         let field_name = &field.name;
        let original_field_type = &field.typ;
        let lifetime = format_ident!("this{}", index);
        let field_type = replace_this_with_lifetime(quote! { #original_field_type }, lifetime.clone());
         if field.field_type == FieldType::Tail {
             mut_fields.push(quote! { #visibility #field_name: &'outer_borrow mut #field_type });
             mut_field_assignments.push(quote! { #field_name: &mut this.#field_name });
            if uses_this_lifetime(quote! { #original_field_type }) {
                lifetime_idents.push(lifetime.clone());
            }
         } else if field.field_type == FieldType::Borrowed {
             let ass = quote! { #field_name: unsafe {
                 ::ouroboros::macro_help::change_lifetime(
             let lt = Lifetime::new(&format!("'{}", lifetime), Span::call_site());
             mut_fields.push(quote! { #visibility #field_name: &#lt #field_type });
             mut_field_assignments.push(ass);
            lifetime_idents.push(lifetime.clone());
         } else if field.field_type == FieldType::BorrowedMut {
             // Add nothing because we cannot borrow something that has already been mutably
             // borrowed.