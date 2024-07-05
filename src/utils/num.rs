pub trait Ths {
    fn ths(&self) -> String;
}

impl Ths for i64 {
    fn ths(&self) -> String {
        if self.abs() < 1000 {
            return self.to_string();
        }
        let mut s = String::new();
        let v = self.to_string();
        for (i, c) in v.chars().rev().enumerate() {
            if i % 3 == 0 && i != 0 && c != '-' {
                s.insert(0, ',');
            }
            s.insert(0, c);
        }
        s
    }
}

macro_rules! impl_ths {
    ($($t:ty),*) => {
        $(
            impl Ths for $t {
                fn ths(&self) -> String {
                    (*self as i64).ths()
                }
            }
        )*
    };
}

impl_ths!(i32, u64, u32, i16, u16, i8, u8, isize, usize);

#[inline]
pub fn money(v: i64) -> String {
    if v < 1000 {
        return format!("${}.{:02}", v / 100, v % 100);
    }
    let mut s = String::new();
    let v = v.to_string();
    s.push('.');
    s.push_str(&v[v.len() - 2..]);
    for (i, c) in v.chars().rev().skip(2).enumerate() {
        if i % 3 == 0 && i != 0 {
            s.insert(0, ',');
        }
        s.insert(0, c);
    }
    s.insert(0, '$');
    s
}

#[inline]
pub fn to_money(v: f64) -> i64 {
    (v * 100.0).round() as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_money() {
        assert_eq!(money(1000), "$10.00");
        assert_eq!(money(100), "$1.00");
        assert_eq!(money(100000), "$1,000.00");
        assert_eq!(money(1000000), "$10,000.00");
        assert_eq!(money(10000000), "$100,000.00");
        assert_eq!(money(100000000), "$1,000,000.00");
        assert_eq!(money(1000000000), "$10,000,000.00");
    }

    #[test]
    fn test_to_money() {
        assert_eq!(to_money(10.0), 1000);
        assert_eq!(to_money(1.0), 100);
        assert_eq!(to_money(1000.0), 100000);
        assert_eq!(to_money(10000.0), 1000000);
        assert_eq!(to_money(100000.0), 10000000);
        assert_eq!(to_money(1000000.0), 100000000);
        assert_eq!(to_money(10000000.0), 1000000000);
    }

    #[test]
    fn test_ths() {
        assert_eq!(1000.ths(), "1,000");
        assert_eq!(100.ths(), "100");
        assert_eq!(100000.ths(), "100,000");
        assert_eq!(1000000.ths(), "1,000,000");
        assert_eq!(10000000.ths(), "10,000,000");
        assert_eq!(100000000.ths(), "100,000,000");
        assert_eq!(1000000000.ths(), "1,000,000,000");
        assert_eq!((-1000).ths(), "-1,000");
        assert_eq!((-100).ths(), "-100");
        assert_eq!((-100000).ths(), "-100,000");
        assert_eq!((-1000000).ths(), "-1,000,000");
        assert_eq!((-10000000).ths(), "-10,000,000");
        assert_eq!((-100000000).ths(), "-100,000,000");
        assert_eq!((-1000000000).ths(), "-1,000,000,000");
    }
}
