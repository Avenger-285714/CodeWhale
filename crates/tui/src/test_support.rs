//! Shared test-only helpers.

use std::sync::{Mutex, MutexGuard, OnceLock};

fn env_lock() -> &'static Mutex<()> {
    static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    LOCK.get_or_init(|| Mutex::new(()))
}

/// Acquire the process-wide env-var mutex.
///
/// If a prior test panicked while holding the lock, recover the guard instead
/// of cascading failures across unrelated tests.
pub(crate) fn lock_test_env() -> MutexGuard<'static, ()> {
    match env_lock().lock() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(),
    }
}

/// RAII guard that isolates `Settings::load()` from the developer's real
/// `~/.deepseek` / `~/.codewhale` config for the duration of a test.
///
/// Holds the process-wide env mutex, points `DEEPSEEK_CONFIG_PATH` at a fresh
/// empty temp config (with a sibling empty `settings.toml`), and restores the
/// previous value — dropping the tempdir — on drop. Without it, any test that
/// builds an `App` or calls `apply_document`/`change` reads the developer's
/// live `provider = "…"` setting, so model normalization (provider-prefixing)
/// and api-key detection diverge from the Deepseek-default CI environment and
/// the test fails on the maintainer's machine but passes on clean CI.
pub(crate) struct IsolatedConfigEnv {
    _guard: MutexGuard<'static, ()>,
    _tempdir: tempfile::TempDir,
    prev: Option<std::ffi::OsString>,
}

impl Drop for IsolatedConfigEnv {
    fn drop(&mut self) {
        // SAFETY: env mutation is serialised by the env mutex held in `_guard`.
        unsafe {
            match self.prev.take() {
                Some(value) => std::env::set_var("DEEPSEEK_CONFIG_PATH", value),
                None => std::env::remove_var("DEEPSEEK_CONFIG_PATH"),
            }
        }
    }
}

/// Acquire the env lock and isolate config loading. Keep the returned guard
/// alive for the whole test (`let _env = isolated_config_env();`).
pub(crate) fn isolated_config_env() -> IsolatedConfigEnv {
    let guard = lock_test_env();
    let tempdir = tempfile::tempdir().expect("isolated config tempdir");
    let config_path = tempdir.path().join("config.toml");
    std::fs::write(&config_path, "").expect("seed isolated config");
    std::fs::write(tempdir.path().join("settings.toml"), "").expect("seed isolated settings");
    let prev = std::env::var_os("DEEPSEEK_CONFIG_PATH");
    // SAFETY: env mutation is serialised by the env mutex held in `guard`.
    unsafe {
        std::env::set_var("DEEPSEEK_CONFIG_PATH", &config_path);
    }
    IsolatedConfigEnv {
        _guard: guard,
        _tempdir: tempdir,
        prev,
    }
}

/// Find the byte position of the first divergence between two strings,
/// returning a windowed view (`±32 bytes` around the divergence) so failures
/// in cache-prefix-stability tests show *which* bytes drifted, not just that
/// they did. Returns `None` when the strings are byte-identical.
pub(crate) fn first_divergence(a: &str, b: &str) -> Option<(usize, String, String)> {
    let a_bytes = a.as_bytes();
    let b_bytes = b.as_bytes();
    let max = a_bytes.len().min(b_bytes.len());
    for i in 0..max {
        if a_bytes[i] != b_bytes[i] {
            let lo = i.saturating_sub(32);
            let a_hi = (i + 32).min(a_bytes.len());
            let b_hi = (i + 32).min(b_bytes.len());
            let a_ctx = String::from_utf8_lossy(&a_bytes[lo..a_hi]).into_owned();
            let b_ctx = String::from_utf8_lossy(&b_bytes[lo..b_hi]).into_owned();
            return Some((i, a_ctx, b_ctx));
        }
    }
    if a_bytes.len() != b_bytes.len() {
        return Some((
            max,
            format!("(len={})", a_bytes.len()),
            format!("(len={})", b_bytes.len()),
        ));
    }
    None
}

/// Assert two strings are byte-identical, panicking with a windowed diff
/// around the first divergence when they aren't. Used by the prefix-cache
/// stability harness (#263, #280) to pin construction surfaces that land in
/// DeepSeek's KV cache prefix.
#[track_caller]
pub(crate) fn assert_byte_identical(label: &str, a: &str, b: &str) {
    if let Some((pos, a_ctx, b_ctx)) = first_divergence(a, b) {
        panic!(
            "{label}: prompt construction is non-deterministic — first diff at byte {pos}\n\
             ── side A (±32B) ──\n{a_ctx:?}\n── side B (±32B) ──\n{b_ctx:?}",
        );
    }
}
