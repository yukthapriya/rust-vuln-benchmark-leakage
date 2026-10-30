// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2023-0042
// CVE: N/A
// CWE: N/A
// Crate: ouroboros
// Type: unknown
// Severity: medium
// Source: https://crates.io/crates/ouroboros
// Extraction Method: repo_search
// GitHub Repo: joshua-maros/ouroboros
// Commit: 7c08e05
// File: ouroboros_macro/src/generate/with_mut.rs


let mut mut_field_assignments = Vec::new();
     let mut lifetime_idents = Vec::new();
     // I don't think the reverse is necessary but it does make the expanded code more uniform.
    for field in info.fields.iter().rev() {
         let field_name = &field.name;
        let field_type = &field.typ;
        let lifetime = format_ident!("this{}", lifetime_idents.len());
        if uses_this_lifetime(quote! { #field_type }) || field.field_type == FieldType::Borrowed {
            lifetime_idents.push(lifetime.clone());
        }
        let field_type = replace_this_with_lifetime(quote! { #field_type }, lifetime.clone());
         if field.field_type == FieldType::Tail {
             mut_fields.push(quote! { #visibility #field_name: &'outer_borrow mut #field_type });
             mut_field_assignments.push(quote! { #field_name: &mut this.#field_name });
         } else if field.field_type == FieldType::Borrowed {
             let ass = quote! { #field_name: unsafe {
                 ::ouroboros::macro_help::change_lifetime(
             let lt = Lifetime::new(&format!("'{}", lifetime), Span::call_site());
             mut_fields.push(quote! { #visibility #field_name: &#lt #field_type });
             mut_field_assignments.push(ass);
         } else if field.field_type == FieldType::BorrowedMut {
             // Add nothing because we cannot borrow something that has already been mutably
             // borrowed.