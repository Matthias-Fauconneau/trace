#![feature(thread_spawn_unchecked, asm, once_cell)]

static RSTACK_SELF: std::lazy::SyncLazy<std::sync::atomic::AtomicBool> = std::lazy::SyncLazy::new(|| false.into());

use {fehler::throws, rstack_self::Error};
#[throws] pub fn rstack_self() {
	RSTACK_SELF.store(true, std::sync::atomic::Ordering::Relaxed);
	if std::env::args().nth(1).unwrap_or_default() == "rstack-self" {
		rstack_self::child()?;
		std::process::exit(0);
	}
}

pub fn trace(_: *const std::ffi::c_void) {
	assert!(RSTACK_SELF.load(std::sync::atomic::Ordering::Relaxed));
	for thread in rstack_self::trace(std::process::Command::new(std::env::current_exe().unwrap()).arg("rstack-self")).unwrap().threads().first() {
		for frame in thread.frames().iter().rev() {
			for symbol in frame.symbols().iter().rev() {
				if let (Some(line), Some(name)) = (symbol.line(), symbol.name()) { eprintln!("{}:{}", name, line) }
				//if let Some(hash) = name.rfind("::") { name = name.split_at(hash).0; }
			}
		}
	}
	/*struct Symbol<'t> {line: u32, name: &'t str}
	let mut symbols = thread.frames().iter().rev().flat_map(|frame|
		frame.symbols().iter().rev().filter_map(|sym|
			sym.line().map(|line| sym.name().map(|mut name| {
				if let Some(hash) = name.rfind("::") { name = name.split_at(hash).0; }
				Symbol{line,name}
			})).flatten().or(||
		)
	);
	for Symbol{line,name,..} in &mut symbols { if name.ends_with("::main") { eprintln!("{}:{}", name, line); break; } }
	for Symbol{line,name,..} in symbols { eprintln!("{}:{}", name, line); }*/
}

use std::{ptr::null, thread::spawn, process::abort};
use signal_hook::{iterator::*, consts::signal::*};
#[throws] pub fn signal_floating_point_exception() {
	rstack_self()?;
	spawn(|| for _ in Signals::new(&[SIGFPE]).unwrap().forever() { eprintln!("Floating point exception"); trace(null()); abort() });
}
#[throws] pub fn signal_interrupt() {
	rstack_self()?;
	spawn(|| for _ in Signals::new(&[SIGINT]).unwrap().forever() { trace(null()); abort() });
}
#[throws] pub fn signal_illegal() {
	rstack_self()?;
	spawn(|| for info in SignalsInfo::<exfiltrator::raw::WithRawSiginfo>::new(&[SIGILL]).unwrap().forever() { trace(unsafe{info.si_addr()}); abort() });
}

#[fehler::throws(std::io::Error)] pub fn timeout_<T>(time: /*std::time::Duration*/u64, task: impl FnOnce()->T, display: impl std::fmt::Display + std::marker::Sync) -> T {
	let time = std::time::Duration::from_millis(time);
	if time.is_zero() { task() } else {
		let done = std::sync::atomic::AtomicBool::new(false);
		let watchdog = || {
			let start = std::time::Instant::now();
			let mut remaining = time;
			while !done.load(std::sync::atomic::Ordering::Acquire) {
				std::thread::park_timeout(remaining);
				let elapsed = start.elapsed();
				if elapsed >= time {
					trace(null()); //#[cfg(feature="trace")] crate::trace::trace();
					eprintln!("{}", display);
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
#[track_caller] pub fn timeout<T>(time: u64, task: impl FnOnce()->T) -> T { timeout_(time, task, std::panic::Location::caller()).unwrap() }

#[allow(non_snake_case)] pub fn unmask_SSE_exceptions() {
	unsafe {
		asm!(
			"sub rsp, 4",
			"stmxcsr [rsp]",
			"and dword ptr [rsp], {csr}",
			"ldmxcsr [rsp]",
			"add rsp, 4",
			csr = const 0b11111111_11111111_11111101_01111111u32, // only invalid, divide-by-zero
		);
	 }
}
