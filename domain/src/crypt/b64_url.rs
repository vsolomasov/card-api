use base64::engine::general_purpose;
use base64::engine::Engine;

// use super::error::Error;
// use super::error::Result;

pub fn encode<T: ?Sized + AsRef<[u8]>>(input: &T) -> String {
  general_purpose::URL_SAFE_NO_PAD.encode(input)
}

// pub fn decode<T: ?Sized + AsRef<[u8]>>(input: &T) -> Result<Vec<u8>> {
//   general_purpose::URL_SAFE_NO_PAD
//     .decode(input)
//     .map_err(|_| Error::FailToDecodeBase64)
// }

// pub fn decode_to_str<T: ?Sized + AsRef<[u8]>>(input: &T) -> Result<String> {
//   decode(input).and_then(|bytes| String::from_utf8(bytes).map_err(|_| Error::FailToDecodeBase64))
// }

#[cfg(test)]
mod tests {
  use anyhow::Result;

  // use rand::RngCore;
  use super::*;

  #[test]
  fn test_crypt_b64_url_encode() -> Result<()> {
    let input = "hello_world";
    let expected = "aGVsbG9fd29ybGQ";
    let output = encode(input);
    assert_eq!(output, expected);
    Ok(())
  }

  // #[test]
  // fn test_crypt_b64_url_decode() -> Result<()> {
  //   let input = "aGVsbG9fd29ybGQ";
  //   let expected = "hello_world";
  //   let output = decode_to_str(input)?;
  //   assert_eq!(output, expected);
  //   Ok(())
  // }

  // #[test]
  // fn test_crypt_b64_url() -> Result<()> {
  //   let mut random_str = [0u8; 64];
  //   rand::thread_rng().fill_bytes(&mut random_str);
  //   let encoded = encode(&random_str);
  //   let decoded = decode(&encoded)?;
  //   assert_eq!(random_str.to_vec(), decoded);
  //   Ok(())
  // }
}
