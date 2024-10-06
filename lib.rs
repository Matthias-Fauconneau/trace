#![no_std] use origin_studio::println;
/*#![feature(thread_spawn_unchecked, asm, once_cell)]

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
*/

use linux_raw_sys::general::{ucontext_t, siginfo_t, SA_RESTART, SA_SIGINFO};

unsafe extern "C" fn handler(_sig: i32, _info: *const siginfo_t, _ctx: *const ucontext_t) {
	panic!("handler");
}

pub fn timeout<T>(_time: u64, task: impl FnOnce()->T) -> T { 
	use rustix::{thread::gettid, runtime::{/*sigaction,*/ Signal, Sigaction, Sigset, tkill}};
	//assert_eq!(std::mem::size_of::<Sigaction>(), 152);
	unsafe{origin::signal::sigaction(Signal::Prof, Some(Sigaction{sa_handler_kernel: Some(handler), sa_flags: (SA_SIGINFO|SA_RESTART) as u64, sa_restorer: None, sa_mask: Sigset{sig: [0; 1]}}))}.unwrap();
	//unsafe{libc::sigaction(Signal::Prof as i32, &Sigaction{sa_handler_kernel: Some(handler), sa_flags: (SA_SIGINFO|SA_RESTART) as u64, sa_restorer: None, sa_mask: Sigset{sig: [0; 16]}} as *const _ as *const _, core::ptr::null_mut())};
	/*let done = std::sync::atomic::AtomicBool::new(false);
	let watchdog = || {
		let time = std::time::Duration::from_millis(time);
		let start = std::time::Instant::now();
		let mut remaining = time;
		while !done.load(std::sync::atomic::Ordering::Acquire) {
			std::thread::park_timeout(remaining);
			let elapsed = start.elapsed();
			if elapsed >= time {
				unsafe{tkill(tid, Signal::Prof).unwrap()}; 
				//panic!("timeout!");
			} else {
				remaining = time - elapsed;
			}
		}
	};
	let watchdog = unsafe { std::thread::Builder::new().spawn_unchecked(watchdog).unwrap() };
	let result = task();
	done.store(true, std::sync::atomic::Ordering::Release);
	watchdog.thread().unpark();
	watchdog.join().unwrap();
	result*/
	let tid = gettid();
	unsafe{tkill(tid, Signal::Prof).unwrap()}; 
	println!("OK");
	task()
}
