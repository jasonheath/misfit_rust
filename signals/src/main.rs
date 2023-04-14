use std::ffi::c_int;

use nix::{
    libc::sigset_t,
    sys::signal::{SigSet, Signal},
};

fn add_nix_signal(signal: Signal, empty: &mut SigSet, all: &mut SigSet) {
    println!("Adjusting SigSets with {:?}", signal);
    empty.add(signal);
    all.remove(signal);
    println!("empty -> {:?}", empty);
    println!("  all -> {:?}", all);
}

fn nix_signaling() {
    println!("\n-- nix style signal handling");
    let mut empty = SigSet::empty();
    let mut all = SigSet::all();
    println!("empty -> {:?}", empty);
    println!("  all -> {:?}", all);
    add_nix_signal(nix::sys::signal::Signal::SIGINT, &mut empty, &mut all);
    add_nix_signal(nix::sys::signal::Signal::SIGTERM, &mut empty, &mut all);
    add_nix_signal(nix::sys::signal::Signal::SIGHUP, &mut empty, &mut all);
    add_nix_signal(nix::sys::signal::Signal::SIGCHLD, &mut empty, &mut all);
    add_nix_signal(nix::sys::signal::Signal::SIGQUIT, &mut empty, &mut all);
    add_nix_signal(nix::sys::signal::Signal::SIGALRM, &mut empty, &mut all);
    add_nix_signal(nix::sys::signal::Signal::SIGUSR1, &mut empty, &mut all);
    add_nix_signal(nix::sys::signal::Signal::SIGUSR2, &mut empty, &mut all);
}

fn add_libc_signal(signal: c_int, empty: &mut sigset_t, all: &mut sigset_t) {
    println!("Adjusting SigSets with {:?}", signal);
    unsafe {
        nix::libc::sigaddset(empty, signal);
        nix::libc::sigdelset(all, signal);
    }
    println!("empty -> {:?}", empty);
    println!("  all -> {:?}", all);
}

fn libc_signaling() {
    unsafe {
        println!("\n-- libc style signal handling");
        let mut empty: nix::libc::sigset_t = std::mem::zeroed();
        let mut all: nix::libc::sigset_t = std::mem::zeroed();
        println!("empty -> {:?} (before libc calls)", empty);
        println!("  all -> {:?} (before libc calls)", all);
        nix::libc::sigemptyset(&mut empty);
        nix::libc::sigfillset(&mut all);
        println!("empty -> {:?} (after libc calls)", empty);
        println!("  all -> {:?} (after libc calls)", all);
        add_libc_signal(nix::libc::SIGINT, &mut empty, &mut all);
        add_libc_signal(nix::libc::SIGTERM, &mut empty, &mut all);
        add_libc_signal(nix::libc::SIGHUP, &mut empty, &mut all);
        add_libc_signal(nix::libc::SIGCHLD, &mut empty, &mut all);
        add_libc_signal(nix::libc::SIGQUIT, &mut empty, &mut all);
        add_libc_signal(nix::libc::SIGALRM, &mut empty, &mut all);
        add_libc_signal(nix::libc::SIGUSR1, &mut empty, &mut all);
        add_libc_signal(nix::libc::SIGUSR2, &mut empty, &mut all);
    }
}

fn main() {
    nix_signaling();
    libc_signaling();
}
