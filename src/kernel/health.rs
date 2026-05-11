#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KernelHealth {
    pub healthy: bool,
    pub reason: &'static str,
}

pub fn evaluate(running_tasks: usize) -> KernelHealth {
    if running_tasks == 0 {
        KernelHealth {
            healthy: false,
            reason: "no_running_tasks",
        }
    } else {
        KernelHealth {
            healthy: true,
            reason: "ok",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn health_check() {
        assert!(!evaluate(0).healthy);
        assert!(evaluate(1).healthy);
    }
}
