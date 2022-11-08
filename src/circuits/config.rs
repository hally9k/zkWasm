use std::str::FromStr;

lazy_static! {
    pub static ref K: u32 = {
        let k = std::env::var("ZKWASM_K")
            .map(|x| u32::from_str(&x).unwrap())
            .unwrap_or(18);
        println!("K is {}", k);
        k
    };
    pub static ref MAX_ETABLE_ROWS: usize = (1usize << (*K - 2)) * 3;
    pub static ref MAX_MATBLE_ROWS: usize = (1usize << (*K - 2)) * 3;
    pub static ref MAX_JATBLE_ROWS: usize = (1usize << (*K - 2)) * 3;
    pub static ref POW_TABLE_LIMIT: u64 = 128;
}

pub const VAR_COLUMNS: usize = 19;
pub const IMTABLE_COLOMNS: usize = 2;
