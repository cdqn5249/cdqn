use cdqn_types::Cdu;
use tokio::fs::{File, OpenOptions};
use tokio::io::{AsyncWriteExt, BufWriter};
use anyhow::Result;

/// Manages writing CDUs to the append-only log file.
pub struct LogWriter {
    writer: BufWriter<File>,
}

impl LogWriter {
    /// Creates a new LogWriter, opening or creating the specified log file.
    pub async fn new(path: &str) -> Result<Self> {
        let file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(path)
            .await?;

        Ok(Self {
            writer: BufWriter::new(file),
        })
    }

    /// Serializes a CDU to a JSON string and appends it to the log file.
    /// Each entry is on its own line.
    pub async fn write_cdu(&mut self, cdu: &Cdu) -> Result<()> {
        // Serialize the CDU to a JSON string.
        let mut json_string = serde_json::to_string(cdu)?;
        // Add a newline character to separate entries.
        json_string.push('\n');

        // Write the bytes to the buffered writer.
        self.writer.write_all(json_string.as_bytes()).await?;
        // Ensure the buffer is flushed to the file on disk.
        self.writer.flush().await?;

        Ok(())
    }
}
