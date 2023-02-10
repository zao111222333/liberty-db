

pub trait Group {
    fn name(&self) -> &String;
}

pub trait Check{
    fn check(&self);
}

pub trait Builder{
    fn build(&self);
}