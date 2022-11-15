use halo2_proofs::{arithmetic::FieldExt, circuit::Region};

pub struct Context<'a, F: FieldExt> {
    pub region: Box<Region<'a, F>>,
    pub offset: usize,
    records: Vec<usize>,
}

impl<'a, F: FieldExt> Context<'a, F> {
    pub fn new(region: Region<'a, F>) -> Self {
        Self {
            region: Box::new(region),
            offset: 0usize,
            records: vec![],
        }
    }

    pub fn next(&mut self) {
        self.offset += 1;
    }

    pub fn reset(&mut self) {
        self.offset = 0;
        self.records.clear();
    }

    pub fn push(&mut self) {
        self.records.push(self.offset)
    }

    pub fn pop(&mut self) {
        self.offset = self.records.pop().unwrap();
    }
}
