pub struct Settings {
    pub width: usize,
    pub height: usize,
    pub scalar: usize,
}

impl Settings {
    pub fn new(width: usize, height: usize, scalar: Option<usize>) -> Settings {
        let scalar = match scalar {
            Some(x) => x,
            None => 4
        };

        Settings {
            width,
            height,
            scalar,
        }
    }
}