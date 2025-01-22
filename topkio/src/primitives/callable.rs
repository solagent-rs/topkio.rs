pub trait Callable {
    type Parameter;

    fn name(&self) -> String;
    fn description(&self) -> String;
    fn parameters(&self) -> Self::Parameter;
}

pub trait Invoke {
    fn call(&self);
}
