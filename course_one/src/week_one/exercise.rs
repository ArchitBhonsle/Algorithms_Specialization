type Number = Vec<u32>;

fn numberify(string: &str) -> Number {
    string
        .chars()
        .map(|x| x.to_digit(10).unwrap())
        .rev()
        .collect()
}

fn stringify(vec: &Number) -> String {
    vec.iter()
        .map(|x| x.to_string())
        .rev()
        .collect::<String>()
        .trim_start_matches("0")
        .to_string()
}

fn multiply(x: u32, y: u32) -> Number {
    let mul = x * y;
    vec![mul % 10, mul / 10]
}

fn add(x: &Number, y: &Number) -> Number {
    // ensures that x is longer than y
    let mut result = Vec::new();
    let (mut trav, mut carry) = (0, 0);

    loop {
        let x_digit = if trav < x.len() { x[trav] } else { 0 };
        let y_digit = if trav < y.len() { y[trav] } else { 0 };

        let sum = x_digit + y_digit + carry;
        result.push(sum % 10);
        carry = sum / 10;
        trav += 1;

        if trav >= x.len() && trav >= y.len() && carry == 0 {
            break;
        }
    }

    result
}

fn pow_10(num: &Number, n: u32) -> Number {
    let mut result = vec![0; n as usize];
    result.extend(num.iter());
    result
}

fn split(num: &Number) -> (Number, Number) {
    let mut num = num.clone();
    let rem = num.split_off(num.len() / 2);

    (rem, num)
}

fn recursive(x: &Number, y: &Number) -> Number {
    let (x_digits, y_digits) = (x.len(), y.len());
    if x_digits == 1 && y_digits == 1 {
        return multiply(x[0], y[0]);
    }
    let n = x_digits as u32; // This will break for uneven splits

    let (a, b) = split(x);
    let (c, d) = split(y);
    // dbg!(&a, &b, &c, &d);

    let ac = recursive(&a, &c);
    let ad = recursive(&a, &d);
    let bc = recursive(&b, &c);
    let bd = recursive(&b, &d);
    let ad_plus_bc = add(&ad, &bc);
    // dbg!(&ac, &ad, &bc, &bd, &ad_plus_bc);

    let first = pow_10(&ac, n);
    let second = pow_10(&ad_plus_bc, n / 2);
    // dbg!(&first, &second);

    add(&add(&first, &second), &bd)
}

#[cfg(test)]
mod tests {
    use super::*;

    const X: &str = "3141592653589793238462643383279502884197169399375105820974944592";
    const Y: &str = "2718281828459045235360287471352662497757247093699959574966967627";

    fn multiplication_helper(x: &str, y: &str, exp: &str) {
        let (x, y) = (numberify(x), numberify(y));
        let res = recursive(&x, &y);
        dbg!(&res);
        let res = stringify(&res);

        assert_eq!(res, exp);
    }

    #[test]
    fn test_solution() {
        dbg!(stringify(&recursive(&numberify(X), &numberify(Y))));
    }

    #[test]
    fn test_recursive() {
        multiplication_helper("12", "20", "240");
    }

    #[test]
    fn test_numberify() {
        assert_eq!(numberify("123"), vec![3, 2, 1]);
    }

    #[test]
    fn test_stringify() {
        assert_eq!("123", stringify(&vec![3, 2, 1]));
    }

    #[test]
    fn test_multiply() {
        assert_eq!(multiply(9, 9), vec![1, 8]);
    }

    #[test]
    fn test_add() {
        assert_eq!(add(&vec![2, 1], &vec![8, 1]), vec![0, 3]);
    }

    #[test]
    fn test_pow_10() {
        assert_eq!(pow_10(&vec![1], 3), vec![0, 0, 0, 1])
    }

    #[test]
    fn test_split() {
        assert_eq!(split(&vec![4, 3, 2, 1]), (vec![4, 3], vec![2, 1]));
    }
}
