/// Provided by the kernel entry-point (which is architecture-specific) to the main kernel routine
/// that is architecture-independent.
pub struct Bootstrap {
    /// Number of mapped pages.
    pub page_count: usize,

    /// Memory address of kernel entry-point.
    pub entry: u64,
}
