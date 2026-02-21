use std::path::Path;

use anyhow::{Context, Result};
use chrono::Utc;
use rusqlite::{params, Connection};

/// Pending bundle states
#[derive(Debug, PartialEq)]
pub enum BundleStatus {
    Pending,
    Sent,
    Failed,
}

impl BundleStatus {
    fn as_str(&self) -> &'static str {
        match self {
            BundleStatus::Pending => "pending",
            BundleStatus::Sent => "sent",
            BundleStatus::Failed => "failed",
        }
    }
}

/// SQLite-backed offline queue for FHIR bundles awaiting transmission.
///
/// Bundles are queued locally and retried for up to 7 days per DHA
/// offline-facility transmission window (Digital Health Regulations 2025).
pub struct OfflineQueue {
    conn: Connection,
}

impl OfflineQueue {
    /// Open (or create) the queue database at the given path.
    pub fn open(db_path: &Path) -> Result<Self> {
        let conn = Connection::open(db_path)
            .with_context(|| format!("Failed to open queue db at {:?}", db_path))?;

        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS pending_bundles (
                id          INTEGER PRIMARY KEY AUTOINCREMENT,
                bundle_id   TEXT NOT NULL,
                bundle_json TEXT NOT NULL,
                patient_id  TEXT NOT NULL,
                clinic_id   TEXT NOT NULL,
                created_at  TEXT NOT NULL,
                retry_count INTEGER NOT NULL DEFAULT 0,
                last_error  TEXT,
                status      TEXT NOT NULL DEFAULT 'pending'
            );
            CREATE INDEX IF NOT EXISTS idx_status ON pending_bundles(status);
            CREATE INDEX IF NOT EXISTS idx_created ON pending_bundles(created_at);",
        )
        .context("Failed to initialise queue schema")?;

        Ok(Self { conn })
    }

    /// Enqueue a bundle for later transmission.
    pub fn enqueue(
        &self,
        bundle_id: &str,
        bundle_json: &str,
        patient_id: &str,
        clinic_id: &str,
    ) -> Result<i64> {
        let now = Utc::now().to_rfc3339();
        self.conn.execute(
            "INSERT INTO pending_bundles
                (bundle_id, bundle_json, patient_id, clinic_id, created_at, status)
             VALUES (?1, ?2, ?3, ?4, ?5, 'pending')",
            params![bundle_id, bundle_json, patient_id, clinic_id, now],
        )?;
        Ok(self.conn.last_insert_rowid())
    }

    /// Retrieve all pending bundles not older than 7 days.
    pub fn pending_within_window(&self) -> Result<Vec<PendingBundle>> {
        let cutoff = (Utc::now() - chrono::Duration::days(7)).to_rfc3339();
        let mut stmt = self.conn.prepare(
            "SELECT id, bundle_id, bundle_json, patient_id, clinic_id,
                    created_at, retry_count, last_error
             FROM pending_bundles
             WHERE status = 'pending' AND created_at >= ?1
             ORDER BY created_at ASC",
        )?;

        let rows = stmt.query_map(params![cutoff], |row| {
            Ok(PendingBundle {
                row_id: row.get(0)?,
                bundle_id: row.get(1)?,
                bundle_json: row.get(2)?,
                patient_id: row.get(3)?,
                clinic_id: row.get(4)?,
                created_at: row.get(5)?,
                retry_count: row.get(6)?,
                last_error: row.get(7)?,
            })
        })?;

        rows.collect::<rusqlite::Result<Vec<_>>>()
            .context("Failed to query pending bundles")
    }

    /// Mark a bundle as successfully sent.
    pub fn mark_sent(&self, row_id: i64) -> Result<()> {
        self.conn.execute(
            "UPDATE pending_bundles SET status = 'sent' WHERE id = ?1",
            params![row_id],
        )?;
        Ok(())
    }

    /// Record a transmission failure and increment retry counter.
    pub fn record_failure(&self, row_id: i64, error: &str) -> Result<()> {
        self.conn.execute(
            "UPDATE pending_bundles
             SET retry_count = retry_count + 1,
                 last_error  = ?2,
                 status      = CASE
                     WHEN retry_count + 1 >= 10 THEN 'failed'
                     ELSE 'pending'
                 END
             WHERE id = ?1",
            params![row_id, error],
        )?;
        Ok(())
    }

    /// Expire bundles older than 7 days (mark as failed, not deleted — for audit).
    pub fn expire_old_bundles(&self) -> Result<usize> {
        let cutoff = (Utc::now() - chrono::Duration::days(7)).to_rfc3339();
        let n = self.conn.execute(
            "UPDATE pending_bundles
             SET status = 'failed', last_error = 'Transmission window (7 days) expired'
             WHERE status = 'pending' AND created_at < ?1",
            params![cutoff],
        )?;
        Ok(n)
    }

    /// Queue statistics for monitoring / web UI.
    pub fn stats(&self) -> Result<QueueStats> {
        let pending: i64 = self.conn.query_row(
            "SELECT COUNT(*) FROM pending_bundles WHERE status = 'pending'",
            [],
            |r| r.get(0),
        )?;
        let sent: i64 = self.conn.query_row(
            "SELECT COUNT(*) FROM pending_bundles WHERE status = 'sent'",
            [],
            |r| r.get(0),
        )?;
        let failed: i64 = self.conn.query_row(
            "SELECT COUNT(*) FROM pending_bundles WHERE status = 'failed'",
            [],
            |r| r.get(0),
        )?;
        Ok(QueueStats { pending, sent, failed })
    }
}

#[derive(Debug)]
pub struct PendingBundle {
    pub row_id: i64,
    pub bundle_id: String,
    pub bundle_json: String,
    pub patient_id: String,
    pub clinic_id: String,
    pub created_at: String,
    pub retry_count: i32,
    pub last_error: Option<String>,
}

#[derive(Debug)]
pub struct QueueStats {
    pub pending: i64,
    pub sent: i64,
    pub failed: i64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    fn open_temp_queue() -> (OfflineQueue, NamedTempFile) {
        let f = NamedTempFile::new().unwrap();
        let q = OfflineQueue::open(f.path()).unwrap();
        (q, f)
    }

    #[test]
    fn enqueue_and_list() {
        let (q, _f) = open_temp_queue();
        q.enqueue("b1", "{}", "p1", "c1").unwrap();
        q.enqueue("b2", "{}", "p2", "c1").unwrap();
        let rows = q.pending_within_window().unwrap();
        assert_eq!(rows.len(), 2);
    }

    #[test]
    fn mark_sent_removes_from_pending() {
        let (q, _f) = open_temp_queue();
        let id = q.enqueue("b1", "{}", "p1", "c1").unwrap();
        q.mark_sent(id).unwrap();
        let rows = q.pending_within_window().unwrap();
        assert!(rows.is_empty());
        let stats = q.stats().unwrap();
        assert_eq!(stats.sent, 1);
    }

    #[test]
    fn record_failure_increments_retry() {
        let (q, _f) = open_temp_queue();
        let id = q.enqueue("b1", "{}", "p1", "c1").unwrap();
        q.record_failure(id, "timeout").unwrap();
        let rows = q.pending_within_window().unwrap();
        assert_eq!(rows[0].retry_count, 1);
        assert_eq!(rows[0].last_error.as_deref(), Some("timeout"));
    }
}
