use actix::prelude::*;
use chrono::Local;
use cron::Schedule;
use std::{fs, str::FromStr, path::Path, time::Duration, sync::Arc};

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

/// Remove old logs past expiry in days
fn delete_old_logs(logs_folder: &Path, expiry: i32, show_logs: bool) {
    if show_logs {
        let exp = match expiry.clone() > 1 {
            true => format!("{} days starting today", expiry.clone()),
            false => format!("{} day starting today", expiry.clone()),
        };

        println!("Deleting logs which is > {}...", exp);
    }

    for entry in fs::read_dir(logs_folder).unwrap() {
        let entry = entry.unwrap();
        let filename = entry.file_name().to_str().unwrap_or("").to_string();
        if !filename.starts_with("logs.") {
            continue;
        }

        let date_str = &filename[5..];
        let date = chrono::NaiveDateTime::parse_from_str(&format!("{} 00:00:00", date_str), "%Y-%m-%d %H:%M:%S");
        if date.is_ok() {
            let now = Local::now().naive_local();
            let days_old = now.signed_duration_since(date.unwrap()).num_days();
            if days_old >= expiry as i64 {
                fs::remove_file(entry.path()).unwrap();
            }
        }
    }
}

/// Struct for scheduler
pub struct Scheduler {
    pub pool: Arc<DBPool>,
    pub show_logs: bool,
    pub duration: String,
    pub directory: String,
    pub expiry: i32,
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
    pub fn new<D1, D2>(pool: DBPool, func: fn(Arc<DBPool>), show_logs:bool, duration: D1, directory: D2, expiry: i32) -> Self
        where D1: Into<String>,
              D2: Into<String>
    {
        Scheduler{
            pool: Arc::new(pool),
            show_logs,
            duration: duration.into(),
            directory: directory.into(),
            expiry,
            func
        }
    }

    /// Execute scheduled task
    fn schedule_task(&self, ctx: &mut Context<Self>) {
        // Check if logs were available
        if self.show_logs {
            println!("{}", format!("Scheduled task for {:?} executed - {:?}", self.duration.clone(), Local::now()));
        }

        // Delete old logs
        let logs_folder = Path::new(&self.directory);
        delete_old_logs(logs_folder, self.expiry, self.show_logs);

        (self.func)(self.pool.clone());

        // Re-run cron
        ctx.run_later(duration_timer(&self.duration), move |this, ctx| {
            this.schedule_task(ctx)
        });
    }
}