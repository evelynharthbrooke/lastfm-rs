#![doc(hidden)]
#![macro_use]

macro_rules! add_param {
    ($f:ident, $p:ident, $t:ty) => {
        pub fn $f(&'a mut self, v: $t) -> &'a mut Self {
            self.url.query_pairs_mut().append_pair(stringify!($p), &*v.to_string());
            self
        }
    };
}
