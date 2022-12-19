use compact_str::CompactString;

pub trait Group {
    fn name(&self) -> &CompactString;
}

pub trait Check{
    fn check(&self);
}