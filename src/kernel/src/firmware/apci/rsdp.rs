/// Root system description pointer (RSDP).
/// TODO: The last 4 entries only exist on version 2.0
#[derive(Clone, Copy, Debug)]
#[repr(packed)]
pub struct RootSysDescPtr {
    /// Must be "RSD PTR" with a trailing blank character.
    signature: [u8; 8],
    /// Checksum of the fields defined in the APCI 1.0 specification. Includes only first 20 bytes
    /// of this table (including checksum field). These bytes must sum to 0.
    checksum: u8,
    /// OEM-supplied string that identifies the OEM.
    oem_id: [u8; 6],
    /// The revision of the structure. Larger revision numbers backwards compatible to lower
    /// revision numbers.
    /// APCI Version 1.0: revision 0
    revision: u8,
    /// Physical address of the RSDT.
    rsdt_address: u32,
    /// Length of the table, in bytes, including the header.
    ///
    /// #[note]
    /// Not available in the APCI version 1.0 RSDP structure.
    length: u32,
    /// Physical address of the XSDT.
    xsdt_address: u64,
    /// Checksum of the entire table, including both checksum fields.
    extended_chksum: u8,
    /// Reserved field.
    xx: [u8; 3],

}
