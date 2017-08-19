extern crate serde;

#[derive(Serialize, Deserialize)]
#[derive(Default, Debug)]
pub struct Query {
    pub timestamp: String,
    pub lines: Vec<Line>,
}

#[derive(Serialize, Deserialize)]
#[derive(Default, Debug)]
pub struct Line {
    pub name: String,
    pub status: String,
    pub date: String,
    pub time: String,
    pub status_detail: Vec<StatusDetail>,
}

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
pub struct StatusDetail {
    pub text: String,
}
