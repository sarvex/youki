use super::{create, delete, kill};
use crate::utils::TempDir;
use crate::utils::{generate_uuid, prepare_bundle};
use test_framework::{TestResult, TestableGroup};

pub struct ContainerCreate {
    project_path: TempDir,
    container_id: String,
}

impl Default for ContainerCreate {
    fn default() -> Self {
        Self::new()
    }
}

impl ContainerCreate {
    pub fn new() -> Self {
        let id = generate_uuid();
        let temp_dir = prepare_bundle(&id).unwrap();
        ContainerCreate {
            project_path: temp_dir,
            container_id: id.to_string(),
        }
    }

    // runtime should not create container with empty id
    fn create_empty_id(&self) -> TestResult {
        let temp = create::create(&self.project_path, "");
        match temp {
            TestResult::Passed => TestResult::Failed(anyhow::anyhow!(
                "Container should not have been created with empty id, but was created."
            )),
            TestResult::Failed(_) => TestResult::Passed,
            TestResult::Skipped => TestResult::Skipped,
        }
    }

    // runtime should create container with valid id
    fn create_valid_id(&self) -> TestResult {
        let temp = create::create(&self.project_path, &self.container_id);
        if let TestResult::Passed = temp {
            kill::kill(&self.project_path, &self.container_id);
            delete::delete(&self.project_path, &self.container_id);
        }
        temp
    }

    // runtime should not create container with is that already exists
    fn create_duplicate_id(&self) -> TestResult {
        let id = generate_uuid().to_string();
        let _ = create::create(&self.project_path, &id);
        let temp = create::create(&self.project_path, &id);
        kill::kill(&self.project_path, &id);
        delete::delete(&self.project_path, &id);
        match temp {
            TestResult::Passed => TestResult::Failed(anyhow::anyhow!(
                "Container should not have been created with same id, but was created."
            )),
            TestResult::Failed(_) => TestResult::Passed,
            TestResult::Skipped => TestResult::Skipped,
        }
    }
}

impl TestableGroup for ContainerCreate {
    fn get_name(&self) -> &'static str {
        "create"
    }

    fn run_all(&self) -> Vec<(&'static str, TestResult)> {
        vec![
            ("empty_id", self.create_empty_id()),
            ("valid_id", self.create_valid_id()),
            ("duplicate_id", self.create_duplicate_id()),
        ]
    }

    fn run_selected(&self, selected: &[&str]) -> Vec<(&'static str, TestResult)> {
        let mut ret = Vec::new();
        for name in selected {
            match *name {
                "empty_id" => ret.push(("empty_id", self.create_empty_id())),
                "valid_id" => ret.push(("valid_id", self.create_valid_id())),
                "duplicate_id" => ret.push(("duplicate_id", self.create_duplicate_id())),
                _ => eprintln!("No test named {name} in lifecycle"),
            };
        }
        ret
    }
}
