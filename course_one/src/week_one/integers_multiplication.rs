fn num_digits(x: u32) -> u32 {
    x.checked_log10().unwrap_or(0) + 1
}

fn cut(x: u32) -> (u32, u32) {
    let digits = num_digits(x);
    let first_digits = digits / 2;

    let (mut f, mut s) = (x, 0);
    for i in 0..first_digits {
        let l = f % 10;
        s = s + l * 10u32.pow(i);
        f /= 10;
    }

    (f, s)
}

// TODO: only works for even splits
fn recursive(x: u32, y: u32) -> u32 {
    let (xd, yd) = (num_digits(x), num_digits(y));
    if xd == 1 && yd == 1 {
        return x * y;
    }

    let (a, b) = cut(x);
    let (c, d) = cut(y);

    return 10u32.pow(xd) * recursive(a, c)
        + 10u32.pow(xd / 2) * (recursive(a, d) + recursive(b, c))
        + recursive(b, d);
}

// TODO: this does not work because I did not handle uneven splits
fn karatsuba(x: u32, y: u32) -> u32 {
    let (xd, yd) = (num_digits(x), num_digits(y));
    dbg!(x, xd, y, yd);
    if xd == 1 && yd == 1 {
        return x * y;
    }

    let (a, b) = cut(x);
    let (c, d) = cut(y);

    let (l, m, n) = (karatsuba(a, c), karatsuba(b, d), karatsuba(a + b, c + d));
    let o = n - m - l;
    dbg!(l, m, n);

    return n * 10u32.pow(xd) + o * 10u32.pow(xd / 2) + m;
}

#[cfg(test)]
mod test {
    use super::*;

    const X: u32 = 1234;
    const Y: u32 = 5678;
    const Z: u32 = X * Y;

    #[test]
    fn cut_test() {
        assert_eq!(cut(X), (12, 34));
    }

    #[test]
    fn recursive_test() {
        assert_eq!(recursive(X, Y), Z);
    }

    #[test]
    fn karatsuba_test() {
        assert_eq!(karatsuba(X, Y), Z);
    }
}
