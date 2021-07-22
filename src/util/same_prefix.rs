pub fn same_prefix<const N: usize>(a: [u8; N], b: [u8; N]) -> u32 {
  let mut count = 0;
  for (i, j) in a.zip(b) {
    if i == j {
      count += 8;
    } else {
      count += (i ^ j).leading_zeros();
      break;
    }
  }
  count
}
