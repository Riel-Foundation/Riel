use std::fs::File;
use std::collections::HashMap;
use crate::mergers::filemerger::{CommitMetadata, CRDT};
pub fn metadata_to_object(metadata: &File) -> CommitMetadata {
  todo!()
}
pub fn object_to_metadata(object: &CommitMetadata) -> Vec<File> {
  // TODO: Remove all clones
  let hash: String = object.hash.clone();
  let msg: String = object.message.clone();
  let author: String = object.author.clone();
  let data: HashMap<String, CRDT> = object.crdtdata.clone();
  let concerned_files: Vec<String> = data.keys().map(|x| x.clone()).collect();
  
  todo!()
}