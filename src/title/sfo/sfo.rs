use crate::title::sfo::header::read_header;
use crate::title::sfo::index_table::read_index;
use crate::title::sfo::key_table::read_key_table;
use std::fmt;
use std::fs::File;
use std::io::Read;

// Read: https://www.psdevwiki.com/ps4/Param.sfo
/*
   Short description
   External Structure: An outer container, that may surround the SFO data in some contexts, like on a disc.
   The wiki itself labels this section as speculation and shows the actual SFO beginning later inside that wrapper, at offset 0x800.

   Internal Structure: The actual contents of param.sfo.
       Header: Header metadata
       Index table: Tells you where everything is
       Key table: Stores the names like TITLE, APP_VER
       Data table Stores the actual values like "Bloodborne", "01.09"
*/
pub fn read_sfo() -> std::io::Result<()> {
    let mut f = File::open("tmp/param-fifa15.sfo")?;

    // Read header
    let header = read_header(&mut f)?;
    println!("{:#?}", header);

    // Read index table
    let index_table = read_index(&mut f, header.index_table_entries)?;
    println!("{:#?}", index_table);

    // Read key table
    let key_table = read_key_table(&mut f, header.key_table_offset, header.data_table_offset)?;
    println!("{:#?}", key_table);

    Ok(())
}
