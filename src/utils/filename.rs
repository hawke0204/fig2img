pub fn sanitize(name: &str) -> String {
  name.replace(['/', '\\', ':', '*', '?', '"', '<', '>', '|'], "_")
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_sanitize() {
    assert_eq!(sanitize("hello/world"), "hello_world");
    assert_eq!(sanitize("test:file*"), "test_file_");
    assert_eq!(sanitize("normal.png"), "normal.png");
  }
}
