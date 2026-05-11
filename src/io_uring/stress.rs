use super::{IoOp, IoUringEngine};

pub fn run_stress(iter: u32) -> bool {
    let mut e = IoUringEngine::default();
    for i in 0..iter {
        if i % 2 == 0 {
            e.submit_with_len(IoOp::Read, 128);
        } else {
            e.submit_with_len(IoOp::Write, 256);
        }
    }
    e.complete_batch() as u32 == iter && e.cqe_count() as u32 == iter
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stress_small() {
        assert!(run_stress(32));
    }
}
