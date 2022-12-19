use compact_str::CompactString;

pub trait Group {
    fn name(&self) -> &CompactString;
}