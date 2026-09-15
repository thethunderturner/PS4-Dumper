// Read: https://www.psdevwiki.com/ps4/Param.sfo
pub struct SfoHeader {
    pub magic: [u8; 4],
    pub version: u32,
    pub key_table_offset: u32,
    pub data_table_offset: u32,
    pub index_table_entries: u32,
}