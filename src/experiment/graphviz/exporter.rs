use std::io::Result;
use std::path::Path;

pub trait GraphvizExporter {
    fn export<P: AsRef<Path>>(&self, path: P) -> Result<()>;
}
