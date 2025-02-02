
fn addition(number1: i32, number2: i32)-> i32{

    number1+number2
 


}

fn main() {
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1_addition() {
        assert_eq!(addition(1, 1), 2);
    }
}

