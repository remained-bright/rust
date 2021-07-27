pub fn find<Hash: Fn(&[u8]) -> u64>(n: u32, v: &[u8], hash: Hash) -> Vec<u8> {
  let mut token = vec![];
  let vec = v.to_vec();
  loop {
    let mut txt = vec.clone();
    txt.extend(&token[..]);

    let h = hash(&txt);
    if h.leading_zeros() >= n {
      return token;
    }
    vec_incr(&mut token);
  }
}

pub fn vec_incr(b256: &mut Vec<u8>) {
  let mut i = b256.len();

  while i != 0 {
    i -= 1;
    let n = b256[i];
    if n == 255 {
      b256[i] = 0;
    } else {
      b256[i] = n + 1;
      return;
    }
  }

  b256.insert(0, 1);
}
