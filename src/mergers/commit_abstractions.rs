use crate::utils::datetime::DateTime;
pub struct CommitMetadata {
    pub hash: String,
    pub message: String,
    pub author: String,
    pub modifications: Vec<CommitModification>
}
#[derive(Clone)]
pub struct CommitModification { 
    pub date: DateTime,
    pub adds: Vec<CommitLine>,
    pub removes: Vec<CommitLine>,
}
impl CommitModification {
    pub fn compare(&self, other: &CommitModification) -> CommitModification {
        let clone1: CommitModification = self.clone();
        let clone2: CommitModification = other.clone();
        let difference: bool = self.date > other.date;
        
        if !difference {
            // they are equal
            /*let hash1: u64 = hash_string(&self.changes.join(""));
            let hash2: u64 = hash_string(&other.changes.join(""));
            fn hash_string(string: &str) -> u64 {
                let mut hasher = DefaultHasher::new();
                string.hash(&mut hasher);
                hasher.finish()
            }
            if hash1 > hash2 {
                return clone1;
            } else {
                return clone2;
            }*/
            todo!()
        } else {
            // they are not equal
            if difference {
                // self is greater
                return clone1;
            } else {
                // other is greater
                return clone2;
            }
            }
        }
}
#[derive(Clone)]
pub struct CommitLine {
    pub line: String, 
    pub line_number: u32,
}