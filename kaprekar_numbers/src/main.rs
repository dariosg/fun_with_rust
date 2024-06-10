use arrayref::array_mut_ref;
use rayon::prelude::*;
use num::integer::div_rem;
use sortnet::*;
use std::env;

// from https://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/book/second-edition/ch12-01-accepting-command-line-arguments.html

// function that extracts the digits from a number and returns them ordered in an array


// unsigned int values cannot have more than 10 digits
fn from_value_to_digits(v: u32) -> ([u32; 10], usize)
{
  let mut digits: [u32; 10] = [0; 10];
  
  let mut val = v;
  let mut i = 0;

  while (val > 9) && (i < 10) {
    (val, digits[i]) = div_rem(val, 10);
    i +=1;
  }
  if i < 10 {
    digits[i] = val;
    i += 1;
  }

  match i {
     0 => i = 1,
     // 1 => sortnet1(&mut digits[..1]),
     1 => sortnet1(array_mut_ref![digits, 0, 1]),
     2 => sortnet2(array_mut_ref![digits, 0, 2]),
     3 => sortnet3(array_mut_ref![digits, 0, 3]),
     4 => sortnet4(array_mut_ref![digits, 0, 4]),
     5 => sortnet5(array_mut_ref![digits, 0, 5]),
     6 => sortnet6(array_mut_ref![digits, 0, 6]),
     7 => sortnet7(array_mut_ref![digits, 0, 7]),
     8 => sortnet8(array_mut_ref![digits, 0, 8]),
     9 => sortnet9(array_mut_ref![digits, 0, 9]),
     10 => sortnet10(array_mut_ref![digits, 0, 10]),
     _ => panic!("This is an error, {}", i),
  }

  (digits, i)
}

#[inline]
const fn fast_10pow(i: u32) -> u32
{
  match i {
      0 => 1,
      1 => 10,
      2 => 100,
      3 => 1000,
      4 => 10000,
      5 => 100000,
      6 => 1000000,
      7 => 10000000,
      _ => 10_u32.pow(i),
  }
}

fn is_kaprekar(n: u32) -> (bool, u32, u32)
{
   let (min_max, len) = from_value_to_digits(n);

   let mut min = min_max[0]*fast_10pow(len as u32 -1);
   let mut max = min_max[0];

   // generate max and min values
   for i in 1..len {
     let j = len - i;
     max += min_max[i]*fast_10pow(i as u32);
     min += min_max[i]*fast_10pow(j as u32 - 1);
   }

   ((max-min) == n, max, min)
}

fn main() {
  let args: Vec<String> = env::args().collect();

  let min_str = &args[1];
  let max_str = &args[2];
  let min_val: u32 = min_str.parse().expect("Not a valid number");
  let max_val: u32 = max_str.parse().expect("Not a valid number");

  // sequential version
  // for i in min_val..max_val {
  //   let tuple = is_kaprekar(i);
  //   if tuple.0 {
  //     println!("Is Kaprekar: {}", i);
  //   }
  // }
  (min_val..max_val).into_par_iter().with_min_len(64).for_each(|i| if is_kaprekar(i).0 { println!("Is Kaprekar: {}", i); });
}
