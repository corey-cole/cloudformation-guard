use crate::rules::values::Value;
use crate::rules::exprs::QueryPart;
use crate::errors::Error;

pub(crate) type Result<R> = std::result::Result<R, Error>;

#[derive(Debug, Clone, PartialEq, Copy)]
pub(crate) enum Status {
    PASS,
    FAIL,
    SKIP,
}

pub(crate) trait EvaluationContext {
    fn resolve_variable(&self,
                        variable: &str) -> Result<Vec<&Value>>;

    fn rule_status(&self, rule_name: &str) -> Result<Status>;

    fn report_status(&self, msg: String, from: Option<Value>, to: Option<Value>, status: Status) {}
}

pub(crate) trait QueryResolver  {
    fn resolve<'r>(&self,
                   index: usize,
                   query: &[QueryPart<'_>],
                   var_resolver: &dyn EvaluationContext,
                   context: &'r Value) -> Result<Vec<&'r Value>>;
}

pub(crate) trait Evaluate {
    fn evaluate(&self,
                context: &Value,
                var_resolver: &dyn EvaluationContext) -> Result<Status>;
}
