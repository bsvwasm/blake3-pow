#[cfg(test)]
mod tests {
    extern crate wasm_bindgen_test;
    use wasm_bindgen_test::*;
    wasm_bindgen_test::wasm_bindgen_test_configure!();


  #[test]
  #[wasm_bindgen_test]
  fn easy_mine_target() {
    let target = "0021e8000000000000000000000000000000000000000000000000000000000";
    println!("mining to target {}...", &target);

    let output = blake3_pow::mine(b"Hello, PoW!", target);


    println!("finished mining!");
    assert_eq!(blake3_pow::verify(&output, target), true)
  }

  #[test]
  #[wasm_bindgen_test]
  fn medium_mine_target() {
    let target = "000021e80000000000000000000000000000000000000000000000000000000";
    println!("mining to target {}...", &target);

    let output = blake3_pow::mine(b"Hello, PoW!", target);


    println!("finished mining!");
    assert_eq!(blake3_pow::verify(&output, target), true)
  }
}