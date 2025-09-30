//! Business Task entity

use serde::{Deserialize, Serialize};
use crate::models::{DeptCode, SectionCode, TaskId};

/// Business Task (業務タスク)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BusinessTask {
    /// タスク固有識別子
    pub id: TaskId,
    /// タスク名/説明
    pub description: String,
    /// 関連部門 (オプショナル、部門横断タスクの場合はNone)
    pub department: Option<DeptCode>,
    /// 関連課 (オプショナル、課横断タスクの場合はNone)
    pub section: Option<SectionCode>,
    /// アクティブ/非アクティブ状態
    pub active: bool,
}

impl BusinessTask {
    pub fn new(id: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            id: TaskId::new(id),
            description: description.into(),
            department: None,
            section: None,
            active: true,
        }
    }

    pub fn with_department(mut self, department: char) -> Self {
        self.department = Some(DeptCode::new(department));
        self
    }

    pub fn with_section(mut self, section: char) -> Self {
        self.section = Some(SectionCode::new(section));
        self
    }

    pub fn inactive(mut self) -> Self {
        self.active = false;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_business_task_new() {
        let task = BusinessTask::new("task001", "契約書作成");
        assert_eq!(task.id.0, "task001");
        assert_eq!(task.description, "契約書作成");
        assert!(task.department.is_none());
        assert!(task.active);
    }

    #[test]
    fn test_business_task_with_department_and_section() {
        let task = BusinessTask::new("task001", "部門タスク")
            .with_department('G')
            .with_section('I');
        assert_eq!(task.department.unwrap().0, 'G');
        assert_eq!(task.section.unwrap().0, 'I');
    }

    #[test]
    fn test_business_task_inactive() {
        let task = BusinessTask::new("task001", "古いタスク").inactive();
        assert!(!task.active);
    }
}
