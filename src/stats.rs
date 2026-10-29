//! Completed PDF jobs. Totals stay in `RESUMA_DATA_DIR/activity.json`.
//! `GET /api/stats` returns them only with the admin cookie.

use std::path::PathBuf;
use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};

static STATE: std::sync::LazyLock<Mutex<Activity>> =
    std::sync::LazyLock::new(|| Mutex::new(Activity::load_file()));

#[derive(Clone, Copy)]
pub enum Kind {
    Compress,
    Merge,
    Split,
    FromJpg,
    ToJpg,
    ToText,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Snapshot {
    pub compress: u64,
    pub merge: u64,
    pub split: u64,
    pub from_jpg: u64,
    pub to_jpg: u64,
    pub to_text: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Activity {
    compress: u64,
    merge: u64,
    split: u64,
    from_jpg: u64,
    to_jpg: u64,
    to_text: u64,
}

impl Default for Activity {
    fn default() -> Self {
        Self {
            compress: 0,
            merge: 0,
            split: 0,
            from_jpg: 0,
            to_jpg: 0,
            to_text: 0,
        }
    }
}

impl Activity {
    fn load_file() -> Self {
        let path = store_path();
        let Ok(raw) = std::fs::read_to_string(&path) else {
            return Self::default();
        };
        serde_json::from_str(&raw).unwrap_or_default()
    }

    fn snapshot(&self) -> Snapshot {
        Snapshot {
            compress: self.compress,
            merge: self.merge,
            split: self.split,
            from_jpg: self.from_jpg,
            to_jpg: self.to_jpg,
            to_text: self.to_text,
        }
    }

    fn add(&mut self, kind: Kind) {
        match kind {
            Kind::Compress => self.compress = self.compress.saturating_add(1),
            Kind::Merge => self.merge = self.merge.saturating_add(1),
            Kind::Split => self.split = self.split.saturating_add(1),
            Kind::FromJpg => self.from_jpg = self.from_jpg.saturating_add(1),
            Kind::ToJpg => self.to_jpg = self.to_jpg.saturating_add(1),
            Kind::ToText => self.to_text = self.to_text.saturating_add(1),
        }
        let path = store_path();
        if let Some(dir) = path.parent() {
            let _ = std::fs::create_dir_all(dir);
        }
        let body = serde_json::json!({
            "compress": self.compress,
            "merge": self.merge,
            "split": self.split,
            "from_jpg": self.from_jpg,
            "to_jpg": self.to_jpg,
            "to_text": self.to_text,
            "updated_at": now_secs(),
        });
        let tmp = path.with_extension("json.tmp");
        if std::fs::write(&tmp, body.to_string()).is_ok() {
            let _ = std::fs::rename(&tmp, &path);
        }
    }
}

fn lock() -> std::sync::MutexGuard<'static, Activity> {
    STATE.lock().unwrap_or_else(|err| err.into_inner())
}

fn store_path() -> PathBuf {
    PathBuf::from(std::env::var("RESUMA_DATA_DIR").unwrap_or_else(|_| ".resuma".into()))
        .join("activity.json")
}

fn now_secs() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

pub fn record(kind: Kind) {
    lock().add(kind);
}

pub fn snapshot() -> Snapshot {
    lock().snapshot()
}

pub fn grouped(n: u64) -> String {
    let raw = n.to_string();
    let mut out = String::with_capacity(raw.len() + raw.len() / 3);
    for (i, ch) in raw.chars().enumerate() {
        if i > 0 && (raw.len() - i) % 3 == 0 {
            out.push(',');
        }
        out.push(ch);
    }
    out
}
