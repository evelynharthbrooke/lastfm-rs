macro_rules! to_option {
  ($e:expr) => (match $e {
    Ok(val) => Some(val),
    Err(_) => None,
  }.unwrap_or(None));
}

macro_rules! debug {
  ($e:expr) => (match $e {
    Some(ref val) => format!("{}", val),
    None => format!("undefined")
  });
}

macro_rules! debug_s {
  ($e:expr) => (match $e {
    Some(ref val) => format!("{}", val.to_string()),
    None => format!("undefined")
  });
}
