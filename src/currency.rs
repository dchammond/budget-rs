use std::ops::{Add, AddAssign, Div, DivAssign, Neg, Sub, SubAssign};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct Currency {
    amount: i64,
}

impl Currency {
    pub fn new(amount: i64) -> Self {
        Currency { amount }
    }

    pub fn set(&mut self, amount: i64) -> i64 {
        let ret = amount - self.amount;
        self.amount = amount;
        ret
    }

    pub fn get(&self) -> i64 {
        self.into()
    }

    pub fn to_string_numeric(&self) -> String {
        let mut s;
        if self.amount < 0 {
            s = String::from("-$");
        } else {
            s = String::from("$");
        }
        let f = (self.amount.abs() as f64) / 100.0;
        s.push_str(&format!("{:.02}", f));
        s
    }

    pub fn to_string_accounting(&self) -> String {
        let mut s;
        if self.amount < 0 {
            s = String::from("(");
        } else {
            s = String::new();
        }
        let f = (self.amount.abs() as f64) / 100.0;
        s.push_str(&format!("{:.02}", f));
        if self.amount < 0 {
            s.push_str(")");
        }
        s
    }
}

impl From<i64> for Currency {
    fn from(amount: i64) -> Self {
        Currency::new(amount)
    }
}

impl From<&i64> for Currency {
    fn from(amount: &i64) -> Self {
        Currency::new(*amount)
    }
}

impl From<&mut i64> for Currency {
    fn from(amount: &mut i64) -> Self {
        Currency::new(*amount)
    }
}

impl From<Currency> for i64 {
    fn from(curr: Currency) -> Self {
        curr.amount
    }
}

impl From<&Currency> for i64 {
    fn from(curr: &Currency) -> Self {
        curr.amount
    }
}

impl From<&mut Currency> for i64 {
    fn from(curr: &mut Currency) -> Self {
        curr.amount
    }
}

impl Neg for Currency {
    type Output = Currency;

    fn neg(self) -> Self::Output {
        Currency::new(-self.get())
    }
}

macro_rules! arith_func {
    ($type_name: ty, $op_name: ident, $fn_name: ident) => {
        impl<'a> $op_name<&'a $type_name> for &'a Currency {
            type Output = Currency;
            fn $fn_name(self, rhs: &'a $type_name) -> Self::Output {
                let lhs = i64::from(self);
                let rhs = i64::from(*rhs);
                Currency::from(i64::$fn_name(lhs, rhs))
            }
        }

        impl $op_name<$type_name> for Currency {
            type Output = Currency;
            fn $fn_name(self, rhs: $type_name) -> Self::Output {
                let lhs = i64::from(self);
                let rhs = i64::from(rhs);
                Currency::from(i64::$fn_name(lhs, rhs))
            }
        }
    };
}

macro_rules! arith_func_type {
    ($type_name: ty) => {
        arith_func!($type_name, Add, add);
        arith_func!($type_name, Sub, sub);
        arith_func!($type_name, Div, div);
    };
}

macro_rules! arith_assign_func {
    ($type_name: ty, $op_name: ident, $fn_name: ident, $base_name: ident) => {
        impl $op_name<$type_name> for Currency {
            fn $fn_name(&mut self, rhs: $type_name) {
                let rhs = Currency::from(rhs);
                *self = Currency::$base_name(*self, rhs);
            }
        }
    };
}

macro_rules! arith_assign_func_type {
    ($type_name: ty) => {
        arith_assign_func!($type_name, AddAssign, add_assign, add);
        arith_assign_func!($type_name, SubAssign, sub_assign, sub);
        arith_assign_func!($type_name, DivAssign, div_assign, div);
    };
}

macro_rules! generate {
    ($type_name: ty) => {
        arith_func_type!($type_name);
        arith_assign_func_type!($type_name);
    };
}

generate!(Currency);
generate!(i64);
