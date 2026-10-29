use serde::Serialize;

pub trait DataTemplate: Serialize {

}

/// Empty
#[derive(Serialize)]
pub struct Empty {

}
impl DataTemplate for Empty {}