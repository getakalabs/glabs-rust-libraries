use actix::prelude::*;
use chrono::Local;
use cron::Schedule;
use std::{str::FromStr, time::Duration, sync::Arc};

use crate::DBPool;

/// Duration timer
fn duration_timer<T: Into<String>>(duration: T) -> Duration {
    let bindings = duration.into();
    let cron_schedule = Schedule::from_str(&bindings).unwrap();
    let now = Local::now();
    let next = cron_schedule.upcoming(Local).next().unwrap();
    let duration_until = next.signed_duration_since(now);

    duration_until.to_std().unwrap()
}

/// Struct for scheduler
pub struct Scheduler {
    pub pool: Arc<DBPool>,
    pub show_logs: bool,
    pub duration: String,
    pub func: fn(Arc<DBPool>)
}

/// Provide Actor implementation for our actor
impl Actor for Scheduler {
    type Context = Context<Self>;

    /// Executes start of scheduled task
    fn started(&mut self, ctx: &mut Context<Self>) {
        if self.show_logs {
            println!("{}", format!("Scheduler for {:?} is now running...", self.duration.clone()));
        }

        ctx.run_later(duration_timer(&self.duration), move |this, ctx| {
            this.schedule_task(ctx)
        });
    }

    /// Stop running task
    fn stopped(&mut self, _ctx: &mut Context<Self>) {
        if self.show_logs {
            println!("{}", format!("Scheduler for {:?} stopped...", self.duration.clone()));
        }
    }
}

/// Scheduler implementation
impl Scheduler {
    /// Initialize scheduler
    pub fn new<T: Into<String>>(pool: DBPool, show_logs:bool, duration: T, func: fn(Arc<DBPool>)) -> Self {
        Scheduler{
            pool: Arc::new(pool),
            show_logs,
            duration: duration.into(),
            func
        }
    }

    /// Execute scheduled task
    fn schedule_task(&self, ctx: &mut Context<Self>) {
        if self.show_logs {
            println!("{}", format!("Scheduled task for {:?} executed - {:?}", self.duration.clone(), Local::now()));
        }

        (self.func)(self.pool.clone());

        // Re-run cron
        ctx.run_later(duration_timer(&self.duration), move |this, ctx| {
            this.schedule_task(ctx)
        });
    }
}

