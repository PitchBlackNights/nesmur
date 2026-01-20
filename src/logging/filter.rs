use log::LevelFilter;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FilterType {
    Module,
    Target,
    Thread,
}

#[derive(Debug, Clone, Copy)]
pub struct LogFilter {
    filter_type: FilterType,
    filter: &'static str,
    level: LevelFilter,
}

impl LogFilter {
    pub fn new(filter_type: FilterType, filter: &'static str, level: LevelFilter) -> Self {
        LogFilter {
            filter_type,
            filter,
            level,
        }
    }

    pub fn filter(&self) -> &'static str {
        self.filter
    }

    pub fn level(&self) -> LevelFilter {
        self.level
    }

    pub fn collect_by_type(arr: &[LogFilter], filter_type: FilterType) -> Vec<LogFilter> {
        let mut new_arr: Vec<LogFilter> = Vec::with_capacity(arr.len());
        for filter in arr {
            if filter.filter_type == filter_type {
                new_arr.push(*filter);
            }
        }
        new_arr.shrink_to_fit();
        new_arr
    }
}

macro_rules! module {
    ($filter:literal, $level:ident) => {
        $crate::logging::filter::LogFilter::new(
            $crate::logging::filter::FilterType::Module,
            $filter,
            ::log::LevelFilter::$level,
        )
    };
}

#[allow(unused_macros)]
macro_rules! target {
    ($filter:literal, $level:ident) => {
        $crate::logging::filter::LogFilter::new(
            $crate::logging::filter::FilterType::Target,
            $filter,
            ::log::LevelFilter::$level,
        )
    };
}

macro_rules! thread {
    ($filter:literal, $level:ident) => {
        $crate::logging::filter::LogFilter::new(
            $crate::logging::filter::FilterType::Thread,
            $filter,
            ::log::LevelFilter::$level,
        )
    };
}

pub(super) use module;
#[allow(unused_imports)]
pub(super) use target;
pub(super) use thread;
