//! SigmaTimer - Timer and Alarm System
//!
//! Provides comprehensive timer and alarm management inspired by Linux timer tools.
//! Supports countdown timers, alarms, and scheduling.


extern crate alloc;

use alloc::{
    collections::BTreeMap,
    string::{String, ToString},
    vec::Vec,
};

/// Timer state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimerState {
    Idle,
    Running,
    Paused,
    Completed,
    Cancelled,
}

/// Timer
#[derive(Debug, Clone)]
pub struct Timer {
    pub id: String,
    pub name: String,
    pub duration_seconds: u64,
    pub remaining_seconds: u64,
    pub state: TimerState,
}

impl Timer {
    pub fn new(id: String, name: String, duration_seconds: u64) -> Self {
        Self {
            id,
            name,
            duration_seconds,
            remaining_seconds: duration_seconds,
            state: TimerState::Idle,
        }
    }

    pub fn start(&mut self) {
        self.state = TimerState::Running;
    }

    pub fn pause(&mut self) {
        self.state = TimerState::Paused;
    }

    pub fn resume(&mut self) {
        self.state = TimerState::Running;
    }

    pub fn complete(&mut self) {
        self.state = TimerState::Completed;
        self.remaining_seconds = 0;
    }

    pub fn cancel(&mut self) {
        self.state = TimerState::Cancelled;
    }

    pub fn tick(&mut self, seconds: u64) {
        if self.state == TimerState::Running {
            self.remaining_seconds = self.remaining_seconds.saturating_sub(seconds);
            if self.remaining_seconds == 0 {
                self.complete();
            }
        }
    }
}

/// Alarm
#[derive(Debug, Clone)]
pub struct Alarm {
    pub id: String,
    pub name: String,
    pub time_seconds: u64,
    pub enabled: bool,
    pub triggered: bool,
}

impl Alarm {
    pub fn new(id: String, name: String, time_seconds: u64) -> Self {
        Self {
            id,
            name,
            time_seconds,
            enabled: true,
            triggered: false,
        }
    }

    pub fn enable(&mut self) {
        self.enabled = true;
    }

    pub fn disable(&mut self) {
        self.enabled = false;
    }

    pub fn trigger(&mut self) {
        self.triggered = true;
    }

    pub fn reset(&mut self) {
        self.triggered = false;
    }
}

/// Timer manager
pub struct SigmaTimer {
    timers: BTreeMap<String, Timer>,
    alarms: BTreeMap<String, Alarm>,
    next_timer_id: u64,
    next_alarm_id: u64,
}

impl SigmaTimer {
    pub fn new() -> Self {
        Self {
            timers: BTreeMap::new(),
            alarms: BTreeMap::new(),
            next_timer_id: 1,
            next_alarm_id: 1,
        }
    }

    pub fn add_timer(&mut self, name: String, duration_seconds: u64) -> String {
        let id = format!("timer-{}", self.next_timer_id);
        self.next_timer_id += 1;
        let timer = Timer::new(id.clone(), name, duration_seconds);
        self.timers.insert(id.clone(), timer);
        id
    }

    pub fn get_timer(&self, id: &str) -> Option<&Timer> {
        self.timers.get(id)
    }

    pub fn get_timer_mut(&mut self, id: &str) -> Option<&mut Timer> {
        self.timers.get_mut(id)
    }

    pub fn list_timers(&self) -> Vec<&Timer> {
        self.timers.values().collect()
    }

    pub fn list_running_timers(&self) -> Vec<&Timer> {
        self.timers
            .values()
            .filter(|t| t.state == TimerState::Running)
            .collect()
    }

    pub fn remove_timer(&mut self, id: &str) -> bool {
        self.timers.remove(id).is_some()
    }

    pub fn add_alarm(&mut self, name: String, time_seconds: u64) -> String {
        let id = format!("alarm-{}", self.next_alarm_id);
        self.next_alarm_id += 1;
        let alarm = Alarm::new(id.clone(), name, time_seconds);
        self.alarms.insert(id.clone(), alarm);
        id
    }

    pub fn get_alarm(&self, id: &str) -> Option<&Alarm> {
        self.alarms.get(id)
    }

    pub fn get_alarm_mut(&mut self, id: &str) -> Option<&mut Alarm> {
        self.alarms.get_mut(id)
    }

    pub fn list_alarms(&self) -> Vec<&Alarm> {
        self.alarms.values().collect()
    }

    pub fn list_enabled_alarms(&self) -> Vec<&Alarm> {
        self.alarms.values().filter(|a| a.enabled).collect()
    }

    pub fn remove_alarm(&mut self, id: &str) -> bool {
        self.alarms.remove(id).is_some()
    }

    pub fn timer_count(&self) -> usize {
        self.timers.len()
    }

