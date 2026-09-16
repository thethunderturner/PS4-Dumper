use std::fs::File;
use std::io::Read;

#[derive(Debug)]
pub struct SfoHeader {
    pub magic: String,
    pub version: String,
    pub key_table_offset: String,
    pub data_table_offset: String,
    pub index_table_entries: u32, // Number of entries
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
        magic: String::from_utf8_lossy(&buffer[1..4]).into_owned(),
        version: format!(
            "0x{:08X}",
            u32::from_le_bytes(buffer[4..8].try_into().unwrap())
        ),
        key_table_offset: format!(
            "0x{:X}",
            u32::from_le_bytes(buffer[8..12].try_into().unwrap())
        ),
        data_table_offset: format!(
            "0x{:X}",
            u32::from_le_bytes(buffer[12..16].try_into().unwrap())
        ),
        index_table_entries: u32::from_le_bytes(buffer[16..20].try_into().unwrap()),
    }
}
