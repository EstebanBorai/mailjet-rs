use std::path::Path;
use std::fs::read as read_file;
use std::io::Write;
use base64;

/// Reads a `File` and retrieves a base64 representation
/// of the file
pub fn file_to_base64<P: AsRef<Path>>(file_path: P) -> String {
  let bytes = read_file(file_path).unwrap();
  let mut wrapped_writer = Vec::new();

  {
    let mut enc = base64::write::EncoderWriter::new(
      &mut wrapped_writer, base64::STANDARD);
  
    enc.write_all(bytes.as_slice()).unwrap();
    enc.finish().unwrap();
  }

  base64::encode(wrapped_writer)
}
