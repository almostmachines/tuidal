#[derive(Clone, Debug, PartialEq)]
pub struct State {
    pub title: String,
}

impl Default for State {
    fn default() -> Self {
        Self {
            title: String::from("Tuidal"),
        }
    }
}
