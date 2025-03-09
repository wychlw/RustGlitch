use quote::ToTokens;
use std::{
    error::Error,
    fs::{read_to_string, write},
    path::PathBuf,
    process::Command,
};

use syn::{
    Expr, Ident,
    visit_mut::{self, VisitMut},
};
use syn::{File, parse_file};

use crate::{
    debug, display::syn_enum_display::fmt_expr, do_fuzz, error, register_nodetype
};

use super::{
    fuzz::Fuzzer,
    strategy::DoFuzzRes,
};

pub struct SynFuzzer {
    ast: Option<File>,
}

register_nodetype!(Expr);
register_nodetype!(Ident);

/*
pub trait VisitMut {


    fn visit_abi_mut(&mut self, i: &mut crate::Abi) {
        visit_abi_mut(self, i);
    }


    fn visit_angle_bracketed_generic_arguments_mut(
        &mut self,
        i: &mut crate::AngleBracketedGenericArguments,
    ) {
        visit_angle_bracketed_generic_arguments_mut(self, i);
    }


    fn visit_arm_mut(&mut self, i: &mut crate::Arm) {
        visit_arm_mut(self, i);
    }


    fn visit_assoc_const_mut(&mut self, i: &mut crate::AssocConst) {
        visit_assoc_const_mut(self, i);
    }


    fn visit_assoc_type_mut(&mut self, i: &mut crate::AssocType) {
        visit_assoc_type_mut(self, i);
    }


    fn visit_attr_style_mut(&mut self, i: &mut crate::AttrStyle) {
        visit_attr_style_mut(self, i);
    }


    fn visit_attribute_mut(&mut self, i: &mut crate::Attribute) {
        visit_attribute_mut(self, i);
    }


    fn visit_attributes_mut(&mut self, i: &mut Vec<crate::Attribute>) {
        for attr in i {
            self.visit_attribute_mut(attr);
        }
    }


    fn visit_bare_fn_arg_mut(&mut self, i: &mut crate::BareFnArg) {
        visit_bare_fn_arg_mut(self, i);
    }


    fn visit_bare_variadic_mut(&mut self, i: &mut crate::BareVariadic) {
        visit_bare_variadic_mut(self, i);
    }


    fn visit_bin_op_mut(&mut self, i: &mut crate::BinOp) {
        visit_bin_op_mut(self, i);
    }


    fn visit_block_mut(&mut self, i: &mut crate::Block) {
        visit_block_mut(self, i);
    }


    fn visit_bound_lifetimes_mut(&mut self, i: &mut crate::BoundLifetimes) {
        visit_bound_lifetimes_mut(self, i);
    }


    fn visit_captured_param_mut(&mut self, i: &mut crate::CapturedParam) {
        visit_captured_param_mut(self, i);
    }


    fn visit_const_param_mut(&mut self, i: &mut crate::ConstParam) {
        visit_const_param_mut(self, i);
    }


    fn visit_constraint_mut(&mut self, i: &mut crate::Constraint) {
        visit_constraint_mut(self, i);
    }


    fn visit_data_mut(&mut self, i: &mut crate::Data) {
        visit_data_mut(self, i);
    }


    fn visit_derive_input_mut(&mut self, i: &mut crate::DeriveInput) {
        visit_derive_input_mut(self, i);
    }


    fn visit_expr_mut(&mut self, i: &mut crate::Expr) {
        visit_expr_mut(self, i);
    }


    fn visit_field_mut(&mut self, i: &mut crate::Field) {
        visit_field_mut(self, i);
    }


    fn visit_field_mutability_mut(&mut self, i: &mut crate::FieldMutability) {
        visit_field_mutability_mut(self, i);
    }


    fn visit_field_pat_mut(&mut self, i: &mut crate::FieldPat) {
        visit_field_pat_mut(self, i);
    }


    fn visit_field_value_mut(&mut self, i: &mut crate::FieldValue) {
        visit_field_value_mut(self, i);
    }


    fn visit_fields_mut(&mut self, i: &mut crate::Fields) {
        visit_fields_mut(self, i);
    }


    fn visit_fields_named_mut(&mut self, i: &mut crate::FieldsNamed) {
        visit_fields_named_mut(self, i);
    }


    fn visit_fields_unnamed_mut(&mut self, i: &mut crate::FieldsUnnamed) {
        visit_fields_unnamed_mut(self, i);
    }


    fn visit_file_mut(&mut self, i: &mut crate::File) {
        visit_file_mut(self, i);
    }


    fn visit_fn_arg_mut(&mut self, i: &mut crate::FnArg) {
        visit_fn_arg_mut(self, i);
    }


    fn visit_foreign_item_mut(&mut self, i: &mut crate::ForeignItem) {
        visit_foreign_item_mut(self, i);
    }


    fn visit_foreign_item_fn_mut(&mut self, i: &mut crate::ForeignItemFn) {
        visit_foreign_item_fn_mut(self, i);
    }


    fn visit_foreign_item_macro_mut(&mut self, i: &mut crate::ForeignItemMacro) {
        visit_foreign_item_macro_mut(self, i);
    }


    fn visit_foreign_item_static_mut(&mut self, i: &mut crate::ForeignItemStatic) {
        visit_foreign_item_static_mut(self, i);
    }


    fn visit_foreign_item_type_mut(&mut self, i: &mut crate::ForeignItemType) {
        visit_foreign_item_type_mut(self, i);
    }


    fn visit_generic_argument_mut(&mut self, i: &mut crate::GenericArgument) {
        visit_generic_argument_mut(self, i);
    }


    fn visit_generic_param_mut(&mut self, i: &mut crate::GenericParam) {
        visit_generic_param_mut(self, i);
    }


    fn visit_generics_mut(&mut self, i: &mut crate::Generics) {
        visit_generics_mut(self, i);
    }
    fn visit_ident_mut(&mut self, i: &mut proc_macro2::Ident) {
        visit_ident_mut(self, i);
    }


    fn visit_impl_item_mut(&mut self, i: &mut crate::ImplItem) {
        visit_impl_item_mut(self, i);
    }


    fn visit_impl_item_const_mut(&mut self, i: &mut crate::ImplItemConst) {
        visit_impl_item_const_mut(self, i);
    }


    fn visit_impl_item_fn_mut(&mut self, i: &mut crate::ImplItemFn) {
        visit_impl_item_fn_mut(self, i);
    }


    fn visit_impl_item_macro_mut(&mut self, i: &mut crate::ImplItemMacro) {
        visit_impl_item_macro_mut(self, i);
    }


    fn visit_impl_item_type_mut(&mut self, i: &mut crate::ImplItemType) {
        visit_impl_item_type_mut(self, i);
    }


    fn visit_impl_restriction_mut(&mut self, i: &mut crate::ImplRestriction) {
        visit_impl_restriction_mut(self, i);
    }


    fn visit_index_mut(&mut self, i: &mut crate::Index) {
        visit_index_mut(self, i);
    }


    fn visit_item_mut(&mut self, i: &mut crate::Item) {
        visit_item_mut(self, i);
    }


    fn visit_label_mut(&mut self, i: &mut crate::Label) {
        visit_label_mut(self, i);
    }
    fn visit_lifetime_mut(&mut self, i: &mut crate::Lifetime) {
        visit_lifetime_mut(self, i);
    }


    fn visit_lifetime_param_mut(&mut self, i: &mut crate::LifetimeParam) {
        visit_lifetime_param_mut(self, i);
    }
    fn visit_lit_mut(&mut self, i: &mut crate::Lit) {
        visit_lit_mut(self, i);
    }


    fn visit_local_mut(&mut self, i: &mut crate::Local) {
        visit_local_mut(self, i);
    }


    fn visit_local_init_mut(&mut self, i: &mut crate::LocalInit) {
        visit_local_init_mut(self, i);
    }


    fn visit_macro_mut(&mut self, i: &mut crate::Macro) {
        visit_macro_mut(self, i);
    }


    fn visit_macro_delimiter_mut(&mut self, i: &mut crate::MacroDelimiter) {
        visit_macro_delimiter_mut(self, i);
    }


    fn visit_member_mut(&mut self, i: &mut crate::Member) {
        visit_member_mut(self, i);
    }


    fn visit_meta_mut(&mut self, i: &mut crate::Meta) {
        visit_meta_mut(self, i);
    }


    fn visit_meta_list_mut(&mut self, i: &mut crate::MetaList) {
        visit_meta_list_mut(self, i);
    }


    fn visit_meta_name_value_mut(&mut self, i: &mut crate::MetaNameValue) {
        visit_meta_name_value_mut(self, i);
    }


    fn visit_parenthesized_generic_arguments_mut(
        &mut self,
        i: &mut crate::ParenthesizedGenericArguments,
    ) {
        visit_parenthesized_generic_arguments_mut(self, i);
    }


    fn visit_pat_mut(&mut self, i: &mut crate::Pat) {
        visit_pat_mut(self, i);
    }

    fn visit_path_mut(&mut self, i: &mut crate::Path) {
        visit_path_mut(self, i);
    }


    fn visit_path_arguments_mut(&mut self, i: &mut crate::PathArguments) {
        visit_path_arguments_mut(self, i);
    }


    fn visit_path_segment_mut(&mut self, i: &mut crate::PathSegment) {
        visit_path_segment_mut(self, i);
    }


    fn visit_pointer_mutability_mut(&mut self, i: &mut crate::PointerMutability) {
        visit_pointer_mutability_mut(self, i);
    }


    fn visit_precise_capture_mut(&mut self, i: &mut crate::PreciseCapture) {
        visit_precise_capture_mut(self, i);
    }


    fn visit_predicate_lifetime_mut(&mut self, i: &mut crate::PredicateLifetime) {
        visit_predicate_lifetime_mut(self, i);
    }


    fn visit_predicate_type_mut(&mut self, i: &mut crate::PredicateType) {
        visit_predicate_type_mut(self, i);
    }


    fn visit_qself_mut(&mut self, i: &mut crate::QSelf) {
        visit_qself_mut(self, i);
    }


    fn visit_range_limits_mut(&mut self, i: &mut crate::RangeLimits) {
        visit_range_limits_mut(self, i);
    }


    fn visit_receiver_mut(&mut self, i: &mut crate::Receiver) {
        visit_receiver_mut(self, i);
    }


    fn visit_return_type_mut(&mut self, i: &mut crate::ReturnType) {
        visit_return_type_mut(self, i);
    }


    fn visit_signature_mut(&mut self, i: &mut crate::Signature) {
        visit_signature_mut(self, i);
    }
    fn visit_span_mut(&mut self, i: &mut proc_macro2::Span) {
        visit_span_mut(self, i);
    }


    fn visit_static_mutability_mut(&mut self, i: &mut crate::StaticMutability) {
        visit_static_mutability_mut(self, i);
    }


    fn visit_stmt_mut(&mut self, i: &mut crate::Stmt) {
        visit_stmt_mut(self, i);
    }


    fn visit_stmt_macro_mut(&mut self, i: &mut crate::StmtMacro) {
        visit_stmt_macro_mut(self, i);
    }


    fn visit_trait_bound_mut(&mut self, i: &mut crate::TraitBound) {
        visit_trait_bound_mut(self, i);
    }


    fn visit_trait_bound_modifier_mut(&mut self, i: &mut crate::TraitBoundModifier) {
        visit_trait_bound_modifier_mut(self, i);
    }


    fn visit_trait_item_mut(&mut self, i: &mut crate::TraitItem) {
        visit_trait_item_mut(self, i);
    }


    fn visit_trait_item_const_mut(&mut self, i: &mut crate::TraitItemConst) {
        visit_trait_item_const_mut(self, i);
    }


    fn visit_trait_item_fn_mut(&mut self, i: &mut crate::TraitItemFn) {
        visit_trait_item_fn_mut(self, i);
    }


    fn visit_trait_item_macro_mut(&mut self, i: &mut crate::TraitItemMacro) {
        visit_trait_item_macro_mut(self, i);
    }


    fn visit_trait_item_type_mut(&mut self, i: &mut crate::TraitItemType) {
        visit_trait_item_type_mut(self, i);
    }


    fn visit_type_mut(&mut self, i: &mut crate::Type) {
        visit_type_mut(self, i);
    }


    fn visit_un_op_mut(&mut self, i: &mut crate::UnOp) {
        visit_un_op_mut(self, i);
    }


    fn visit_use_glob_mut(&mut self, i: &mut crate::UseGlob) {
        visit_use_glob_mut(self, i);
    }


    fn visit_use_group_mut(&mut self, i: &mut crate::UseGroup) {
        visit_use_group_mut(self, i);
    }


    fn visit_use_name_mut(&mut self, i: &mut crate::UseName) {
        visit_use_name_mut(self, i);
    }


    fn visit_use_path_mut(&mut self, i: &mut crate::UsePath) {
        visit_use_path_mut(self, i);
    }


    fn visit_use_rename_mut(&mut self, i: &mut crate::UseRename) {
        visit_use_rename_mut(self, i);
    }


    fn visit_use_tree_mut(&mut self, i: &mut crate::UseTree) {
        visit_use_tree_mut(self, i);
    }


    fn visit_variadic_mut(&mut self, i: &mut crate::Variadic) {
        visit_variadic_mut(self, i);
    }


    fn visit_variant_mut(&mut self, i: &mut crate::Variant) {
        visit_variant_mut(self, i);
    }


    fn visit_vis_restricted_mut(&mut self, i: &mut crate::VisRestricted) {
        visit_vis_restricted_mut(self, i);
    }


    fn visit_visibility_mut(&mut self, i: &mut crate::Visibility) {
        visit_visibility_mut(self, i);
    }


    fn visit_where_clause_mut(&mut self, i: &mut crate::WhereClause) {
        visit_where_clause_mut(self, i);
    }


    fn visit_where_predicate_mut(&mut self, i: &mut crate::WherePredicate) {
        visit_where_predicate_mut(self, i);
    }
}
*/

