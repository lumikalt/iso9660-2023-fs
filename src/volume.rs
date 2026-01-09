use std::path::Path;

use crate::{
    block::BlockDevice,
    directory::{DirEntry, Directory, parse_directory},
    error::IsoError,
};

pub struct IsoFs {
    dev: BlockDevice,
    root: Directory,
}

impl IsoFs {
    pub fn open(path: impl AsRef<Path>) -> Result<Self, IsoError> {
        let mut dev = BlockDevice::open(path)?;

        let pvd = dev.read_block(16)?;

        if &pvd[1..6] != b"CD001" {
            return Err(IsoError::InvalidSignature);
        }

        let root_rec = &pvd[156..];
        let lba = u32::from_le_bytes(root_rec[2..6].try_into().unwrap());
        let _size = u32::from_le_bytes(root_rec[10..14].try_into().unwrap());

        let data = dev.read_block(lba)?;
        let entries = parse_directory(&data);

        Ok(Self {
            dev,
            root: Directory { entries },
        })
    }

    fn find_entry(&mut self, path: &str) -> Result<DirEntry, IsoError> {
        let mut current_dir = self.root.clone();
        let mut current_entry = None;

        for part in path.trim_start_matches('/').split('/') {
            let entry = current_dir
                .entries
                .iter()
                .find(|e| e.name.eq_ignore_ascii_case(part))
                .ok_or_else(|| IsoError::NotFound(path.into()))?
                .clone();

            current_entry = Some(entry.clone());

            if entry.is_dir {
                current_dir = self.load_directory(entry.lba, entry.size)?;
            }
        }

        current_entry.ok_or_else(|| IsoError::NotFound(path.into()))
    }

    pub fn read_file(&mut self, path: &str) -> Result<Vec<u8>, IsoError> {
        let entry = self.find_entry(path)?;
        if entry.is_dir {
            return Err(IsoError::NotADirectory(path.into()));
        }

        let mut data = vec![];
        let blocks =
            (entry.size as usize + self.dev.block_size as usize - 1) / self.dev.block_size as usize;

        for i in 0..blocks {
            data.extend(self.dev.read_block(entry.lba + i as u32)?);
        }

        data.truncate(entry.size as usize);
        Ok(data)
    }

    fn load_directory(&mut self, lba: u32, size: u32) -> Result<Directory, IsoError> {
        let blocks =
            (size as usize + self.dev.block_size as usize - 1) / self.dev.block_size as usize;

        let mut buf = vec![];

        for i in 0..blocks {
            buf.extend(self.dev.read_block(lba + i as u32)?);
        }

        buf.truncate(size as usize);
        Ok(Directory {
            entries: parse_directory(&buf),
        })
    }

    pub fn list_dir(&mut self, path: &str) -> Result<Vec<DirEntry>, IsoError> {
        if path == "/" {
            return Ok(self.root.entries.clone());
        }

        let entry = self.find_entry(path)?;
        if !entry.is_dir {
            return Err(IsoError::NotADirectory(path.into()));
        }

        let dir = self.load_directory(entry.lba, entry.size)?;
        Ok(dir.entries)
    }
}
