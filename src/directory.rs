#[derive(Debug, Clone)]
pub struct DirEntry {
    pub name: String,
    pub lba: u32,
    pub size: u32,
    pub is_dir: bool,
}

#[derive(Debug, Clone)]
pub struct Directory {
    pub entries: Vec<DirEntry>,
}

pub fn parse_directory(data: &[u8]) -> Vec<DirEntry> {
    let mut pos = 0;
    let mut entries = vec![];

    while pos < data.len() {
        let len = data[pos];
        if len == 0 {
            pos = ((pos / 2048) + 1) * 2048;
            continue;
        }

        let rec = &data[pos..pos + len as usize];

        let lba = u32::from_le_bytes(rec[2..6].try_into().unwrap());
        let size = u32::from_le_bytes(rec[10..14].try_into().unwrap());
        let flags = rec[25];
        let name_len = rec[32] as usize;
        let name = String::from_utf8_lossy(&rec[33..33 + name_len]).to_string();

        if name != "\u{0}" && name != "\u{1}" {
            entries.push(DirEntry {
                name: name.split(';').next().unwrap().to_string(),
                lba,
                size,
                is_dir: flags & 0x02 != 0,
            });
        }

        pos += len as usize;
    }

    entries
}
