use rs_tfhe::bit_utils::{convert, AsBits};
use rs_tfhe::utils::Ciphertext;
use rs_tfhe::key::CloudKey;
use rs_tfhe::key::SecretKey;
use rs_tfhe::gates::*;
use rs_tfhe::tlwe::TLWELv0;
use rs_tfhe::params::*;


fn full_adder(
  server_key: &CloudKey,
  ct_a: &Ciphertext,
  ct_b: &Ciphertext,
  ct_c: &Ciphertext,
) -> (Ciphertext, Ciphertext) {
  // tlwe_nand = trgsw::hom_nand(&ct_a, &ct_b, &server_key, &mut fft_plan);
  
  let a_xor_b = xor(ct_a, ct_b, server_key);
  let a_and_b = and(ct_a, ct_b, server_key);
  let a_xor_b_and_c = and(&a_xor_b, ct_c, server_key);
  // sum = (a xor b) xor c
  let ct_sum = xor(&a_xor_b, ct_c, server_key);
  // carry = (a and b) or ((a xor b) and c)
  let ct_carry = or(&a_and_b, &a_xor_b_and_c, server_key);
  // return sum and carry
  (ct_sum, ct_carry)
}

pub fn add(
  server_key: &CloudKey,
  a: &Vec<Ciphertext>,
  b: &Vec<Ciphertext>,
  cin: Ciphertext,
) -> (Vec<Ciphertext>, Ciphertext) {
  assert_eq!(
    a.len(),
    b.len(),
    "Cannot add two numbers with different number of bits!"
  );
  let mut result = Vec::with_capacity(a.len());
  let mut carry = cin;
  for i in 0..a.len() {
    let (sum, c) = full_adder(server_key, &a[i], &b[i], &carry);
    carry = c;
    result.push(sum);
  }
  (result, carry)
}

pub fn add2(
    server_key: &CloudKey,
    a: &Vec<Ciphertext>,
    b: &Vec<Ciphertext>,
    cin: Ciphertext,
  ) -> Vec<Ciphertext> {
    assert_eq!(
      a.len(),
      b.len(),
      "Cannot add two numbers with different number of bits!"
    );
    let mut result = Vec::with_capacity(a.len());
    let mut carry = cin;
    for i in 0..a.len() {
      let (sum, c) = full_adder(server_key, &a[i], &b[i], &carry);
      carry = c;
      result.push(sum);
    }
    result
  }

pub fn sub(
  server_key: &CloudKey,
  a: &Vec<Ciphertext>,
  b: &Vec<Ciphertext>,
  cin: Ciphertext,
) -> (Vec<Ciphertext>, Ciphertext) {
  assert_eq!(
    a.len(),
    b.len(),
    "Cannot add two numbers with different number of bits!"
  );

  // WARNING: this function does not work as it is off by one

  let not_b = b.iter().map(not).collect::<Vec<Ciphertext>>();
  add(server_key, a, &not_b, cin)
}

fn encrypt(x: bool, secret_key: &SecretKey) -> Ciphertext {
  TLWELv0::encrypt_bool(x, tlwe_lv0::ALPHA, &secret_key.key_lv0)
}

fn decrypt(x: &Ciphertext, secret_key: &SecretKey) -> bool {
  TLWELv0::decrypt_bool(x, &secret_key.key_lv0)
}

fn get_enc_x(mut base: Vec<Ciphertext>, mut result: Vec<Ciphertext>, cin : TLWELv0, mut x: u8, server_key: &CloudKey) -> Vec<Ciphertext> {

    x = x - 1;

    while x > 0 {
        if x & 1 == 1 {
            result = add2(& server_key,&result, &base, cin); // Add when the bit is set
        }
        base = add2(& server_key,&base, &base, cin); // Double the base
        x >>= 1; // Shift x right
    }

    result
}

fn uint_to_cipher(y : u8, secret_key: &SecretKey) -> Vec<Ciphertext> {
    let pt = y.to_bits();
    let out = pt
    .iter()
    .map(|x| encrypt(*x, &secret_key))
    .collect::<Vec<Ciphertext>>();

    out
}

fn cipher_to_uint(c : &Vec<Ciphertext>, secret_key: &SecretKey) -> u8 {
    let r = c
    .iter()
    .map(|x| decrypt(x, &secret_key))
    .collect::<Vec<bool>>();

    let out = convert::<u8>(&r);
    out
}

// fn multi_uint_to_chipher(my_vec : Vec<u8>, secret_key: &SecretKey) -> Vec<Vec<Ciphertext>> {
//     let mut temp: Vec<Vec<Ciphertext>> = Vec::new();
//     for y in my_vec {
//         temp.push(uint_to_cipher(y, &secret_key));
//     }
//     temp
// }

fn server(enc_xi: Vec<Ciphertext>, cone : Vec<Ciphertext>, czero : Vec<Ciphertext>, cin : TLWELv0, yi: u8, server_r: u8, server_key: &CloudKey) -> Vec<Ciphertext> {
  let enc_yi = get_enc_x(cone.clone(), czero.clone(), cin.clone(), yi, &server_key);

  let (test_equal, _cnew) = sub(&server_key, &enc_xi, &enc_yi, cin.clone());

  let test_equal2 = get_enc_x(test_equal.clone(), czero.clone(), cin.clone(), server_r, &server_key);

  return test_equal2;
}

fn main() {
  let secret_key = SecretKey::new();
  let cloud_key = CloudKey::new(&secret_key);

  // Use the client secret key to encrypt plaintext a to ciphertext a
  let xi = uint_to_cipher(102, &secret_key);

  let czero = uint_to_cipher(0, &secret_key);

  let cone = uint_to_cipher(1, &secret_key);

  let cin = encrypt(false, &secret_key);

  // ----------------- SERVER SIDE -----------------
  // Use the server public key to add the a and b ciphertexts
  let diff = server(xi, cone, czero, cin, 102, 31,&cloud_key);
  // -------------------------------------------------

  // Decrept
  println!("sum: {}", cipher_to_uint(&diff, &secret_key));
}
