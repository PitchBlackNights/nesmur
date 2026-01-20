mod filter;

use self::filter::{FilterType, LogFilter, module, thread};
use colored::*;
use log::{LevelFilter, Record};
use std::{
    collections::HashMap,
    io::Write,
    sync::LazyLock,
    thread::{self, Thread},
};

static LOG_FILTERS: LazyLock<Vec<Vec<LogFilter>>> = LazyLock::new(|| {
    vec![
        // verbose = 0
        vec![],
        // verbose = 1
        vec![
            module!("eframe::native::run", Info),
            module!("eframe::native::glow_integration", Info),
            module!("gilrs", Info),
            module!("egui_glow", Info),
        ],
        // verbose = 2
        vec![
            module!("eframe::native::run", Debug),
            module!("eframe::native::glow_integration", Info),
            module!("egui_glow", Info),
            module!("egui_extras::loaders::svg_loader", Debug),
            module!("egui_winit", Debug),
            module!("gilrs", Debug),
            thread!("eframe_persist", Debug),
        ],
        // verbose = 3
        vec![
            module!("eframe::native::run", Debug),
            module!("eframe::native::glow_integration", Debug),
            module!("egui_extras::loaders::svg_loader", Debug),
            module!("egui_winit", Debug),
            module!("gilrs", Debug),
        ],
        // verbose = 4
        vec![
            module!("eframe::native::run", Debug),
            module!("eframe::native::glow_integration", Debug),
            module!("gilrs::ff::server", Debug),
        ],
        // verbose = 5
        vec![],
    ]
});

/// Initializes the logger
pub fn init_logger(verbose_level: u8) {
    let log_filters: &Vec<LogFilter> =
        &LOG_FILTERS[(verbose_level as usize).min(LOG_FILTERS.len() - 1)];
    let module_filters: Vec<LogFilter> =
        LogFilter::collect_by_type(&log_filters, FilterType::Module);
    let target_filters: HashMap<&str, LevelFilter> =
        LogFilter::collect_by_type(&log_filters, FilterType::Target)
            .iter()
            .map(|filter: &LogFilter| (filter.filter(), filter.level()))
            .collect();
    let thread_filters: HashMap<&str, LevelFilter> =
        LogFilter::collect_by_type(&log_filters, FilterType::Thread)
            .iter()
            .map(|filter| (filter.filter(), filter.level()))
            .collect();

    // println!("LOG_FILTERS[{}] = {:#?};", (verbose_level as usize).min(LOG_FILTERS.len() - 1), log_filters);

    // Determine log level based on verbosity flag
    let log_level_filter: LevelFilter = match verbose_level {
        0 => LevelFilter::Info,
        1 => LevelFilter::Debug,
        _ => LevelFilter::Trace,
    };

    let mut builder: env_logger::Builder = env_logger::Builder::new();
    builder.format(
        move |buf: &mut env_logger::fmt::Formatter,
              record: &Record<'_>|
              -> Result<(), std::io::Error> {
            let current_thread: Thread = thread::current();
            let thread_name: &str = current_thread.name().unwrap_or("<unnamed>");

            // if `thread_name` is in `thread_filters`
            // AND `filter` is less than `level` (Error < Warn)
            // THEN discard log
            if let Some(filter) = thread_filters.get(thread_name)
                && *filter < record.level()
            {
                return Ok(());
            }

            let module_path: String = record.module_path().unwrap_or("UNKNOWN").to_string();
            let mut target: String = record.target().to_string();

            if target != module_path {
                // if `target` is in `target_filters`
                // AND `filter` is less than `level` (Error < Warn)
                // THEN discard log
                if let Some(filter) = target_filters.get(target.as_str())
                    && *filter < record.level()
                {
                    return Ok(());
                }

                if verbose_level >= 2 {
                    target = format!("{}/{}", target, module_path)
                }
            }

            let level: String = record.level().to_string();

            // Log output format
            let log_output: String = format!(
                "[{}] [{}/{}]: {}",
                thread_name,
                target,
                level,
                record.args()
            );

            // Apply severity color to the whole log line
            let colored_log: ColoredString = match record.level() {
                log::Level::Error => log_output.bright_red().bold(),
                log::Level::Warn => log_output.bright_yellow(),
                log::Level::Info => log_output.normal(),
                log::Level::Debug => log_output.bright_blue(),
                log::Level::Trace => log_output.bright_black(),
            };

            writeln!(buf, "{}", colored_log)
        },
    );

    if verbose_level == 0 {
        builder
            .filter(None, LevelFilter::Off)
            .filter(Some("nesmur"), log_level_filter)
            .filter(Some("nes"), log_level_filter);
    } else {
        builder.filter(None, log_level_filter);
        for filter in module_filters {
            builder.filter(Some(filter.filter()), filter.level());
        }
    }

    builder.init();
}
