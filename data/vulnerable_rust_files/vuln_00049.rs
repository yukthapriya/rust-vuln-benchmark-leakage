// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2019-0037
// CVE: CVE-2019-25054
// CWE: N/A
// Crate: pnet
// Type: unknown
// Severity: medium
// Source: https://crates.io/crates/pnet
// Extraction Method: repo_search
// GitHub Repo: libpnet/libpnet
// Commit: 49428d3
// File: pnet_macros/src/decorator.rs


format!(
             "/// Populates a {name}Packet using a {name} structure
              #[inline]
             #[cfg_attr(feature = \"clippy\", allow(used_underscore_binding))]
              pub fn populate(&mut self, packet: &{name}) {{
                  let _self = self;
                  {set_fields}
             let s = format!(
                 "
                 impl<'a> ::pnet_macros_support::packet::PacketSize for {name}<'a> {{
                    #[cfg_attr(feature = \"clippy\", allow(used_underscore_binding))]
                     fn packet_size(&self) -> usize {{
                         let _self = self;
                         {size}
             fn packet{u_mut}<'p>(&'p {mut_} self) -> &'p {mut_} [u8] {{ &{mut_} self.packet[..] }}
 
             #[inline]
            #[cfg_attr(feature = \"clippy\", allow(used_underscore_binding))]
             fn payload{u_mut}<'p>(&'p {mut_} self) -> &'p {mut_} [u8] {{
                 let _self = self;
                 {pre}
             let s = format!(
                 "
         impl<'p> ::core::fmt::Debug for {packet}<'p> {{
            #[cfg_attr(feature = \"clippy\", allow(used_underscore_binding))]
             fn fmt(&self, fmt: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {{
                 let _self = self;
                 write!(fmt,
         "{mutators}
                     /// Set the value of the {name} field.
                     #[inline]
                    #[allow(trivial_numeric_casts)]
                    #[cfg_attr(feature = \"clippy\", allow(used_underscore_binding))]
                     pub fn set_{name}(&mut self, val: {ty_str}) {{
                         use pnet_macros_support::packet::PrimitiveValues;
                         let _self = self;
         "{accessors}
                         /// Get the value of the {name} field
                         #[inline]
                        #[allow(trivial_numeric_casts)]
                        #[cfg_attr(feature = \"clippy\", allow(used_underscore_binding))]
                         pub fn get_{name}(&self) -> {ty_str} {{
                             {ctor}
                         }}
             *accessors = format!("{accessors}
                                     /// Get the value of the {name} field (copies contents)
                                     #[inline]
                                    #[allow(trivial_numeric_casts, unused_parens, unused_braces)]
                                    #[cfg_attr(feature = \"clippy\", allow(used_underscore_binding))]
                                     pub fn get_{name}(&self) -> Vec<{inner_ty_str}> {{
                                         use core::cmp::min;
                                         let _self = self;
             "{mutators}
                                 /// Set the value of the {name} field (copies contents)
                                 #[inline]
                                #[allow(trivial_numeric_casts)]
                                #[cfg_attr(feature = \"clippy\", allow(used_underscore_binding))]
                                 pub fn set_{name}(&mut self, vals: &[{inner_ty_str}]) {{
                                     let mut _self = self;
                                     let current_offset = {co};
         *accessors = format!("{accessors}
                                 /// Get the raw &[u8] value of the {name} field, without copying
                                 #[inline]
                                #[allow(trivial_numeric_casts)]
                                #[cfg_attr(feature = \"clippy\", allow(used_underscore_binding))]
                                 pub fn get_{name}_raw(&self) -> &[u8] {{
                                     use core::cmp::min;
                                     let _self = self;
         *mutators = format!("{mutators}
                                 /// Get the raw &mut [u8] value of the {name} field, without copying
                                 #[inline]
                                #[allow(trivial_numeric_casts)]
                                #[cfg_attr(feature = \"clippy\", allow(used_underscore_binding))]
                                 pub fn get_{name}_raw_mut(&mut self) -> &mut [u8] {{
                                     use core::cmp::min;
                                     let _self = self;
                     "{mutators}
                 /// Set the value of the {name} field.
                 #[inline]
                #[allow(trivial_numeric_casts)]
                #[cfg_attr(feature = \"clippy\", allow(used_underscore_binding))]
                 pub fn set_{name}(&mut self, vals: &Vec<{inner_ty_str}>) {{
                     use pnet_macros_support::packet::PrimitiveValues;
                     let _self = self;
                     "{accessors}
                     /// Get the value of the {name} field
                     #[inline]
                    #[allow(trivial_numeric_casts)]
                    #[cfg_attr(feature = \"clippy\", allow(used_underscore_binding))]
                     pub fn get_{name}(&self) -> Vec<{inner_ty_str}> {{
                         let _self = self;
                         let length = {packet_length};
             *accessors = format!("{accessors}
                                 /// Get the value of the {name} field (copies contents)
                                 #[inline]
                                #[allow(trivial_numeric_casts)]
                                #[cfg_attr(feature = \"clippy\", allow(used_underscore_binding))]
                                 pub fn get_{name}(&self) -> Vec<{inner_ty_str}> {{
                                     use pnet_macros_support::packet::FromPacket;
                                     use core::cmp::min;
 
                                 /// Get the value of the {name} field as iterator
                                 #[inline]
                                #[allow(trivial_numeric_casts)]
                                #[cfg_attr(feature = \"clippy\", allow(used_underscore_binding))]
                                 pub fn get_{name}_iter(&self) -> {inner_ty_str}Iterable {{
                                     use core::cmp::min;
                                     let _self = self;
             *mutators = format!("{mutators}
                                 /// Set the value of the {name} field (copies contents)
                                 #[inline]
                                #[allow(trivial_numeric_casts)]
                                #[cfg_attr(feature = \"clippy\", allow(used_underscore_binding))]
                                 pub fn set_{name}(&mut self, vals: &[{inner_ty_str}]) {{
                                     use pnet_macros_support::packet::PacketSize;
                                     let _self = self;
     let mutator = if let Some(struct_name) = inner {
         format!(
             "#[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = \"clippy\", allow(used_underscore_binding))]
     fn set_{name}(_self: &mut {struct_name}, val: {ty}) {{
         let co = {co};
         {operations}
         format!(
             "{comment}
     #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = \"clippy\", allow(used_underscore_binding))]
     pub fn set_{name}(&mut self, val: {ty}) {{
         let _self = self;
         let co = {co};
 
     format!(
         "#[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = \"clippy\", allow(used_underscore_binding))]
     fn set_{name}(_self: &mut {struct_name}, val: {ty}, offset: usize) {{
         let co = {co} + offset;
         {operations}
     let accessor = if let Some(struct_name) = inner {
         format!(
             "#[inline(always)]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = \"clippy\", allow(used_underscore_binding))]
         fn get_{name}(_self: &{struct_name}) -> {ty} {{
             let co = {co};
             {operations}
         format!(
             "{comment}
         #[inline]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = \"clippy\", allow(used_underscore_binding))]
         pub fn get_{name}(&self) -> {ty} {{
             let _self = self;
             let co = {co};
 
     format!(
         "#[inline(always)]
    #[allow(trivial_numeric_casts, unused_parens)]
    #[cfg_attr(feature = \"clippy\", allow(used_underscore_binding))]
     fn get_{name}(_self: &{struct_name}, offset: usize) -> {ty} {{
         let co = {co} + offset;
         {operations}