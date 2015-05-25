use ::Image;

pub trait ImageCollection {
  fn to_string(&self) -> String;
}

impl ImageCollection for Vec<Image> {
  fn to_string(&self) -> String {
    return self.iter()
      .filter(|i| !i.url.clone().unwrap().is_empty())
      .map(|i| format!("  {}", i.to_string()))
      .collect::<Vec<String>>().connect("\n")
  }
}
