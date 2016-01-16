use reporter;

#[derive(Debug)]
pub struct WorldState {
    pub reporter: reporter::Reporter,
    pub failed: bool,
}
