pub struct WorldResult {
    pub failed: bool
}

impl WorldResult {
    pub fn new(failed: bool) -> WorldResult {
        WorldResult {
            failed: failed,
        }
    }
}
