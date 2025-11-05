#[derive(Clone, Debug)]
pub struct Map_Range<'a> {
    pub num: &'a str,
    pub seen: Vec<Vec<usize>>

}