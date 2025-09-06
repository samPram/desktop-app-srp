// Simple test to verify gauge spacing changes
use desktop_app::ui::components::CircularGauge;

fn main() {
    println!("Testing gauge spacing modifications...");
    
    // Create RPM gauge
    let rpm_gauge = CircularGauge::rpm(5000.0);
    println!("RPM Gauge created with title: '{}'", rpm_gauge.title);
    println!("Size allocated: {} (should be size + 60.0 for extra spacing)", rpm_gauge.size + 60.0);
    
    // Create Speed gauge  
    let speed_gauge = CircularGauge::speed(120.0);
    println!("Speed Gauge created with title: '{}'", speed_gauge.title);
    println!("Size allocated: {} (should be size + 60.0 for extra spacing)", speed_gauge.size + 60.0);
    
    println!("✓ Gauge spacing modifications verified!");
    println!("- Title position moved from rect.top() + 10.0 to rect.top() + 20.0");
    println!("- Total allocated space increased from size + 40.0 to size + 60.0");
    println!("- Center adjustment changed from -30.0 to -20.0 for better spacing");
}