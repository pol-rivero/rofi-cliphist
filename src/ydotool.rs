use anyhow::{bail, Context};
use log::trace;

pub struct Ydotool {
    bin: String,
}

pub fn new(bin: impl Into<String>) -> Ydotool {
    Ydotool { bin: bin.into() }
}

impl Ydotool {
    pub fn paste(&self) -> anyhow::Result<()> {
        trace!("Pasting from clipboard");
        // "29:1 47:1 47:0 29:0" = Ctrl+V
        // Alternatively, we could use "42:1 110:1 110:0 42:0" = Shift+Insert
        let mut child = std::process::Command::new(&self.bin)
            .arg("key")
            .arg("29:1")
            .arg("47:1")
            .arg("47:0")
            .arg("29:0")
            .spawn()
            .context("Error executing ydotool")?;

        let status = child.wait().context("Error executing ydotool")?;
        if !status.success() {
            bail!("Error executing ydotool");
        }
        Ok(())
    }
}
