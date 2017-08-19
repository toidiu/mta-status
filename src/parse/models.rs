extern crate serde;


/// The fields in `Query` are declared as `T` rather than `&T`
/// and the signifance of this is that `Query` owns all it's
/// fields.
///
/// Imagine the counter scenario where `Query` only has a
/// reference to `lines` which is owned by some other part
/// of the codebase. We now have to ensure that that
/// `lines` lives atleast as long as `Query`. Additionally
/// we also need to worry if the data within `lines` is
/// being modified.
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
    pub status_details: Vec<StatusDetail>,
}

impl Line {
    #[allow(dead_code)]
    fn default() -> Self {
        Line { ..Default::default() }
    }
}

/// Even though this only contains one field at the moment,
/// it can be expanded by parsing the response in more detail.
#[derive(Serialize, Deserialize)]
#[derive(Debug)]
pub struct StatusDetail {
    pub text: String,
}
