use super::fault::{SyscallFault, syscall_precheck};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FaultMatrixRow {
    pub num: u64,
    pub privileged: bool,
    pub expected: Result<(), SyscallFault>,
}

pub fn default_fault_matrix() -> Vec<FaultMatrixRow> {
    vec![
        FaultMatrixRow {
            num: 39,
            privileged: false,
            expected: Ok(()),
        },
        FaultMatrixRow {
            num: 165,
            privileged: false,
            expected: Err(SyscallFault::PolicyDenied),
        },
        FaultMatrixRow {
            num: 165,
            privileged: true,
            expected: Ok(()),
        },
        FaultMatrixRow {
            num: 166,
            privileged: false,
            expected: Err(SyscallFault::PolicyDenied),
        },
        FaultMatrixRow {
            num: 166,
            privileged: true,
            expected: Ok(()),
        },
    ]
}

pub fn validate_fault_matrix() -> bool {
    default_fault_matrix()
        .into_iter()
        .all(|r| syscall_precheck(r.num, r.privileged) == r.expected)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matrix_validation_passes() {
        assert!(validate_fault_matrix());
    }
}
