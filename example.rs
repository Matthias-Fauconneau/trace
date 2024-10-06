#![no_std]#![no_main]origin_studio::no_problem!();
fn main() {
	#[cfg(feature="std")] std::panic::set_hook(Box::new(|panic| println!("{}:{}: {}", panic.location().unwrap().file(), panic.location().unwrap().line(), panic.message())));
	trace::timeout(1, || println!("{:?}", rustix::thread::nanosleep(&rustix::thread::Timespec{tv_sec: 1, tv_nsec: 0}))) 
}
