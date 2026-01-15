use nanonis_rs::scan::{AutosaveMode, ScanPropsBuilder};
use nanonis_rs::NanonisClient;

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let mut client = NanonisClient::new("127.0.0.1", 6501)?;

    println!("=== Scan Properties Test ===\n");

    // Step 1: Read current properties
    println!("1. Reading current scan properties...");
    let props_before = client.scan_props_get()?;
    println!("   Continuous scan: {}", props_before.continuous_scan);
    println!("   Bouncy scan: {}", props_before.bouncy_scan);
    println!("   Autosave: {:?}", props_before.autosave);
    println!("   Series name: {:?}", props_before.series_name);
    println!("   Comment: {:?}", props_before.comment);
    println!("   Modules: {:?}", props_before.modules_names);
    println!("   Autopaste: {:?}", props_before.autopaste);
    println!();

    // Step 2: Modify properties using builder
    println!("2. Setting new scan properties...");
    let new_props = ScanPropsBuilder::new()
        .continuous_scan(true)         // Enable continuous scan
        .bouncy_scan(true)             // Enable bouncy scan
        .autosave(AutosaveMode::Off);  // Disable autosave

    client.scan_props_set(new_props)?;
    println!("   Properties set successfully");
    println!();

    // Step 3: Read properties again to verify
    println!("3. Reading properties again to verify changes...");
    let props_after = client.scan_props_get()?;
    println!("   Continuous scan: {}", props_after.continuous_scan);
    println!("   Bouncy scan: {}", props_after.bouncy_scan);
    println!("   Autosave: {:?}", props_after.autosave);
    println!("   Series name: {:?}", props_after.series_name);
    println!("   Comment: {:?}", props_after.comment);
    println!("   Modules: {:?}", props_after.modules_names);
    println!("   Autopaste: {:?}", props_after.autopaste);
    println!();

    // Step 4: Verify changes
    println!("4. Verifying changes...");
    let mut success = true;

    if !props_after.continuous_scan {
        println!("   ✗ Continuous scan not set correctly");
        success = false;
    } else {
        println!("   ✓ Continuous scan set to On");
    }

    if !props_after.bouncy_scan {
        println!("   ✗ Bouncy scan not set correctly");
        success = false;
    } else {
        println!("   ✓ Bouncy scan set to On");
    }

    if props_after.autosave != AutosaveMode::Off {
        println!("   ✗ Autosave not set correctly");
        success = false;
    } else {
        println!("   ✓ Autosave set to Off");
    }

    println!();
    if success {
        println!("✓ All properties changed successfully!");
    } else {
        println!("✗ Some properties did not change as expected");
    }

    // Step 5: Restore original properties
    println!("\n5. Restoring original properties...");
    let restore = ScanPropsBuilder::new()
        .continuous_scan(props_before.continuous_scan)
        .bouncy_scan(props_before.bouncy_scan)
        .autosave(props_before.autosave);
    client.scan_props_set(restore)?;
    println!("   Properties restored");

    Ok(())
}
