fn div_by_three(num: int) -> bool {
    num % 3 == 0
}

fn div_by_five(num: int) -> bool {
    num % 5 == 0
}

fn div_by_fifteen(num: int) -> bool {
    num % 15 == 0
}

fn main() {
    for num in range(1i, 100) {
        println!("{:s}",
            if div_by_fifteen(num) { "Fizzbuzz".to_string() }
            else if div_by_three(num) { "Fizz".to_string() }
            else if div_by_five(num) { "Buzz".to_string() }
            else { num.to_string() }
        );
    }
}

#[test]
fn div_by_three_test() {
    let x = div_by_three(1); //semicolon denotes no returned valuestrue
    assert!(x == false); //clearly no return values
}

#[test]
fn div_by_three_with_three_test() {
    let x = div_by_three(3);
    assert!(x == true);
}

#[test]
fn div_by_five_test() {
    let x = div_by_five(1);
    assert!(x == false);
}

#[test]
fn div_by_five_with_five_test() {
    let x = div_by_five(5);
    assert!(x == true);
}

#[test]
fn div_by_fifteen_test() {
    let x = div_by_fifteen(1);
    assert!(x == false)
}

#[test]
fn div_by_fifteen_with_fifteen_test() {
    let x = div_by_fifteen(15);
    assert!(x == true)
}
