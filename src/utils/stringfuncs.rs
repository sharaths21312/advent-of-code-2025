pub trait StrExt {
    fn unsafe_parse(self) -> i64;
    fn lstrip_parse(self, strip: &str) -> i64;
    fn rstrip_parse(self, strip: &str) -> i64;
}

impl StrExt for &str {
    fn unsafe_parse(self) -> i64 {
        self.parse().unwrap()
    }
    fn lstrip_parse(self, strip: &str) -> i64 {
        self.strip_prefix(strip).unwrap().parse().unwrap()
    }
    fn rstrip_parse(self, strip: &str) -> i64 {
        self.strip_suffix(strip).unwrap().parse().unwrap()
    }
}