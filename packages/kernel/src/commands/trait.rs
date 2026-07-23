/// Kernel command trait for state-changing operations.
pub trait Command {
    fn name(&self) -> &'static str;
}
