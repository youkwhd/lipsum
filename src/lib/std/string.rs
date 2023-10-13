pub trait Capitalize {
    fn capitalize(&self) -> String;
}

impl Capitalize for String {
    fn capitalize(&self) -> String {
        let mut buf = String::with_capacity(self.len());
        let mut chars = self.chars();

        buf.push(chars.next().unwrap().to_uppercase().next().unwrap());

        for c in chars {
            buf.push(c);
        }

        buf
    }
}
