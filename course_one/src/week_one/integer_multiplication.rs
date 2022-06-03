use std::iter;

type Number = Vec<u8>;

// returns a tuple
//  .0 -> if num is a power of 2
//  .1 -> closest power of 2 >= num
fn check_pow2(num: usize) -> (bool, usize) {
    let mut pow = 0;
    while num > 1 << pow {
        pow += 1;
    }

    (if num == 1 << pow { true } else { false }, 1 << pow)
}

fn fix_number(mut num: Number) -> Number {
    while num.len() > 1 && num.last().unwrap() == &0 {
        num.pop();
    }

    let (is_pow2, target_pow2) = check_pow2(num.len());
    if !is_pow2 {
        num.extend(iter::repeat(0).take(target_pow2 - num.len()));
    }

    num
}

fn numberify(string: &str) -> Number {
    fix_number(
        string
            .chars()
            .map(|x| x.to_digit(10).unwrap() as u8)
            .rev()
            .collect(),
    )
}

fn stringify(vec: &Number) -> String {
    vec.iter()
        .map(|x| x.to_string())
        .rev()
        .collect::<String>()
        .trim_start_matches("0")
        .to_string()
}

fn multiply(x: u8, y: u8) -> Number {
    let mul = x * y;

    fix_number(vec![mul % 10, mul / 10])
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

    fix_number(result)
}

fn sub(x: &Number, y: &Number) -> Number {
    // this function should panic if x < y
    let mut result = Vec::new();
    let (mut trav, mut borrow) = (0, false);

    loop {
        let xd = x[trav] as i32 - if borrow { 1 } else { 0 };
        let yd = if trav < y.len() { y[trav] as i32 } else { 0 };

        let (xd, yd) = if xd < yd {
            borrow = true;
            ((xd + 10) as u8, yd as u8)
        } else {
            borrow = false;
            (xd as u8, yd as u8)
        };

        result.push(xd - yd);
        trav += 1;

        if trav >= x.len() {
            break;
        }
    }

    fix_number(result)
}

fn pow_10(num: &Number, n: usize) -> Number {
    let mut result = vec![0; n];
    result.extend(num.iter());

    fix_number(result)
}

fn split(num: &Number) -> (Number, Number) {
    let mut num = num.clone();
    let rem = num.split_off(num.len() / 2);

    (fix_number(rem), fix_number(num))
}

fn recursive(x: &mut Number, y: &mut Number) -> Number {
    // dbg!(stringify(x), stringify(y));

    let n = usize::max(x.len(), y.len());
    if x.len() < n {
        x.extend(iter::repeat(0).take(n - x.len()));
    }
    if y.len() < n {
        y.extend(iter::repeat(0).take(n - y.len()));
    }

    if n == 1 {
        return multiply(x[0], y[0]);
    }

    let (mut a, mut b) = split(x);
    let (mut c, mut d) = split(y);
    // dbg!(&a, &b, &c, &d);

    let ac = recursive(&mut a, &mut c);
    let ad = recursive(&mut a, &mut d);
    let bc = recursive(&mut b, &mut c);
    let bd = recursive(&mut b, &mut d);
    let ad_plus_bc = add(&ad, &bc);
    // dbg!(&ac, &ad, &bc, &bd, &ad_plus_bc);

    let first = pow_10(&ac, n);
    let second = pow_10(&ad_plus_bc, n / 2);
    // dbg!(&first, &second);

    *x = fix_number(x.clone());
    *y = fix_number(y.clone());

    add(&add(&first, &second), &bd)
}

fn karatsuba(x: &mut Number, y: &mut Number) -> Number {
    // dbg!(stringify(x), stringify(y));

    let k = usize::max(x.len(), y.len());
    if x.len() < k {
        x.extend(iter::repeat(0).take(k - x.len()));
    }
    if y.len() < k {
        y.extend(iter::repeat(0).take(k - y.len()));
    }

    if k == 1 {
        return multiply(x[0], y[0]);
    }

    let (mut a, mut b) = split(x);
    let (mut c, mut d) = split(y);
    // dbg!(&a, &b, &c, &d);

    let l = karatsuba(&mut a, &mut c);
    let m = karatsuba(&mut b, &mut d);
    let mut a_plus_b = add(&a, &b);
    let mut c_plus_d = add(&c, &d);
    let n = karatsuba(&mut a_plus_b, &mut c_plus_d);
    let o = sub(&sub(&n, &m), &l);
    // dbg!(&l, &m, &n, &o);

    let first = pow_10(&l, k);
    let second = pow_10(&o, k / 2);
    // dbg!(&first, &second);

    *x = fix_number(x.clone());
    *y = fix_number(y.clone());

    add(&add(&first, &second), &m)
}

#[cfg(test)]
mod tests {
    use super::*;

    const X: &str = "3141592653589793238462643383279502884197169399375105820974944592";
    const Y: &str = "2718281828459045235360287471352662497757247093699959574966967627";

    fn recursive_helper(x: &str, y: &str, exp: &str) {
        let (mut x, mut y) = (numberify(x), numberify(y));
        let res = recursive(&mut x, &mut y);
        let res = stringify(&res);

        assert_eq!(res, exp);
    }

    fn karatsuba_helper(x: &str, y: &str, exp: &str) {
        let (mut x, mut y) = (numberify(x), numberify(y));
        let res = karatsuba(&mut x, &mut y);
        let res = stringify(&res);

        assert_eq!(res, exp);
    }

    #[test]
    fn test_solution() {
        let (mut x, mut y) = (numberify(X), numberify(Y));
        let rec = stringify(&recursive(&mut x, &mut y));
        let kar = stringify(&karatsuba(&mut x, &mut y));

        assert_eq!(rec, kar);
    }

    #[test]
    fn test_recursive() {
        recursive_helper("12", "20", "240");
        recursive_helper("123", "10", "1230");
    }

    #[test]
    fn test_karatsuba() {
        karatsuba_helper("12", "34", "408");
        karatsuba_helper("12", "20", "240");
        karatsuba_helper("123", "10", "1230");
    }

    #[test]
    fn test_check_pow2() {
        assert_eq!(check_pow2(0), (false, 1));
        assert_eq!(check_pow2(1), (true, 1));
        assert_eq!(check_pow2(8), (true, 8));
        assert_eq!(check_pow2(9), (false, 16));
    }

    #[test]
    fn test_numberify() {
        assert_eq!(numberify("123"), vec![3, 2, 1, 0]);
        assert_eq!(numberify("001"), vec![1]);
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
    fn test_sub() {
        assert_eq!(sub(&vec![2, 1], &vec![2, 1]), vec![0]);
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
