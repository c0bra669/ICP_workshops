#[ic_cdk::query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}
#[ic_cdk::query]
fn calculate(x: i32 ,y: i32,operator: String)->String{
    let result = match operator.as_str() {
        "+"=>Some(x+y),
        "-"=>Some(x-y),
        "*"=>Some(x*y),
        "/"=>if y !=0{Some(x/y)}else{None},
        _=>None
    };
    match  result {
        Some(value)=> format!("Result:{}",value),
        None => "invalid operator or deviding by zero".to_string(),
    }

}
