use std::sync::Arc;
use std::sync::Mutex;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;
use std::sync::atomic::Ordering::Relaxed;
use std::time::{Duration, Instant};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BackendState {
    Healthy,
    Ejected { until: Instant },
    HalfOpen,
    //Draining { deadline: Instant }, Later
}

pub struct Inner {
    pub timeout: Duration,
    pub state: BackendState,
}

pub struct Backend {
    pub inner: Mutex<Inner>,
    pub addr: String,
    pub active: AtomicUsize,
    pub consecutive_failures: AtomicUsize,
}

impl Backend {
    pub fn new(addr: String) -> Arc<Backend> {
        Arc::new(Backend {
            inner: Mutex::new(Inner {
                timeout: Duration::from_secs(5),
                state: BackendState::Healthy,
            }),
            addr,
            active: AtomicUsize::new(0),
            consecutive_failures: AtomicUsize::new(0),
        })
    }

    pub fn is_available(&self) -> bool {
        let inner = self.inner.lock().unwrap();
        match inner.state {
            BackendState::Healthy => true,
            BackendState::Ejected { until } => Instant::now() >= until,
            _ => false,
        }
    }

    pub fn try_to_get(&self) -> bool {
        let mut inner = self.inner.lock().unwrap();
        match inner.state {
            BackendState::Healthy => true,
            BackendState::Ejected { until } if Instant::now() >= until => {
                inner.state = BackendState::HalfOpen;
                true
            }

            // Return false ALSO for BackendState::HalfOpen so that only one connection goes through.
            _ => false,
        }
    }

    pub fn record_success(&self) {
        let mut inner = self.inner.lock().unwrap();
        self.consecutive_failures.store(0, Ordering::Relaxed);
        if matches!(inner.state, BackendState::HalfOpen) {
            inner.state = BackendState::Healthy;
            inner.timeout = (inner.timeout / 2).max(Duration::from_secs(5));
        }
    }

    pub fn record_failure(&self) -> Option<Duration> {
        let mut inner = self.inner.lock().unwrap();

        match inner.state {
            BackendState::Ejected { .. } => None,

            BackendState::HalfOpen => Some(self.eject(&mut inner)),

            BackendState::Healthy => {
                let fails = self.consecutive_failures.fetch_add(1, Relaxed) + 1;
                if fails >= 3 {
                    Some(self.eject(&mut inner))
                } else {
                    None
                }
            }
        }
    }

    fn eject(&self, inner: &mut Inner) -> Duration {
        let eject_for = inner.timeout;
        inner.state = BackendState::Ejected {
            until: Instant::now() + eject_for,
        };
        inner.timeout = (eject_for * 2).min(Duration::from_secs(60 * 5));
        self.consecutive_failures.store(0, Relaxed);
        eject_for
    }
}

pub struct ProxyState {
    pub backends: Vec<Arc<Backend>>,
    pub next: AtomicUsize,
    pub connect_timeout: Duration,
    //pub check_interval: Duration, Useless for now
}

impl ProxyState {
    pub fn new(
        backends: Vec<Arc<Backend>>,
        connect_timeout: Duration,
        //check_interval: Duration,
    ) -> Arc<ProxyState> {
        Arc::new(ProxyState {
            backends,
            next: AtomicUsize::new(0),
            connect_timeout,
            //check_interval,
        })
    }
}

pub struct Config {
    pub bind: String,
    pub backends: Vec<String>,
    pub connect_timeout_secs: u64,
    pub check_interval_secs: u64,
}
