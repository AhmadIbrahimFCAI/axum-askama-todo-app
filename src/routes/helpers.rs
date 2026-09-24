use validator::ValidationErrors;



pub fn extract_error<F>(input: &str, mut f:F) where F: FnMut(String, String){
    let lines = input.lines();
    lines.for_each(|line| {
        if let Some((first, second)) = line.split_once(": "){
            f(first.to_string(), second.to_string())
        }
    });
}

pub fn extract_errors(errors: &ValidationErrors)-> Vec<(String, String)>{
    errors.field_errors().into_iter()
    .map(|(field, errs)| (
        field.to_string(),
        errs[0].message.as_ref().map_or("Invalid", |m| m).to_string(),
    )).collect()
}