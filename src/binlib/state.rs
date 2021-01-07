use std::collections::{HashMap, HashSet};
use std::fs;
use std::io::{BufWriter, Write};
use std::ops::Add;
use std::path::Path;

use anyhow::{ensure, Result};

use semver::Version;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct StateEntry {
    pub name: String,
    pub artifacts: Vec<String>,
    pub url: String,
    pub version: Version,
}

pub struct State {
    path: String,
    internal_data: HashMap<String, StateEntry>,
}

impl State {
    pub fn new(path: &str) -> Result<State> {
        let mut s = State {
            path: String::from(path),
            internal_data: HashMap::new(),
        };
        s.acquire_lock()?;
        s.refresh()?;

        Ok(s)
    }

    fn acquire_lock(&self) -> Result<()> {
        let lock_str = self.path.clone().add(".lock");
        let lock_path = Path::new(&lock_str);

        ensure!(!lock_path.exists(), "Lock is already acquired");

        let mut file_handle = fs::File::create(lock_path)?;
        file_handle.write_all(b"lock")?;
        Ok(())
    }

    fn release_lock(&self) -> Result<()> {
        let lock_str = self.path.clone().add(".lock");
        let lock_path = Path::new(&lock_str);

        ensure!(lock_path.exists(), "Attempted to release a free lock");
        fs::remove_file(lock_path)?;

        Ok(())
    }

    fn refresh(&mut self) -> Result<()> {
        self.internal_data = HashMap::new();
        if let Ok(contents) = fs::read_to_string(&self.path) {
            if let Ok(internal_data) = serde_json::from_str(&contents) {
                self.internal_data = internal_data;
            }
        }
        Ok(())
    }

    pub fn get(&self, name: &str) -> Option<&StateEntry> {
        self.internal_data.get(name)
    }

    pub fn get_copy(&self, name: &str) -> Option<StateEntry> {
        self.internal_data.get(name).cloned()
    }

    pub fn list(&self) -> Vec<&StateEntry> {
        self.internal_data.iter().map(|(_, v)| v).collect()
    }

    pub fn insert(&mut self, mut entry: StateEntry) -> Result<()> {
        // Will throw if entry already exists.
        ensure!(
            !self.internal_data.contains_key(&entry.name),
            "Target {} already in state",
            &entry.name
        );

        // De-duplicate entry artifacts.
        let mut v = Vec::new();
        let mut hsh = HashSet::new();
        for itm in entry.artifacts.into_iter() {
            if !hsh.contains(&itm) {
                hsh.insert(itm.clone());
                v.push(itm);
            }
        }
        entry.artifacts = v;
        self.internal_data.insert(entry.name.clone(), entry);
        self.save()
    }

    fn save(&self) -> Result<()> {
        let file_handle = fs::File::create(&self.path)?;
        serde_json::to_writer(BufWriter::new(file_handle), &self.internal_data)?;
        Ok(())
    }

    pub fn remove(&mut self, entry_name: &str) -> Result<()> {
        if self.internal_data.contains_key(entry_name) {
            self.internal_data.remove(entry_name);
            self.save()?
        }
        Ok(())
    }
}

impl Drop for State {
    fn drop(&mut self) {
        match self.release_lock() {
            Ok(_) => {}
            Err(e) => {
                eprintln!("{}", e);
            }
        }
    }
}
