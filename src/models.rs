
pub struct Course<'a> {
    pub course_code: &'a str,
    pub batches: Vec<Batch<'a>>,
}

pub struct Batch<'a> {
    pub evaluation_time: &'a str,
    pub evaluation_desc: &'a str,
    pub learning_descs: Vec<&'a str>,
}
