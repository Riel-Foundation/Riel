use crate::mergers::commit_abstractions::{CommitMetadata, CommitModification};
use crate::mergers::commitreader::{abstraction_to_metadata, metadata_to_abstraction};
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
#[allow(dead_code)]
use std::fs::File;
use std::fs::{self, read, read_to_string};
use std::hash::{Hash, Hasher};
use std::io::{BufRead, BufReader};
use std::path::Path;
pub fn try_merge(before: &File, after: &File) -> Option<CommitMetadata> {
    todo!()
}
