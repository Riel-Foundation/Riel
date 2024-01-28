#[allow(dead_code)]
use std::fs::File;
use std::collections::HashMap;
use std::fs::{read, read_to_string, self};
use std::io::{BufRead, BufReader};
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;
use std::path::Path;
use crate::mergers::commitreader::{metadata_to_abstraction, abstraction_to_metadata};
use crate::mergers::commit_abstractions::{CommitMetadata, CommitModification};
pub fn try_merge(before: &File, after: &File) -> Option<CommitMetadata> {
    todo!()
}
