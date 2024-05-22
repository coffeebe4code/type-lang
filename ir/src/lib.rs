use ast::*;
use cranelift_codegen::entity::EntityRef;
use cranelift_codegen::ir::entities::FuncRef;
use cranelift_codegen::ir::function::DisplayFunction;
use cranelift_codegen::ir::types::*;
use cranelift_codegen::ir::AbiParam;
use cranelift_codegen::ir::{Function, InstBuilder, Signature, UserFuncName, Value};
use cranelift_codegen::isa::CallConv;
use cranelift_codegen::settings;
use cranelift_codegen::verifier::verify_function;
use cranelift_frontend::*;
use cranelift_frontend::{FunctionBuilder, FunctionBuilderContext};
use perror::*;
use symtable::*;
use token::*;

pub struct IRSource {
    package: u32,
    fname: u32,
    variables: u32,
    scope: SymTable,
}

impl IRSource {
    pub fn new(package: u32, scope: SymTable) -> Self {
        IRSource {
            package,
            fname: 0,
            variables: 0,
            scope,
        }
    }
    pub fn handle_decl(
        &mut self,
        op: &InnerDecl,
        builder: &mut FunctionBuilder,
    ) -> ResultFir<Variable> {
        let result = self.add_var();
        builder.declare_var(result, I64);
        let temp = self.recurse(&op.expr, builder).unwrap();
        // todo:: optimization: not all paths need declare var if value is only ever read. or something similar, this statement is in the same ballpark, but might not be totally correct
        let x = builder.use_var(temp);

        self.scope.table.insert(
            op.identifier.into_symbol().val.slice.to_string(),
            temp.as_u32(),
        );
        builder.def_var(temp, x);
        Ok(temp)
    }
    pub fn handle_invoke(
        &mut self,
        op: &Invoke,
        builder: &mut FunctionBuilder,
    ) -> ResultFir<Variable> {
        let mut args: Vec<Value> = vec![];
        if op.args.is_some() {
            args = op
                .args
                .as_ref()
                .unwrap()
                .iter()
                .map(|x| {
                    let result = self.recurse(&x, builder).unwrap();
                    return builder.use_var(result).clone();
                })
                .collect::<Vec<Value>>();
        }
        // todo:: get this correct with funcref. on how to get this from slt?
        let call = builder.ins().call(FuncRef::from_u32(0), args.as_slice());
        let result = self.add_var();
        builder.declare_var(result, I64);
        builder.def_var(result, builder.inst_results(call)[0]);
        Ok(result)
    }
    pub fn handle_block(
        &mut self,
        op: &Block,
        builder: &mut FunctionBuilder,
    ) -> ResultFir<Variable> {
        let temp: Vec<Variable> = op
            .exprs
            .iter()
            .map(|x| {
                return self.recurse(&x, builder).unwrap();
            })
            .collect();
        Ok(*temp.last().unwrap())
    }
    pub fn handle_ret(&mut self, op: &RetOp, builder: &mut FunctionBuilder) -> ResultFir<Variable> {
        let temp = self.recurse(&op.expr, builder).unwrap();
        let arg = builder.use_var(temp);
        builder.ins().return_(&[arg]);
        Ok(temp)
    }
    pub fn handle_sym(&self, op: &Symbol) -> ResultFir<Variable> {
        Ok(Variable::from_u32(
            *self.scope.table.get(&op.val.slice).unwrap(),
        ))
    }
    pub fn handle_num(
        &mut self,
        num: &Number,
        builder: &mut FunctionBuilder,
    ) -> ResultFir<Variable> {
        let result = self.add_var();
        builder.declare_var(result, I64);
        let temp = builder
            .ins()
            .iconst(I64, num.val.slice.parse::<i64>().unwrap());
        builder.def_var(result, temp);
        Ok(result)
    }
    pub fn handle_bin(
        &mut self,
        num: &BinOp,
        builder: &mut FunctionBuilder,
    ) -> ResultFir<Variable> {
        let result = self.add_var();
        builder.declare_var(result, I64);
        let left = self.recurse(&num.left, builder).unwrap();
        let right = self.recurse(&num.right, builder).unwrap();
        let arg1 = builder.use_var(left);
        let arg2 = builder.use_var(right);
        let temp = match num.op.token {
            Token::Plus => builder.ins().iadd(arg1, arg2),
            Token::Dash => builder.ins().isub(arg1, arg2),
            Token::Asterisk => builder.ins().imul(arg1, arg2),
            Token::Slash => builder.ins().udiv(arg1, arg2),
            _ => panic!("invalid binary operand"),
        };
        builder.def_var(result, temp);
        Ok(result)
    }
    pub fn recurse(&mut self, expr: &Expr, builder: &mut FunctionBuilder) -> ResultFir<Variable> {
        match expr {
            Expr::Block(op) => self.handle_block(&op, builder),
            Expr::Invoke(op) => self.handle_invoke(&op, builder),
            Expr::BinOp(op) => self.handle_bin(&op, builder),
            Expr::RetOp(op) => self.handle_ret(&op, builder),
            Expr::Number(op) => self.handle_num(&op, builder),
            Expr::InnerDecl(op) => self.handle_decl(&op, builder),
            Expr::Symbol(op) => self.handle_sym(&op),
            _ => panic!("developer error unexpected expression {:?}", expr),
        }
    }
    pub fn begin(&mut self, func_def: FuncDecl) -> Function {
        let mut ctx = FunctionBuilderContext::new();
        let mut sig = Signature::new(CallConv::SystemV);
        let name = UserFuncName::user(self.package, self.fname);
        // todo:: types need to be worked out, params and returns defined
        if let Some(val) = func_def.args {
            val.iter()
                .for_each(|_x| sig.params.push(AbiParam::new(I64)));
        }
        sig.returns.push(AbiParam::new(I64));
        let mut func = Function::with_name_signature(name, sig);
        self.fname += 1;
        let mut builder = FunctionBuilder::new(&mut func, &mut ctx);
        let root_block = builder.create_block();
        builder.append_block_params_for_function_params(root_block);
        builder.switch_to_block(root_block);
        let _result = self.recurse(&func_def.block, &mut builder);
        builder.seal_block(root_block);
        builder.finalize();
        func
    }
    pub fn get_ir(self, func: &Function) -> Result<DisplayFunction> {
        let flags = settings::Flags::new(settings::builder());
        let res = verify_function(func, &flags);
        match res {
            Err(error) => panic!("get_ir: {}", error),
            _ => Ok(func.display()),
        }
    }
    pub fn add_var(&mut self) -> Variable {
        let temp = Variable::new(usize::try_from(self.variables).unwrap());
        self.variables += 1;
        temp
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use lexer::*;
    #[test]
    fn it_should_build_ret_5() {
        let func_def = FuncDecl::new(
            None,
            Lexeme {
                token: Token::Const,
                span: 0..3,
                slice: "const".to_string(),
            },
            expr!(
                Symbol,
                Lexeme {
                    token: Token::Symbol,
                    span: 4..5,
                    slice: "x".to_string()
                }
            ),
            None,
            expr!(
                Sig,
                Some(expr!(
                    ValueType,
                    Lexeme {
                        token: Token::USize,
                        span: 6..7,
                        slice: "usize".to_string(),
                    }
                )),
                None,
                None,
                None
            ),
            expr!(
                Block,
                vec![expr!(
                    RetOp,
                    Lexeme {
                        token: Token::Return,
                        span: 8..10,
                        slice: "return".to_string()
                    },
                    expr!(
                        Number,
                        Lexeme {
                            token: Token::Number,
                            span: 6..7,
                            slice: "5".to_string()
                        }
                    )
                )]
            ),
            None,
        );
        let mut fir = IRSource::new(0, SymTable::new());
        let result = fir.begin(func_def);
        /*
         * function u0:0() -> i64 system_v
         *  {
         *      block0:
         *      v0 = iconst.i64 5
         *      return v0  ; v0 = 5
         *  }
         */
        assert_eq!(format!("{}", fir.get_ir(&result).unwrap()), "function u0:0() -> i64 system_v {\nblock0:\n    v0 = iconst.i64 5\n    return v0  ; v0 = 5\n}\n");
    }
}
