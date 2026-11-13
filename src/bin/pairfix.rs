//! One-off repair tool: re-pairs the connected iPhone with this PC and stores a
//! fresh pairing record in Apple's lockdown directory.
//!
//! Usage:
//!   pairfix            -> pair the device (trust dialog) and save the record
//!   pairfix --verify   -> only verify that the stored record starts a session
//!
//! The iPhone must be plugged in, unlocked, and the "Trust This Computer"
//! dialog must be accepted while the tool is waiting. The tool keeps retrying
//! while the phone is locked, so unlock it at any time during the wait.

use std::time::{Duration, Instant};

use idevice::{
    IdeviceError, IdeviceService,
    lockdown::LockdownClient,
    usbmuxd::{UsbmuxdAddr, UsbmuxdConnection},
};

const WAIT: Duration = Duration::from_secs(240);
const LOCKDOWN_DIR: &str = r"C:\ProgramData\Apple\Lockdown";

#[tokio::main]
async fn main() {
    if let Err(e) = run().await {
        eprintln!("\nFAILED: {e}");
        std::process::exit(1);
    }
}

async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let verify_only = std::env::args().any(|a| a == "--verify");

    let mut usbmux = UsbmuxdConnection::default().await?;
    let devices = usbmux.get_devices().await?;
    if devices.is_empty() {
        return Err("No device found. Plug in the iPhone, unlock it and try again.".into());
    }

    for d in &devices {
        println!("found device: udid={} id={}", d.udid, d.device_id);
    }
    let device = devices[0].clone();

    let addr = UsbmuxdAddr::from_env_var()?;
    let provider = device.to_provider(addr, "pairfix");
    let record_path = format!("{LOCKDOWN_DIR}\\{}.plist", device.udid);

    if verify_only {
        println!("verifying stored record {record_path} ...");
        return verify(&mut usbmux, &provider, &device.udid).await;
    }

    println!("connecting to lockdownd ...");
    let mut lockdown = LockdownClient::connect(&provider).await?;

    let buid = usbmux.get_buid().await?;
    let host_id = uuid::Uuid::new_v4().to_string().to_uppercase();
    println!("SystemBUID = {buid}");
    println!("new HostID = {host_id}");
    println!(">>> Unlock the iPhone and tap \"Trust / Tin cậy\" within {WAIT:?} ...");

    let deadline = Instant::now() + WAIT;
    let mut locked_warned = false;
    let record = loop {
        if Instant::now() >= deadline {
            return Err("timed out waiting for the Trust dialog".into());
        }
        match lockdown
            .pair_once(host_id.clone(), buid.clone(), Some("B1nLoader"))
            .await
        {
            Ok(r) => break r,
            Err(IdeviceError::DeviceLocked) => {
                if !locked_warned {
                    println!("phone is locked -> please unlock it and keep the screen on");
                    locked_warned = true;
                }
            }
            Err(IdeviceError::PairingDialogResponsePending) => {
                locked_warned = false;
            }
            Err(e) => return Err(Box::new(e)),
        }
        tokio::time::sleep(Duration::from_millis(1500)).await;
    };

    let bytes = record.serialize()?;
    std::fs::write(&record_path, &bytes)?;
    println!("\npair record written to {record_path} ({} bytes)", bytes.len());

    // usbmuxd's own SavePairRecord is unreliable on Windows; try it but ignore.
    if let Err(e) = usbmux.save_pair_record(&device.udid, bytes).await {
        println!("(usbmuxd SavePairRecord failed, ignored: {e})");
    }

    verify(&mut usbmux, &provider, &device.udid).await
}

async fn verify(
    usbmux: &mut UsbmuxdConnection,
    provider: &idevice::provider::UsbmuxdProvider,
    udid: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let stored = usbmux.get_pair_record(udid).await?;
    let mut lockdown = LockdownClient::connect(provider).await?;
    lockdown.start_session(&stored).await?;
    let version = lockdown.get_value(Some("ProductVersion"), None).await?;
    let name = lockdown.get_value(Some("DeviceName"), None).await?;
    println!(
        "SUCCESS: lockdown session OK -> {} (iOS {})",
        name.as_string().unwrap_or("?"),
        version.as_string().unwrap_or("?")
    );
    Ok(())
}
