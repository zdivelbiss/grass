#![allow(unused_imports, unused_variables, dead_code, unused_mut)]

use std::collections::BTreeMap;

use codemap::{Span, Spanned};

use crate::{
    args::CallArgs,
    atrule::Mixin,
    builtin::Builtin,
    common::{Identifier, QuoteKind},
    error::SassResult,
    parse::Parser,
    scope::Scope,
    value::{SassFunction, SassMap, Value},
};

mod color;
mod list;
mod map;
mod math;
mod meta;
mod selector;
mod string;

#[derive(Debug, Default)]
pub(crate) struct Module(pub Scope);

#[derive(Debug, Default)]
pub(crate) struct Modules(BTreeMap<Identifier, Module>);

impl Modules {
    pub fn insert(&mut self, name: Identifier, module: Module) {
        self.0.insert(name, module);
    }

    pub fn get(&self, name: Identifier, span: Span) -> SassResult<&Module> {
        match self.0.get(&name) {
            Some(v) => Ok(v),
            None => Err((
                format!(
                    "There is no module with the namespace \"{}\".",
                    name.as_str()
                ),
                span,
            )
                .into()),
        }
    }
}

impl Module {
    pub fn get_var(&self, name: Spanned<Identifier>) -> SassResult<&Value> {
        if name.node.as_str().starts_with('-') {
            return Err((
                "Private members can't be accessed from outside their modules.",
                name.span,
            )
                .into());
        }

        match self.0.vars.get(&name.node) {
            Some(v) => Ok(v),
            None => Err(("Undefined variable.", name.span).into()),
        }
    }

    pub fn insert_builtin_var(&mut self, name: &'static str, value: Value) {
        self.0.vars.insert(name.into(), value);
    }

    pub fn get_fn(&self, name: Spanned<Identifier>) -> SassResult<Option<SassFunction>> {
        if name.node.as_str().starts_with('-') {
            return Err((
                "Private members can't be accessed from outside their modules.",
                name.span,
            )
                .into());
        }

        Ok(self.0.functions.get(&name.node).cloned())
    }

    pub fn var_exists(&self, name: Identifier) -> bool {
        !name.as_str().starts_with('-') && self.0.var_exists(name)
    }

    pub fn mixin_exists(&self, name: Identifier) -> bool {
        !name.as_str().starts_with('-') && self.0.mixin_exists(name)
    }

    pub fn insert_builtin(
        &mut self,
        name: &'static str,
        function: fn(CallArgs, &mut Parser<'_>) -> SassResult<Value>,
    ) {
        let ident = name.into();
        self.0
            .functions
            .insert(ident, SassFunction::Builtin(Builtin::new(function), ident));
    }

    pub fn functions(&self) -> SassMap {
        SassMap::new_with(
            self.0
                .functions
                .iter()
                .filter(|(key, _)| !key.as_str().starts_with('-'))
                .map(|(key, value)| {
                    (
                        Value::String(key.to_string(), QuoteKind::Quoted),
                        Value::FunctionRef(value.clone()),
                    )
                })
                .collect::<Vec<(Value, Value)>>(),
        )
    }

    pub fn variables(&self) -> SassMap {
        SassMap::new_with(
            self.0
                .vars
                .iter()
                .filter(|(key, _)| !key.as_str().starts_with('-'))
                .map(|(key, value)| {
                    (
                        Value::String(key.to_string(), QuoteKind::Quoted),
                        value.clone(),
                    )
                })
                .collect::<Vec<(Value, Value)>>(),
        )
    }

    pub const fn new_from_scope(scope: Scope) -> Self {
        Module(scope)
    }
}

pub(crate) fn declare_module_color() -> Module {
    let mut module = Module::default();
    color::declare(&mut module);
    module
}

pub(crate) fn declare_module_list() -> Module {
    let mut module = Module::default();
    list::declare(&mut module);
    module
}

pub(crate) fn declare_module_map() -> Module {
    let mut module = Module::default();
    map::declare(&mut module);
    module
}

pub(crate) fn declare_module_math() -> Module {
    let mut module = Module::default();
    math::declare(&mut module);
    module
}

pub(crate) fn declare_module_meta() -> Module {
    let mut module = Module::default();
    meta::declare(&mut module);
    module
}

pub(crate) fn declare_module_selector() -> Module {
    let mut module = Module::default();
    selector::declare(&mut module);
    module
}

pub(crate) fn declare_module_string() -> Module {
    let mut module = Module::default();
    string::declare(&mut module);
    module
}
