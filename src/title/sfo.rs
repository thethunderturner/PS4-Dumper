use std::fmt;
use std::fs::File;
use std::io::Read;

// Read: https://www.psdevwiki.com/ps4/Param.sfo
pub struct SfoHeader {
    pub magic: [u8; 4],
    pub version: u32,
    pub key_table_offset: u32,
    pub data_table_offset: u32,
    pub index_table_entries: u32,
}

impl fmt::Debug for SfoHeader {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SfoHeader")
            .field("magic", &String::from_utf8_lossy(&self.magic[1..]))
            .field("version", &format_args!("{:#010X}", self.version))
            .field(
                "key_table_offset",
                &format_args!("{:#X}", self.key_table_offset),
            )
            .field(
                "data_table_offset",
                &format_args!("{:#X}", self.data_table_offset),
            )
            .field("index_table_entries", &self.index_table_entries)
            .finish()
    }
}

pub fn read_sfo() -> std::io::Result<()> {
    let mut f = File::open("tmp/param.sfo")?;
    let mut header_buffer = [0u8; 20];

    // Read header
    f.read_exact(&mut header_buffer)?;
    let header = read_header(&header_buffer);

    println!("{:#?}", header);
    Ok(())
}

pub fn read_header(buffer: &[u8; 20]) -> SfoHeader {
    SfoHeader {
        magic: buffer[0..4].try_into().unwrap(),
        version: u32::from_le_bytes(buffer[4..8].try_into().unwrap()),
        key_table_offset: u32::from_le_bytes(buffer[8..12].try_into().unwrap()),
        data_table_offset: u32::from_le_bytes(buffer[12..16].try_into().unwrap()),
        index_table_entries: u32::from_le_bytes(buffer[16..20].try_into().unwrap()),
    }
}