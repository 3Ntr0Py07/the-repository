fn main() {
    let uefi_path = env!("UEFI_PATH");
    let bios_path = env!("BIOS_PATH");

    let uefi = true;

    println!(
        "Booting with {} at {}.",
        if uefi { "UEFI" } else { "BIOS" },
        if uefi { uefi_path } else { bios_path }
    );

    let mut cmd = std::process::Command::new("qemu-system-x86_64");
    if uefi {
        cmd.arg("-bios").arg(ovmf_prebuilt::ovmf_pure_efi());
        cmd.arg("-drive")
            .arg(format!("format=raw,file={uefi_path}"));
    } else {
        cmd.arg("-drive")
            .arg(format!("format=raw,file={bios_path}"));
    }
    cmd.arg("-accel").arg("whpx");
    let mut child = cmd.spawn().unwrap();
    child.wait().unwrap();
}