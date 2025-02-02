use dashu_float::ops::Abs;
use dashu_float::DBig;
use std::ops::Deref;
use std::str::FromStr;
use std::sync::LazyLock;

static PI: LazyLock<DBig> = LazyLock::new(|| {
    DBig::from_str("3.141592653589793238462643383279502884197169399375105820974944592307816406286")
        .unwrap()
});

pub static PIMUL2: LazyLock<DBig> = LazyLock::new(|| {
    DBig::from_str("3.141592653589793238462643383279502884197169399375105820974944592307816406286")
        .unwrap()
        * DBig::from(2)
});

static PIDIV2: LazyLock<DBig> = LazyLock::new(|| {
    DBig::from_str("3.141592653589793238462643383279502884197169399375105820974944592307816406286")
        .unwrap()
        / DBig::from(2)
});

static DBIGTEN: LazyLock<DBig> = LazyLock::new(|| DBig::from(10));

pub fn sin(x: DBig, precision: i64) -> DBig {
    let x = (x / PIMUL2.deref()).fract() * PIMUL2.deref();
    let mut term = x.clone();
    let mut result = x.clone();
    let mut n = 1;
    let x_sq = &x * &x;

    let limit = DBIGTEN.powf(&DBig::from(-precision));

    while term.clone().abs() > limit {
        term = -term * &x_sq / DBig::from((2 * n) * (2 * n + 1));
        result += &term;
        n += 1;
    }

    result
}

pub fn cos(x: DBig, precision: i64) -> DBig {
    sin(x + PIDIV2.deref(), precision)
}

pub fn dbig_to_f64(v: &DBig) -> f64 {
    f64::from_str(v.to_string().as_str()).unwrap()
}

pub fn f64_to_dbig(v: f64) -> DBig {
    DBig::from_str(v.to_string().as_str()).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dbig_to_f64(v: &DBig) -> f64 {
        f64::from_str(v.to_string().as_str()).unwrap()
    }

    #[test]
    fn sin_works() {
        for i in -10..10 {
            for f in -10..10 {
                let v = i as f64 + (f as f64) / 10.0;
                let dec = DBig::from_str(v.to_string().as_str()).unwrap();
                let sin_dec = sin(dec, 32);
                let sin_ref = v.sin();
                // println!("sin({v}) resulted in {sin_dec}, reference is {sin_ref}");
                assert!((dbig_to_f64(&sin_dec) - sin_ref).abs() < 0.0000000000001);
            }
        }
    }

    #[test]
    fn cos_works() {
        for i in -10..10 {
            for f in -10..10 {
                let v = i as f64 + (f as f64) / 10.0;
                let dec = DBig::from_str(v.to_string().as_str()).unwrap();
                let cos_dec = cos(dec, 32);
                let cos_ref = v.cos();
                // println!("sin({v}) resulted in {sin_dec}, reference is {sin_ref}");
                assert!((dbig_to_f64(&cos_dec) - cos_ref).abs() < 0.0000000000001);
            }
        }
    }
}
