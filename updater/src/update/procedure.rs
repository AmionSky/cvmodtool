use super::{StepAction, UpdateStep};
use crate::Progress;
use log::info;
use std::error::Error;
use std::sync::Arc;

pub struct UpdateProcedure<T> {
    title: String,
    progress: Arc<Progress>,
    steps: Vec<Box<dyn UpdateStep<T>>>,
    data: T,
}

impl<T> UpdateProcedure<T> {
    pub fn new(title: String, data: T) -> Self {
        UpdateProcedure {
            title,
            progress: Arc::new(Progress::default()),
            steps: Vec::new(),
            data,
        }
    }

    pub fn add_step(&mut self, step: Box<dyn UpdateStep<T>>) {
        self.steps.push(step)
    }

    pub fn title(&self) -> &String {
        &self.title
    }

    pub fn progress(&self) -> &Arc<Progress> {
        &self.progress
    }

    pub fn steps(&self) -> &Vec<Box<dyn UpdateStep<T>>> {
        &self.steps
    }

    pub fn data(&self) -> &T {
        &self.data
    }

    pub fn execute(&mut self) -> Result<(), Box<dyn Error>> {
        for step in &self.steps {
            self.progress.reset();

            match step.exec(&mut self.data, &self.progress)? {
                StepAction::Cancel => break,
                StepAction::Complete => break,
                StepAction::Continue => {}
            }

            if !step.verify(&self.data) {
                return Err("Verification failed".into());
            }

            if self.progress.cancelled() {
                break;
            }
        }

        self.progress.set_complete(true);
        if self.progress.cancelled() {
            info!("Update cancelled!")
        } else {
            info!("Update successful!");
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestData;

    struct TestStepContinue;
    impl UpdateStep<TestData> for TestStepContinue {
        fn exec(&self, _: &mut TestData, _: &Arc<Progress>) -> Result<StepAction, Box<dyn Error>> {
            Ok(StepAction::Continue)
        }
        fn label(&self, _: &TestData) -> String {
            "Test Continue Step".to_string()
        }
    }

    struct TestStepComplete;
    impl UpdateStep<TestData> for TestStepComplete {
        fn exec(&self, _: &mut TestData, _: &Arc<Progress>) -> Result<StepAction, Box<dyn Error>> {
            Ok(StepAction::Complete)
        }
        fn label(&self, _: &TestData) -> String {
            "Test Complete Step".to_string()
        }
    }

    struct TestStepCancel;
    impl UpdateStep<TestData> for TestStepCancel {
        fn exec(&self, _: &mut TestData, _: &Arc<Progress>) -> Result<StepAction, Box<dyn Error>> {
            Ok(StepAction::Cancel)
        }
        fn label(&self, _: &TestData) -> String {
            "Test Cancel Step".to_string()
        }
    }

    struct TestStepError;
    impl UpdateStep<TestData> for TestStepError {
        fn exec(&self, _: &mut TestData, _: &Arc<Progress>) -> Result<StepAction, Box<dyn Error>> {
            Err("Test Error".into())
        }
        fn label(&self, _: &TestData) -> String {
            "Test Error Step".to_string()
        }
    }

    #[test]
    fn test_procedure_ok() {
        let mut procedure = UpdateProcedure::new("Procedure Ok".to_string(), TestData);
        procedure.add_step(Box::new(TestStepContinue));
        procedure.add_step(Box::new(TestStepContinue));
        procedure.add_step(Box::new(TestStepContinue));
        assert!(procedure.execute().is_ok());
    }

    #[test]
    fn test_procedure_cancelled() {
        let mut procedure = UpdateProcedure::new("Procedure Cancelled".to_string(), TestData);
        procedure.add_step(Box::new(TestStepContinue));
        procedure.add_step(Box::new(TestStepCancel));
        procedure.add_step(Box::new(TestStepContinue));
        assert!(procedure.execute().is_ok());
    }

    #[test]
    fn test_procedure_err() {
        let mut procedure = UpdateProcedure::new("Procedure Error".to_string(), TestData);
        procedure.add_step(Box::new(TestStepContinue));
        procedure.add_step(Box::new(TestStepError));
        procedure.add_step(Box::new(TestStepContinue));
        assert!(procedure.execute().is_err());
    }

    #[test]
    fn test_procedure_early_complete() {
        let mut procedure = UpdateProcedure::new("Procedure Early Complete".to_string(), TestData);
        procedure.add_step(Box::new(TestStepContinue));
        procedure.add_step(Box::new(TestStepComplete));
        procedure.add_step(Box::new(TestStepError));
        assert!(procedure.execute().is_ok());
    }
}
