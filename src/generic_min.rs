use std::cmp::Ordering;

pub fn min <T:Ord>(a:T, b:T) -> T {
    let result = a.cmp(&b);
        match result {
         Ordering::Less | Ordering::Equal  => a,
         Ordering::Greater => b,
        }
}

