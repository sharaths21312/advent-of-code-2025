pub trait StrExt<'a> {
    fn unsafe_parse(self) -> i64;
    fn lstrip_parse(self, strip: &str) -> i64;
    fn rstrip_parse(self, strip: &str) -> i64;
    fn clean_split(self, pat: &str) -> Vec<&'a str>;
    fn interval_split(self, interval: usize) -> Vec<&'a str>;
}

impl<'a> StrExt<'a> for &'a str {
    fn clean_split(self, pat: &str) -> Vec<&'a str> {
        self.split(pat)
            .filter(|s| *s != "")
            .collect()
    }
    fn unsafe_parse(self) -> i64 {
        self.parse().unwrap()
    }
    fn lstrip_parse(self, strip: &str) -> i64 {
        self.strip_prefix(strip).unwrap().parse().unwrap()
    }
    fn rstrip_parse(self, strip: &str) -> i64 {
        self.strip_suffix(strip).unwrap().parse().unwrap()
    }

    fn interval_split(self, interval: usize) -> Vec<&'a str> {
        let mut output: Vec<&'a str> = vec![];
        if interval == 0 { panic!("Zero interval in split") }
        let mut rest = self;
        while rest.len() >= interval {
            let out = rest.split_at(interval);
            output.push(out.0);
            rest = out.1;
        }
        output
    }
}