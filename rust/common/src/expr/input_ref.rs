use std::convert::TryFrom;

use risingwave_pb::expr::expr_node::RexNode;
use risingwave_pb::expr::{expr_node::Type, ExprNode as ProstExprNode};

use crate::array::{ArrayRef, DataChunk};
use crate::error::{ErrorCode, Result, RwError};
use crate::expr::Expression;
use crate::types::{build_from_prost as type_build_from_prost, DataType, DataTypeRef};

/// `InputRefExpression` references to a column in input relation
pub struct InputRefExpression {
    return_type: DataTypeRef,
    idx: usize,
}

impl Expression for InputRefExpression {
    fn return_type(&self) -> &dyn DataType {
        &*self.return_type
    }

    fn return_type_ref(&self) -> DataTypeRef {
        self.return_type.clone()
    }

    fn eval(&mut self, input: &DataChunk) -> Result<ArrayRef> {
        Ok(input.column_at(self.idx)?.array())
    }
}

impl InputRefExpression {
    pub fn new(return_type: DataTypeRef, idx: usize) -> Self {
        InputRefExpression { return_type, idx }
    }

    pub fn eval_immut(&self, input: &DataChunk) -> Result<ArrayRef> {
        Ok(input.column_at(self.idx)?.array())
    }
}

impl<'a> TryFrom<&'a ProstExprNode> for InputRefExpression {
    type Error = RwError;

    fn try_from(prost: &'a ProstExprNode) -> Result<Self> {
        ensure!(prost.get_expr_type() == Type::InputRef);

        let ret_type = type_build_from_prost(prost.get_return_type())?;
        if let RexNode::InputRef(input_ref_node) = prost.get_rex_node() {
            Ok(Self {
                return_type: ret_type,
                idx: input_ref_node.column_idx as usize,
            })
        } else {
            Err(RwError::from(ErrorCode::NotImplementedError(
                "expects a input ref node".to_string(),
            )))
        }
    }
}
