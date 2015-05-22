pub struct SearchResults<'a, T> {
  pub query: &'a str,
  pub results: Vec<T>
}
