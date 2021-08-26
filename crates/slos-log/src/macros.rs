#[macro_export(local_inner_macros)]
macro_rules! log {
	($lvl:expr, $fmt:expr) => ($crate::logcrate::log!(
		target: __log_target!(),
		$lvl,
		core::concat!("{}: ", $fmt),
		slos_helpers::function!()
	));

	($lvl:expr, $fmt:expr, $($arg:tt)+) => ($crate::logcrate::log!(
		target: __log_target!(),
		$lvl,
		core::concat!("{}: ", $fmt),
		slos_helpers::function!(),
		$($arg)+
	))
}

#[macro_export(local_inner_macros)]
macro_rules! error {
	($($arg:tt)+) => (log!($crate::logcrate::Level::Error, $($arg)+))
}

#[macro_export(local_inner_macros)]
macro_rules! warn {
	($($arg:tt)+) => (log!($crate::logcrate::Level::Warn, $($arg)+))
}

#[macro_export(local_inner_macros)]
macro_rules! info {
	($($arg:tt)+) => (log!($crate::logcrate::Level::Info, $($arg)+))
}

#[macro_export(local_inner_macros)]
macro_rules! debug {
	($($arg:tt)+) => (log!($crate::logcrate::Level::Debug, $($arg)+))
}

#[macro_export(local_inner_macros)]
macro_rules! trace {
	($($arg:tt)+) => (log!($crate::logcrate::Level::Trace, $($arg)+))
}

#[doc(hidden)]
#[macro_export]
macro_rules! __log_target {
	() => {
		module_path!()
	}
}
