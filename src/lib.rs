#[macro_use]
extern crate ld_preload;

ld_preload_init!({
    println!("== PRELOAD - INIT");
    let p = procinfo::pid::statm_self();
    println!("== {:#?}\n==\n", p);
});

ld_preload_deinit!({
    let p = procinfo::pid::statm_self();
    println!("\n== {:#?}\n==", p);
    println!("== PRELOAD - FINI");
});
