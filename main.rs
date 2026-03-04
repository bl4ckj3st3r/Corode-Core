#![no_std]
#![no_main]

use core::arch::asm;
use core::panic::PanicInfo;

// Der Harlekin-Drop (Uralte Weisheit in ANSI)
#[link_section = ".vault"]
const HARLEKIN_SAGT_NEIN: &[u8] = b"\x1B[2J\x1B[H\n\r\
    *** HARLEKIN SAGT NEIN ***\n\r\
    Zugriffsverletzung erkannt.\n\r\
    Der Vault ist versiegelt.\n\r";

#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {
    // 1. Trap-Vektor setzen
    asm!("csrw mtvec, {}", in(reg) handle_trap as usize);

    // 2. PMP Schutzschild hochziehen
    activate_pmp_shield();

    // 3. Den "Self-Attack" provozieren
    let vault_ptr = 0x80001000 as *mut u32;
    core::ptr::write_volatile(vault_ptr, 0xDEADBEEF);

    loop {}
}

unsafe fn activate_pmp_shield() {
    extern "C" {
        static __vault_start: usize;
        static __vault_end: usize;
    }

    let end = &__vault_end as *const _ as usize;

    // PMP Konfiguration: TOR, Read-Only, Locked
    asm!("csrw pmpaddr0, {}", in(reg) end >> 2);
    asm!("csrw pmpcfg0, {}", in(reg) 0x89); // 0x89 = Locked
}

#[no_mangle]
pub unsafe extern "C" fn handle_trap() {
    // Hier schlägt der Harlekin zu
    for &b in HARLEKIN_SAGT_NEIN {
        core::ptr::write_volatile(0x10000000 as *mut u8, b);
    }

    loop { asm!("wfi"); }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! { loop {} }
Dateiname: main.rs