impl VisitMut for SynFuzzer {
    fn visit_expr_mut(&mut self, node: &mut Expr) {
        debug!("Visiting Expr:\n {:#?}", node);
        match do_fuzz!(Expr, self, node) {
            Ok(DoFuzzRes::Success) => {}
            Ok(DoFuzzRes::NoStreatgy) => {
                visit_mut::visit_expr_mut(self, node);
            }
            Err(e) => {
                error!("Error: {:?}", e);
                panic!();
            }
        }
    }
    fn visit_ident_mut(&mut self, node: &mut Ident) {
        match do_fuzz!(Ident, self, node) {
            Ok(DoFuzzRes::Success) => {
                debug!("Fuzzing node success");
            }
            Ok(DoFuzzRes::NoStreatgy) => {
                syn::visit_mut::visit_ident_mut(self, node);
            }
            Err(e) => {
                error!("Error: {:?}", e);
                panic!();
            }
        }
    }
}

impl SynFuzzer {
    #[allow(unused)]
    pub fn new(file: &PathBuf, extra_args: &[String]) -> Result<Box<dyn Fuzzer>, Box<dyn Error>> {
        let code = read_to_string(file)?;
        let ast = parse_file(&code)?;
        let res = Self { ast: Some(ast) };
        Ok(Box::new(res))
    }
}

impl Fuzzer for SynFuzzer {
    fn replace(&mut self) -> Result<(), Box<dyn Error>> {
        let mut ast = self.ast.take().ok_or("No AST")?;
        self.visit_file_mut(&mut ast);
        self.ast = Some(ast);
        Ok(())
    }
    fn dump(&mut self, output: &PathBuf) -> Result<(), Box<dyn Error>> {
        let stream = self.ast.to_token_stream();
        let code = stream.to_string();
        write(output, &code)?;
        Command::new("rustfmt").arg(output).status()?;
        Ok(())
    }
}

