use crate::Progress;
use std::error::Error;
use std::sync::Arc;

#[derive(Debug)]
pub enum StepAction {
    Cancel,
    Complete,
    Continue,
}

pub trait UpdateStep<T> {
    fn exec(&self, data: &mut T, progress: &Arc<Progress>) -> Result<StepAction, Box<dyn Error>>;

    fn verify(&self, _: &T) -> bool {
        true
    }

    fn label(&self, data: &T) -> String;
}
