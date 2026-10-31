use serde::Serialize;

#[allow(unused)]


pub trait DataTemplate: Serialize {

}

/// Empty
#[derive(Serialize)]
pub struct Empty {

}
impl DataTemplate for Empty {}