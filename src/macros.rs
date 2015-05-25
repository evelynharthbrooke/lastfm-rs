macro_rules! to_option {
  ($e:expr) => (match $e {
    Ok(val) => Some(val),
    Err(_) => None,
  }.unwrap_or(None));
}
