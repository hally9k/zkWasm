#[derive(Clone)]
pub struct Status {
    pub eid: u64,
    pub moid: u16,
    pub fid: u16,
    pub iid: u16,
    pub mmid: u16,
    pub sp: u64,
    pub last_jump_eid: u64,
}

pub struct StepStatus<'a> {
    pub current: &'a Status,
    pub next: &'a Status,
}
