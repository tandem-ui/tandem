use paperclip_proto::ast::pc::Node;
use paperclip_parser::pc::parser::parse as parse_pc;
use paperclip_proto::ast;
use paperclip_proto::ast::all::Expression;
use paperclip_proto::ast_mutate::{
    mutation_result, AppendChild, ExpressionInserted, MutationResult,
};

use crate::ast::{all::Visitor, all::VisitorResult};

impl Visitor<Vec<MutationResult>> for AppendChild {
    fn visit_document(
        &mut self,
        expr: &mut ast::pc::Document,
    ) -> VisitorResult<Vec<MutationResult>> {
        if expr.get_id() == &self.parent_id {
            let child = parse_pc(&self.child_source, &expr.checksum())
                .expect("Unable to parse child source for AppendChild");
            let child = child.body.get(0).unwrap();

            expr.body.push(child.clone());
            return VisitorResult::Return(vec![mutation_result::Inner::ExpressionInserted(
                ExpressionInserted {
                    id: child.get_id().to_string(),
                },
            )
            .get_outer()]);
        }
        VisitorResult::Continue
    }
    fn visit_element(&mut self, expr: &mut ast::pc::Element) -> VisitorResult<Vec<MutationResult>> {
        if expr.get_id() == &self.parent_id {
            let child = parse_pc(&self.child_source, &expr.checksum())
                .expect("Unable to parse child source for AppendChild");
            let child: Node = child.body.get(0).unwrap().clone().try_into().unwrap();

            expr.body.push(child.clone());

            return VisitorResult::Return(vec![mutation_result::Inner::ExpressionInserted(
                ExpressionInserted {
                    id: child.get_id().to_string(),
                },
            )
            .get_outer()]);
        }
        VisitorResult::Continue
    }
}