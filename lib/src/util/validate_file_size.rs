use std::fs::File;

/// The max size of a single attachment in bytes
const MAX_ATTACHMENT_SIZE_BYTES: u64 = 15728640;

/// The equivalent of 1 Byte in Megabytes
const BYTE_IN_MEGABYTES: f32 = 0.00000095367432;

/// Check if the size of the file is too heavy to be sent as an
/// attachment.
/// 
/// This function returns a tuple where the first value is a `bool`,
/// this `bool` will be `true` if the file is too heavy (more than 15 MB of size),
/// otherwise returns `false`.
/// 
/// The second value of the tuple is a `usize` which represents the current size
/// of the provided file in MB
/// 
/// This function will neither panic or return an `Error`, otherwise this function
/// will return `(false, 0)` when an error occurs.
pub fn validate_file_size<'a>(file: &'a File) -> (bool, f32) {
  match file.metadata() {
    Ok(file_metadata) => {
      let size = file_metadata.len();
      let size_in_mb = size as f32 * BYTE_IN_MEGABYTES;

      if size > MAX_ATTACHMENT_SIZE_BYTES {

        return (true, size_in_mb);
      }

      (false, size_in_mb)
    },
    Err(_) => {
      // unable to read file and gather metadata,
      // returns `false` and `0`
      (false, 0_f32)
    }
  }
}
