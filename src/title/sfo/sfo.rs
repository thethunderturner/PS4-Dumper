use crate::title::sfo::header::read_header;
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
    let mut header_buffer = [0u8; 20];

    // Read header
    f.read_exact(&mut header_buffer)?;
    let header = read_header(&header_buffer);

    println!("{:#?}", header);
    Ok(())
}
