use battery::Manager;
use uom::si::time::second;
use uom::si::f32::Time;

fn main() -> Result<(), battery::Error> {
    let manager = Manager::new()?;

    for (_idx, maybe_battery) in manager.batteries()?.enumerate() {
        let battery = maybe_battery?;
        println!("========================================");
        println!("Battery: {:.2}%", battery.state_of_charge().get::<battery::units::ratio::percent>());
        println!("{:?}", battery.state());
        if let Some(time_to_full) = battery.time_to_full() {
            println!("Time until fully charged: {}", format_duration(time_to_full));
        } else {
            println!("Time until fully charged: N/A");
        }
        println!("========================================");
        println!("");
    }

    Ok(())
}

fn format_duration(duration: Time) -> String {
    let total_seconds = duration.get::<second>();
    let total_minutes = total_seconds / 60.0;
    let hours = (total_minutes / 60.0).floor();
    let minutes = (total_minutes % 60.0).floor();
    format!("{:02}:{:02} hours", hours as i64, minutes as i64)
}