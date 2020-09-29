#![feature(duration_zero,thread_spawn_unchecked)]
#[fehler::throws(rstack_self::Error)] pub fn rstack_self() {
	if std::env::args().nth(1).unwrap_or_default() == "rstack-self" {
		rstack_self::child()?;
		std::process::exit(0);
	}
}

pub fn trace() {
	for thread in rstack_self::trace(std::process::Command::new(std::env::current_exe().unwrap()).arg("rstack-self")).unwrap().threads().first() {
		struct Symbol<'t> {line: u32, name: &'t str};
		let mut symbols = thread.frames().iter().rev().flat_map(|frame|
			frame.symbols().iter().rev().filter_map(|sym|
				sym.line().map(|line| sym.name().map(|mut name| {
					if let Some(hash) = name.rfind("::") { name = name.split_at(hash).0; }
					Symbol{line,name}
				})).flatten()
			)
		);
		for Symbol{line,name,..} in &mut symbols { if name.ends_with("::main") { eprintln!("{}:{}", name, line); break; } }
		for Symbol{line,name,..} in symbols { eprintln!("{}:{}", name, line); }
	}
}

pub fn sigint() { std::thread::spawn(|| for _ in signal_hook::iterator::Signals::new(&[signal_hook::SIGINT]).unwrap().forever() { trace(); std::process::abort() }); }

#[fehler::throws(std::io::Error)] fn timeout_<T>(task: impl FnOnce()->T, time: std::time::Duration, display: impl std::fmt::Display + std::marker::Sync) -> T {
	if time.is_zero() { task() } else {
		let done = std::sync::atomic::AtomicBool::new(false);
		let watchdog = || {
			let start = std::time::Instant::now();
			let mut remaining = time;
			while !done.load(std::sync::atomic::Ordering::Acquire) {
				std::thread::park_timeout(remaining);
				let elapsed = start.elapsed();
				if elapsed >= time {
					eprintln!("{}", display);
					//trace(); //#[cfg(feature="trace")] crate::trace::trace();
					std::process::abort()
				}
				remaining = time - elapsed;
			}
		};
		let watchdog = unsafe { std::thread::Builder::new().spawn_unchecked(watchdog)? };
		let result = task();
		done.store(true, std::sync::atomic::Ordering::Release);
		watchdog.thread().unpark();
		watchdog.join().unwrap();
		result
	}
}
//pub fn timeout<T>(debug: impl std::fmt::Debug + std::marker::Sync, task: impl FnOnce()->T) -> T { timeout_(task, std::time::Duration::from_millis(1), debug).unwrap() }
//#[track_caller] pub fn timeout<T>(task: impl FnOnce()->T) -> T { timeout_(task, std::time::Duration::from_millis(1), std::panic::Location::caller()).unwrap() }
#[track_caller] pub fn timeout<T>(time: u64, task: impl FnOnce()->T) -> T { timeout_(task, std::time::Duration::from_millis(time), std::panic::Location::caller()).unwrap() }
