use next_binding::swc::core::{
    common::errors::HANDLER,
    ecma::{
        ast::*,
        atoms::JsWord,
        visit::{noop_visit_type, Visit},
    },
};

pub struct FontFunctionsCollector<'a> {
    pub font_loaders: &'a [JsWord],
    pub state: &'a mut super::State,
}

impl<'a> Visit for FontFunctionsCollector<'a> {
    noop_visit_type!();

    fn visit_import_decl(&mut self, import_decl: &ImportDecl) {
        if self.font_loaders.contains(&import_decl.src.value) {
            self.state
                .removeable_module_items
                .insert(import_decl.span.lo);
            for specifier in &import_decl.specifiers {
                match specifier {
                    ImportSpecifier::Named(ImportNamedSpecifier {
                        local, imported, ..
                    }) => {
                        self.state
                            .font_functions_in_allowed_scope
                            .insert(local.span.lo);

                        let function_name = if let Some(ModuleExportName::Ident(ident)) = imported {
                            ident.sym.clone()
                        } else {
                            local.sym.clone()
                        };
                        self.state.font_functions.insert(
                            local.to_id(),
                            super::FontFunction {
                                loader: import_decl.src.value.clone(),
                                function_name: Some(function_name),
                            },
                        );
                    }
                    ImportSpecifier::Default(ImportDefaultSpecifier { local, .. }) => {
                        self.state
                            .font_functions_in_allowed_scope
                            .insert(local.span.lo);
                        self.state.font_functions.insert(
                            local.to_id(),
                            super::FontFunction {
                                loader: import_decl.src.value.clone(),
                                function_name: None,
                            },
                        );
                    }
                    ImportSpecifier::Namespace(_) => {
                        HANDLER.with(|handler| {
                            handler
                                .struct_span_err(
                                    import_decl.span,
                                    "Font loaders can't have namespace imports",
                                )
                                .emit()
                        });
                    }
                }
            }
        }
    }
}
