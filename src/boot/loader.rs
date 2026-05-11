use super::elf::ElfImage;
use super::entry::EntryPoint;
use super::error::BootError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LoaderState {
    Start,
    ImageParsed,
    KernelLoaded,
    Ready,
}

#[derive(Debug, Clone)]
pub struct BootLoader {
    state: LoaderState,
    entry: EntryPoint,
}

impl BootLoader {
    pub fn new() -> Self {
        Self {
            state: LoaderState::Start,
            entry: EntryPoint(0),
        }
    }

    pub fn load_image(&mut self, image_bytes: &[u8]) -> Result<(), BootError> {
        let image = ElfImage::parse(image_bytes)?;
        self.state = LoaderState::ImageParsed;
        self.entry = EntryPoint(image.entry);

        if self.entry.is_null() {
            return Err(BootError::InvalidElfImage);
        }

        self.state = LoaderState::KernelLoaded;
        self.state = LoaderState::Ready;
        Ok(())
    }

    pub fn state(&self) -> LoaderState {
        self.state.clone()
    }

    pub fn entry(&self) -> EntryPoint {
        self.entry
    }
}

impl Default for BootLoader {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn loader_reaches_ready() {
        let mut img = vec![0u8; 64];
        img[0..4].copy_from_slice(&[0x7f, b'E', b'L', b'F']);
        img[24..32].copy_from_slice(&0x8000u64.to_le_bytes());

        let mut loader = BootLoader::new();
        loader.load_image(&img).expect("load failed");
        assert_eq!(loader.state(), LoaderState::Ready);
        assert_eq!(loader.entry().0, 0x8000);
    }

    #[test]
    fn loader_rejects_null_entry() {
        let mut img = vec![0u8; 64];
        img[0..4].copy_from_slice(&[0x7f, b'E', b'L', b'F']);
        let mut loader = BootLoader::new();
        assert_eq!(loader.load_image(&img), Err(BootError::InvalidElfImage));
    }
}
