#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MigrationPlan {
    pub vm_id: u64,
    pub source: String,
    pub target: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MigrationResult {
    Success,
    Reject,
}

pub fn validate_plan(plan: &MigrationPlan) -> MigrationResult {
    if plan.source != plan.target && !plan.source.is_empty() && !plan.target.is_empty() {
        MigrationResult::Success
    } else {
        MigrationResult::Reject
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn migration_plan_validation() {
        let ok = MigrationPlan {
            vm_id: 1,
            source: "node-a".into(),
            target: "node-b".into(),
        };
        let bad = MigrationPlan {
            vm_id: 1,
            source: "node-a".into(),
            target: "node-a".into(),
        };
        assert_eq!(validate_plan(&ok), MigrationResult::Success);
        assert_eq!(validate_plan(&bad), MigrationResult::Reject);
    }
}
