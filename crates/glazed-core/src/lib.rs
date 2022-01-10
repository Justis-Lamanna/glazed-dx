pub mod math;

/// Represents that this object has an associated ID
pub trait Id<Output = usize> {
    fn get_id(&self) -> Output;
}