    pub fn alarm_count(&self) -> usize {
        self.alarms.len()
    }
}

impl Default for SigmaTimer {
    fn default() -> Self {
        Self::new()
    }
}

static SIGMATIMER_INSTANCE: std::sync::OnceLock<std::sync::Mutex<SigmaTimer>> =
    std::sync::OnceLock::new();

pub fn init_sigmatimer() {
    let _ = SIGMATIMER_INSTANCE.set(std::sync::Mutex::new(SigmaTimer::new()));
}

pub fn get_sigmatimer() -> std::sync::MutexGuard<'static, SigmaTimer> {
    SIGMATIMER_INSTANCE
        .get()
        .expect("SigmaTimer not initialized")
        .lock()
        .expect("SigmaTimer lock poisoned")
}

pub fn add_timer(name: String, duration_seconds: u64) -> String {
    get_sigmatimer().add_timer(name, duration_seconds)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_timer_creation() {
        let timer = Timer::new("timer-1".to_string(), "Pomodoro".to_string(), 1500);
        assert_eq!(timer.name, "Pomodoro");
        assert_eq!(timer.duration_seconds, 1500);
        assert_eq!(timer.remaining_seconds, 1500);
        assert_eq!(timer.state, TimerState::Idle);
    }

    #[test]
    fn test_timer_state() {
        let mut timer = Timer::new("timer-1".to_string(), "Timer".to_string(), 60);
        timer.start();
        assert_eq!(timer.state, TimerState::Running);

        timer.pause();
        assert_eq!(timer.state, TimerState::Paused);

        timer.resume();
        assert_eq!(timer.state, TimerState::Running);

        timer.complete();
        assert_eq!(timer.state, TimerState::Completed);
    }

    #[test]
    fn test_timer_tick() {
        let mut timer = Timer::new("timer-1".to_string(), "Timer".to_string(), 60);
        timer.start();
        timer.tick(10);
        assert_eq!(timer.remaining_seconds, 50);
    }

    #[test]
    fn test_alarm_creation() {
        let alarm = Alarm::new("alarm-1".to_string(), "Wake Up".to_string(), 28800);
        assert_eq!(alarm.name, "Wake Up");
        assert!(alarm.enabled);
        assert!(!alarm.triggered);
    }

    #[test]
    fn test_alarm_enable_disable() {
        let mut alarm = Alarm::new("alarm-1".to_string(), "Alarm".to_string(), 28800);
        alarm.disable();
        assert!(!alarm.enabled);

        alarm.enable();
        assert!(alarm.enabled);
    }

    #[test]
    fn test_timer_manager() {
        let manager = SigmaTimer::new();
        assert_eq!(manager.timer_count(), 0);
        assert_eq!(manager.alarm_count(), 0);
    }

    #[test]
    fn test_add_timer() {
        let mut manager = SigmaTimer::new();
        let id = manager.add_timer("Pomodoro".to_string(), 1500);
        assert_eq!(id, "timer-1");
        assert_eq!(manager.timer_count(), 1);
    }

    #[test]
    fn test_add_alarm() {
        let mut manager = SigmaTimer::new();
        let id = manager.add_alarm("Wake Up".to_string(), 28800);
        assert_eq!(id, "alarm-1");
        assert_eq!(manager.alarm_count(), 1);
    }

    #[test]
    fn test_list_running_timers() {
        let mut manager = SigmaTimer::new();
        let id1 = manager.add_timer("Timer1".to_string(), 60);
        let id2 = manager.add_timer("Timer2".to_string(), 120);

        manager.get_timer_mut(&id1).unwrap().start();
        manager.get_timer_mut(&id2).unwrap().start();

        let running = manager.list_running_timers();
        assert_eq!(running.len(), 2);
    }

    #[test]
    fn test_list_enabled_alarms() {
        let mut manager = SigmaTimer::new();
        let id1 = manager.add_alarm("Alarm1".to_string(), 28800);
        let id2 = manager.add_alarm("Alarm2".to_string(), 36000);

        manager.get_alarm_mut(&id2).unwrap().disable();

        let enabled = manager.list_enabled_alarms();
        assert_eq!(enabled.len(), 1);
    }

    #[test]
    fn test_global_instance() {
        init_sigmatimer();
        let before = get_sigmatimer().timer_count();

        {
            let mut manager_mut = get_sigmatimer();
            manager_mut.add_timer("Timer".to_string(), 60);
        }

        assert_eq!(get_sigmatimer().timer_count(), before + 1);
    }

    #[test]
    fn test_add_timer_convenience() {
        init_sigmatimer();
        let before = get_sigmatimer().timer_count();
        let id = add_timer("Timer".to_string(), 60);
        assert_eq!(id, format!("timer-{}", before + 1));

        let manager = get_sigmatimer();
        assert!(manager.get_timer(&id).is_some());
    }
}
