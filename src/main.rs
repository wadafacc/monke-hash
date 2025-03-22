use std::{fs::File, io::Read};

fn main() -> std::io::Result<()>{
  let str_in = [
    "Dolores harum consequuntur libero ipsa dolorum reprehenderit incidunt. Aut pariatur esse nesciunt consequatur accusantium asperiores quia laudantium. Quae ducimus reprehenderit magnam placeat perferendis omnis. Dolores quis omnis itaque optio. Qui eos aut pariatur.", 
    "Monkey was here", 
    "monkey was here ", 
    "monkey was HERE"
  ];

  // uses the pixel values of the monkey png to hash the string
  let mut img = File::open("monke.png")?;  
  
  let mut buf = Vec::new();
  _ = img.read_to_end(&mut buf)?;

  for s in str_in {
    println!("{:<16} -> {:x}", format!("'{}...'", &s[0..12]), hash(s, &buf));
  }

  Ok(())
}

/// main hashing function -> returns the hashed value as u128
fn hash(str_in: &str, buf: &[u8]) -> u128 {
  let mut hash = 1;
  for c in str_in.chars() {
    let mut i = c as u128;
    let mut i1= 0;
    let pixel_idx= (i % (buf.len() as u128)) as usize;

    // some arbitrary mathematical stuff to seem important and obfuscate the shit out of it
    for n in 1..64 {
      i1 = i & buf[pixel_idx] as u128;
      // multiply by nth semi-prime number
      let i2 = i1 ^ calc_prime(pixel_idx, buf.len());
      // + index^-1
      let i3 = i2 + buf[buf.len() - pixel_idx] as u128;
      // xor
      let i4 = i3 ^ 2_u128.pow(pixel_idx as u32);

      i = i4.rotate_right(n as u32) ^ i4.rotate_left(i1 as u32);
      // println!("i: {i}");
    }
    hash = (hash + i).rotate_left(i1 as u32);
  }
  hash
}

// semi-prime that switches based on len
fn calc_prime(n: usize, len: usize) -> u128 {
  if n < (len / 2 ) {
    return (6 * n as u128) -1;
  }
  (6 * n as u128) +1
}