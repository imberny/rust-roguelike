use super::Action;

#[derive(Debug, Default)]
pub struct Activity {
    pub time_to_complete: i32,
    pub action: Action,
}
