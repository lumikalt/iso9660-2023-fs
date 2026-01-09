use std::{
    fs::File,
    io::{self, Read, Seek, SeekFrom},
    path::Path,
};

pub struct BlockDevice {
    file: File,
    ///  DVD-Video mandates 2048, but not ISO9660. Use this to read blocks.
    pub block_size: u16,
}

impl BlockDevice {
    pub fn open(path: impl AsRef<Path>) -> io::Result<Self> {
        let mut file = File::open(path)?;
        let mut pvd = [0u8; 2048];

        file.seek(SeekFrom::Start(16 * 2048))?;
        file.read_exact(&mut pvd)?;

        let block_size = u16::from_le_bytes([pvd[128], pvd[129]]);

        Ok(Self { file, block_size })
    }

    pub fn read_block(&mut self, lba: u32) -> std::io::Result<Vec<u8>> {
        let mut buf = vec![0u8; self.block_size as usize];
        self.file
            .seek(SeekFrom::Start(lba as u64 * self.block_size as u64))?;
        self.file.read_exact(&mut buf)?;
        Ok(buf)
    }
}
