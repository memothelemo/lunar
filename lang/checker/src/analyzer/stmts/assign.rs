use super::*;

#[allow(clippy::or_fun_call)]
impl Validate for hir::LocalAssign {
    type Output = ();

    fn validate<'a>(&self, analyzer: &mut Analyzer<'a>) -> Result<Self::Output, AnalyzeError> {
        for variable in self.variables.iter() {
            match (variable.expr.as_ref(), variable.explicit_type.as_ref()) {
                (None, Some(ty)) => {
                    return Err(AnalyzeError::NotDefined {
                        variable: variable.name.to_string(),
                        explicit_type: analyzer.type_description(ty),
                        span: variable.name_span,
                    })
                }
                (Some(a), Some(b)) => analyzer.resolve_type(
                    a,
                    b,
                    variable
                        .expr_source
                        .unwrap_or(variable.explicit_type.as_ref().unwrap().span()),
                )?,
                _ => {}
            }
        }
        Ok(())
    }
}