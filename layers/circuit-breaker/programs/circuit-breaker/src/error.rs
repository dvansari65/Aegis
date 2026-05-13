#[derive(Clone, Copy)]
#[derive(Debug)]
pub enum CircuitBreakerError {
    InvalidInstruction = 0,
    NotEnoughAccounts = 1,
    Unauthorized = 2,
    InvalidStateData = 3,
}
