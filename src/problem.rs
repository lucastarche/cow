pub type ProblemId = i64;

#[derive(Default, Debug)]
pub struct Problem {
    pub id: ProblemId,
    pub name: Option<String>,
    pub url: Option<String>,
    pub date: Option<String>,
    pub comment: Option<String>,
    pub code: Option<String>,
}
