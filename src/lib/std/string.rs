pub trait Capitalize {
    fn capitalize(&mut self);
}

fn to_uppercase(ch: char) -> char {
    ch.to_uppercase().next().unwrap()
}

fn to_lowercase(ch: char) -> char {
    ch.to_lowercase().next().unwrap()
}

impl Capitalize for String {
    fn capitalize(&mut self) {
        let mut buf = String::with_capacity(self.len());
        let mut chars = self.chars();

        buf.push(to_uppercase(chars.next().unwrap()));
        buf.extend(chars.map(|ch| to_lowercase(ch)));

        *self = buf;
    }
}
