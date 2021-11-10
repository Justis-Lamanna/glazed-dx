pub trait Indexed {
    fn index(&self) -> u16;
}

#[derive(Debug)]
pub enum Season {
    Spring, Summer, Autumn, Winter
}