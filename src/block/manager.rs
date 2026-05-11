use super::bio::Bio;
use super::device::BlockDevice;
use super::error::BlockError;
use super::request::BlockRequest;

#[derive(Debug, Default)]
pub struct BlockManager {
    pub device: Option<BlockDevice>,
    inflight: Vec<BlockRequest>,
    completed: Vec<BlockRequest>,
    next_id: u64,
}

impl BlockManager {
    pub fn attach_device(&mut self, dev: BlockDevice) {
        self.device = Some(dev);
    }

    pub fn submit_bio(&mut self, bio: Bio) -> Result<(), BlockError> {
        if self.device.is_none() {
            return Err(BlockError::DeviceNotSet);
        }
        if bio.bytes == 0 {
            return Err(BlockError::InvalidRequest);
        }
        self.next_id += 1;
        self.inflight.push(BlockRequest::new(
            self.next_id,
            bio.sector,
            bio.bytes as u32,
        ));
        Ok(())
    }

    pub fn inflight(&self) -> usize {
        self.inflight.len()
    }

    pub fn complete_one(&mut self) -> Option<u64> {
        if self.inflight.is_empty() {
            return None;
        }
        let req = self.inflight.remove(0);
        let id = req.id;
        self.completed.push(req);
        Some(id)
    }

    pub fn complete_all(&mut self) -> usize {
        let mut done = 0usize;
        while self.complete_one().is_some() {
            done += 1;
        }
        done
    }

    pub fn completed(&self) -> usize {
        self.completed.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn submit_requires_device() {
        let mut m = BlockManager::default();
        assert_eq!(
            m.submit_bio(Bio::new(0, 512)),
            Err(BlockError::DeviceNotSet)
        );
    }

    #[test]
    fn submit_after_attach() {
        let mut m = BlockManager::default();
        m.attach_device(BlockDevice::new("sda", 512));
        m.submit_bio(Bio::new(8, 512)).expect("submit failed");
        assert_eq!(m.inflight(), 1);
        assert_eq!(m.complete_all(), 1);
        assert_eq!(m.completed(), 1);
    }
}
