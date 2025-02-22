use crate::compiler::op::{BinaryOp, UnaryOp};
use crate::io_ctx::Type21;

pub trait SyntaxVisitor {
    type ExprResult;
    type StmtResult;
    type DeclResult;
    type Error;

    fn visit_ident(&mut self, ident: &str) -> Result<Self::ExprResult, Self::Error>;
    fn visit_lit_int(&mut self, value: i32) -> Self::ExprResult;
    fn visit_lit_float(&mut self, value: f32) -> Self::ExprResult;
    fn visit_lit_bool(&mut self, value: bool) -> Self::ExprResult;
    fn visit_uop(
        &mut self,
        op: UnaryOp,
        operand: Self::ExprResult
    ) -> Result<Self::ExprResult, Self::Error>;
    fn visit_bin_op(
        &mut self,
        op: BinaryOp,
        lhs: Self::ExprResult,
        rhs: Self::ExprResult
    ) -> Result<Self::ExprResult, Self::Error>;
    fn visit_assign(
        &mut self,
        name: &str,
        value: Self::ExprResult
    ) -> Result<Self::ExprResult, Self::Error>;
    fn visit_assign2(
        &mut self,
        names: &[&str],
        value: Self::ExprResult
    ) -> Result<Self::ExprResult, Self::Error>;
    fn visit_type_cast(
        &mut self,
        ty: Type21,
        operand: Self::ExprResult
    ) -> Result<Self::ExprResult, Self::Error>;
    fn visit_call(
        &mut self,
        name: &str,
        args: &[Self::ExprResult]
    ) -> Result<Self::ExprResult, Self::Error>;

    fn visit_expr_stmt(&mut self) -> Self::StmtResult;
    fn fin_expr_stmt(&mut self, expr: Self::ExprResult) -> Result<Self::StmtResult, Self::Error>;
    fn visit_decl_stmt(&mut self) -> Result<Self::StmtResult, Self::Error>;
    fn fin_decl_stmt(&mut self, decl: Self::DeclResult) -> Result<Self::StmtResult, Self::Error>;
    fn visit_if_stmt(
        &mut self,
        cond: Self::ExprResult
    ) -> Result<Self::StmtResult, Self::Error>;
    fn fin_if_stmt(
        &mut self,
        if_clause: Self::StmtResult,
        else_clause: Option<Self::StmtResult>
    ) -> Result<Self::StmtResult, Self::Error>;
    fn visit_while_stmt(
        &mut self,
        cond: Self::ExprResult
    ) -> Result<Self::StmtResult, Self::Error>;
    fn fin_while_stmt(
        &mut self,
        body: Self::StmtResult
    ) -> Result<Self::StmtResult, Self::Error>;
    fn visit_for_stmt(
        &mut self,
        init: Option<Self::ExprResult>,
        cond: Option<Self::ExprResult>,
        step: Option<Self::ExprResult>,
        body: Self::StmtResult
    ) -> Result<Self::StmtResult, Self::Error>;
    fn visit_break_stmt(&mut self) -> Result<Self::StmtResult, Self::Error>;
    fn visit_continue_stmt(&mut self) -> Result<Self::StmtResult, Self::Error>;
    fn visit_return_stmt(
        &mut self,
        value: Option<Self::ExprResult>
    ) -> Result<Self::StmtResult, Self::Error>;
    fn visit_block_stmt(
        &mut self,
        stmts: &[Self::StmtResult]
    ) -> Result<Self::StmtResult, Self::Error>;

    fn visit_var_decl(
        &mut self,
        ty: Option<Type21>,
        name: &str,
        init: Option<Self::ExprResult>
    ) -> Result<Self::DeclResult, Self::Error>;
    fn visit_func_decl(
        &mut self,
        ty: &[Type21],
        name: &str,
        params: &[(Type21, &str)],
        body: Option<Self::StmtResult>
    ) -> Result<Self::DeclResult, Self::Error>;
    fn visit_const_decl(
        &mut self,
        name: &str,
        init: Self::ExprResult
    ) -> Result<Self::DeclResult, Self::Error>;
}
