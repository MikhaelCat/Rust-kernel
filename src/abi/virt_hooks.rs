#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VirtHookResult {
    Ok,
    Unsupported,
}

pub fn kvm_create_vm_hook(enabled: bool) -> VirtHookResult {
    if enabled {
        VirtHookResult::Ok
    } else {
        VirtHookResult::Unsupported
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hook_basic() {
        assert_eq!(kvm_create_vm_hook(true), VirtHookResult::Ok);
        assert_eq!(kvm_create_vm_hook(false), VirtHookResult::Unsupported);
    }
}
