#[cfg(test)]
mod tests {
    use super::super::checkpoint::Checkpoint;
    use std::path::Path;
    use tempfile::NamedTempFile;

    #[test]
    fn test_default_offset_is_zero() {
        let cp = Checkpoint::new();
        assert_eq!(cp.offset_for(Path::new("/var/log/app.log")), 0);
    }

    #[test]
    fn test_set_and_get_offset() {
        let mut cp = Checkpoint::new();
        let p = Path::new("/var/log/app.log");
        cp.set_offset(p, 4096);
        assert_eq!(cp.offset_for(p), 4096);
    }

    #[test]
    fn test_overwrite_offset() {
        let mut cp = Checkpoint::new();
        let p = Path::new("/var/log/app.log");
        cp.set_offset(p, 100);
        cp.set_offset(p, 9999);
        assert_eq!(cp.offset_for(p), 9999);
    }

    #[test]
    fn test_len_and_is_empty() {
        let mut cp = Checkpoint::new();
        assert!(cp.is_empty());
        cp.set_offset(Path::new("/a"), 1);
        cp.set_offset(Path::new("/b"), 2);
        assert_eq!(cp.len(), 2);
        assert!(!cp.is_empty());
    }

    #[test]
    fn test_save_and_load_roundtrip() {
        let tmp = NamedTempFile::new().unwrap();
        let path = tmp.path();

        let mut cp = Checkpoint::load(path).unwrap();
        cp.set_offset(Path::new("/logs/a.log"), 512);
        cp.set_offset(Path::new("/logs/b.log"), 1024);
        cp.save().unwrap();

        let cp2 = Checkpoint::load(path).unwrap();
        assert_eq!(cp2.offset_for(Path::new("/logs/a.log")), 512);
        assert_eq!(cp2.offset_for(Path::new("/logs/b.log")), 1024);
        assert_eq!(cp2.len(), 2);
    }

    #[test]
    fn test_load_nonexistent_file_returns_empty() {
        let cp = Checkpoint::load(Path::new("/tmp/__logshear_no_such_checkpoint_xyz.ckpt")).unwrap();
        assert!(cp.is_empty());
    }

    #[test]
    fn test_in_memory_save_is_noop() {
        let mut cp = Checkpoint::new();
        cp.set_offset(Path::new("/x"), 42);
        // Should not error even though there's no backing file
        assert!(cp.save().is_ok());
    }

    #[test]
    fn test_unknown_path_returns_zero_after_load() {
        let tmp = NamedTempFile::new().unwrap();
        let path = tmp.path();
        let mut cp = Checkpoint::load(path).unwrap();
        cp.set_offset(Path::new("/known"), 77);
        cp.save().unwrap();

        let cp2 = Checkpoint::load(path).unwrap();
        assert_eq!(cp2.offset_for(Path::new("/unknown")), 0);
    }
}
