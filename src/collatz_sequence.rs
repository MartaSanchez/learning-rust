pub fn collatz_length(mut n: i32) -> u32 {
    let mut result: u32 = 1;
    while n > 1 {
        n = if n % 2 != 0 { 3 * n + 1 } else { n / 2 };
        result += 1;
    }
     return result;
}
   
  


