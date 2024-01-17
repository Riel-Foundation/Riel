use std::fs::File;
use std::collections::HashMap;
use crate::mergers::filemerger::{CommitMetadata, CommitModification};
pub fn metadata_to_abstraction(metadata: &File) -> CommitMetadata {
  todo!()
}
pub fn abstraction_to_metadata(object: &CommitMetadata) -> Vec<File> {
  todo!()
}