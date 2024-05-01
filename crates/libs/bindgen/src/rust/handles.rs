use super::*;
use metadata::{AsRow, HasAttributes};

pub fn writer(writer: &Writer, def: metadata::TypeDef) -> TokenStream {
    if writer.sys {
        gen_sys_handle(writer, def)
    } else {
        gen_win_handle(writer, def)
    }
}

pub fn gen_sys_handle(writer: &Writer, def: metadata::TypeDef) -> TokenStream {
    let ident = to_ident(def.name());
    match def.underlying_type() {
        metadata::Type::ISize if writer.std => quote! {
            pub type #ident = *mut core::ffi::c_void;
        },
        underlying_type => {
            let signature = writer.type_default_name(&underlying_type);

            quote! {
                pub type #ident = #signature;
            }
        }
    }
}

pub fn gen_win_handle(writer: &Writer, def: metadata::TypeDef) -> TokenStream {
    let name = def.name();
    let ident = to_ident(name);
    let underlying_type = def.underlying_type();
    let signature = writer.type_default_name(&underlying_type);
    let is_invalid = if underlying_type.is_pointer() {
        quote! {
            impl #ident {
                pub fn is_invalid(&self) -> bool {
                    self.0.is_null()
                }
            }
        }
    } else {
        let invalid = metadata::type_def_invalid_values(def);

        if !invalid.is_empty() {
            let invalid = invalid.iter().map(|value| {
                let literal = Literal::i64_unsuffixed(*value);

                if *value < 0 && underlying_type.is_unsigned() {
                    quote! { self.0 == #literal as _ }
                } else {
                    quote! { self.0 == #literal }
                }
            });
            quote! {
                impl #ident {
                    pub fn is_invalid(&self) -> bool {
                        #(#invalid)||*
                    }
                }
            }
        } else {
            quote! {}
        }
    };

    let free = if let Some(function) = free_function(def) {
        if is_invalid.is_empty() {
            // TODO: https://github.com/microsoft/win32metadata/issues/1891
            quote! {}
        } else {
            let name = to_ident(function.name());
            let signature = metadata::method_def_signature(def.namespace(), function, &[]);

            // BCryptCloseAlgorithmProvider has an unused trailing parameter.
            let tail = if signature.params.len() > 1 {
                quote! { , 0 }
            } else {
                quote! {}
            };

            let result = if signature.return_type == metadata::Type::Void {
                quote! {}
            } else {
                quote! { _ = }
            };

            quote! {
                impl windows_core::Free for #ident {
                    unsafe fn free(&mut self) {
                        if !self.is_invalid() {
                            #result #name(*self #tail);
                        }
                    }
                }
            }
        }
    } else {
        quote! {}
    };

    let mut tokens = quote! {
        #[repr(transparent)]
        #[derive(PartialEq, Eq)]
        pub struct #ident(pub #signature);
        #is_invalid
        #free
        impl Default for #ident {
            fn default() -> Self {
                unsafe { core::mem::zeroed() }
            }
        }
        impl Clone for #ident {
            fn clone(&self) -> Self {
                *self
            }
        }
        impl Copy for #ident {}
        impl core::fmt::Debug for #ident {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.debug_tuple(#name).field(&self.0).finish()
            }
        }
        impl windows_core::TypeKind for #ident {
            type TypeKind = windows_core::CopyType;
        }
    };

    if let Some(dependency) = type_def_usable_for(def) {
        let type_name = dependency.type_name();
        let mut dependency = writer.namespace(type_name.namespace());
        dependency.push_str(type_name.name());

        tokens.combine(&quote! {
            impl windows_core::CanInto<#dependency> for #ident {}
            impl From<#ident> for #dependency {
                fn from(value: #ident) -> Self {
                    Self(value.0)
                }
            }
        });
    }

    tokens
}

fn type_def_usable_for(row: metadata::TypeDef) -> Option<metadata::TypeDef> {
    if let Some(attribute) = row.find_attribute("AlsoUsableForAttribute") {
        if let Some((_, metadata::Value::String(name))) = attribute.args().first() {
            return row.reader().get_type_def(row.namespace(), name.as_str()).next();
        }
    }
    None
}

fn free_function(def: metadata::TypeDef) -> Option<metadata::MethodDef> {
    if let Some(attribute) = def.find_attribute("RAIIFreeAttribute") {
        // TODO: https://github.com/microsoft/win32metadata/issues/1892
        if matches!(def.name(), "COMPRESSOR_HANDLE" | "WSAEVENT") {
            return None;
        }

        if let Some((_, metadata::Value::String(name))) = attribute.args().first() {
            if let Some((method, _)) = def.reader().get_method_def(def.namespace(), name.as_str()).next() {
                return Some(method);
            }
        }
    }

    None
}
