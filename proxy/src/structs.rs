use std::sync::Arc;
use std::sync::Mutex;
use std::sync::atomic::AtomicUsize;
use std::time::{Duration, Instant};

pub enum BackendState {
    Healthy,
    Ejected { until: Instant },
    HalfOpen,
    Draining { deadline: Instant },
}

pub struct Backend {
    pub addr: String,
    pub active: AtomicUsize,
    pub consecutive_failures: AtomicUsize,
    pub state: Mutex<BackendState>,
}

impl Backend {
    pub fn new(addr: String) -> Arc<Backend> {
        Arc::new(Backend {
            addr,
            active: AtomicUsize::new(0),
            consecutive_failures: AtomicUsize::new(0),
            state: Mutex::new(BackendState::Healthy),
        })
    }

    pub fn is_available(&self) -> bool {
        match *self.state.lock().unwrap() {
            BackendState::Healthy => true,
            BackendState::Ejected { until } => Instant::now() >= until,
            _ => false,
        }
    }

    pub fn mark_failed(&self) {
        *self.state.lock().unwrap() = BackendState::Ejected {
            until: Instant::now() + Duration::from_secs(5),
        };
    }
}

pub struct ProxyState {
    pub backends: Vec<Arc<Backend>>,
    pub next: AtomicUsize,
    pub connect_timeout: Duration,
    pub check_interval: Duration,
}

impl ProxyState {
    pub fn new(
        backends: Vec<Arc<Backend>>,
        connect_timeout: Duration,
        check_interval: Duration,
    ) -> Arc<ProxyState> {
        Arc::new(ProxyState {
            backends,
            next: AtomicUsize::new(0),
            connect_timeout,
            check_interval,
        })
    }
}

pub struct Config {
    pub bind: String,
    pub backends: Vec<String>,
    pub connect_timeout_secs: u64,
    pub check_interval_secs: u64,
}
