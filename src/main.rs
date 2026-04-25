trait LedOutput {
    fn apply(&mut self, leds: &LedArray) -> Result<(), std::io::Error>;
}

struct ConsoleLedOutput;

impl LedOutput for ConsoleLedOutput {
    fn apply(&mut self, leds: &LedArray) -> Result<(), std::io::Error> {
        println!("{}", leds.describe());
        Ok(())
    }
}

struct AppliedLedRecord {
    target: LedTarget,
    color: LedColor,
    active: bool,
}

struct FakeLedOutput {
    last_snapshot: Vec<AppliedLedRecord>,
}

impl LedOutput for FakeLedOutput {
    fn apply(&mut self, leds: &LedArray) -> Result<(), std::io::Error> {
        let mut snapshot = Vec::new();

        for i in 0..leds.leds.len() {
            snapshot.push(AppliedLedRecord {
                target: leds.leds[i].target,
                color: leds.leds[i].color,
                active: leds.leds[i].active,
            });
        }

        self.last_snapshot = snapshot;
        Ok(())
    }
}


#[derive(PartialEq, Clone, Copy, Debug)]
enum LedTarget  {
    Network,
    Disk0,
    Disk1,
}

#[derive(PartialEq, Clone, Copy, Debug)]
enum LedColor {
    Cool,
    Hot,
}

struct LedState {
    target: LedTarget,
    color: LedColor,
    active: bool,
}

struct LedArray {
    leds: [LedState; 3],
}

impl LedArray {
    fn describe(&self) -> String {
        
        let mut s = String::new();

        s.push_str(format!("State of the LED array: {} LEDs \n", self.leds.len()).as_str());
        for i in 0..self.leds.len() {
            s.push_str(format!("{} \n",self.leds[i].describe()).as_str());
        }
        return s;
    }

    fn toggle_target(&mut self, target: LedTarget) {
        for i in 0..self.leds.len() {
            if self.leds[i].target == target {
                self.leds[i].toggle();
                break;
            }
        }
    }
    fn new() -> Self {
        LedArray {
            leds: [
                LedState { target: LedTarget::Network, color: LedColor::Cool, active: false },
                LedState { target: LedTarget::Disk0,   color: LedColor::Hot,  active: false },
                LedState { target: LedTarget::Disk1,   color: LedColor::Hot,  active: false },
            ],
        }
    }
}

struct LedService {
    leds: LedArray,
    tick_interval: u64,
}

impl LedService { 
    fn new() -> Self {
        LedService {
            leds: LedArray::new(),
            tick_interval: 3,
        }
    }

    fn start(self) {
        loop {
            std::thread::sleep(std::time::Duration::from_secs(self.tick_interval));
            println!("Tick...");
            println!("{}", self.leds.describe());
        }
    }
}

fn main() {
    let led_service = LedService::new();
    led_service.start();
}

impl LedState {
    fn describe(&self) -> String {
        let target_str = match self.target {
            LedTarget::Network => "Network",
            LedTarget::Disk0 => "Disk0",
            LedTarget::Disk1 => "Disk1",
        };

        let color_str = match self.color {
            LedColor::Cool => "Cool",
            LedColor::Hot => "Hot",
        };

        return format!("State of LED: target={}, color={}, active={}", target_str, color_str, self.active)
    }

    fn toggle(&mut self) {
        self.active = !self.active;
    }
}

#[test]
fn toggle_led() {
    let mut led = LedState { target: LedTarget::Network, color: LedColor::Cool, active: false };
    led.toggle();
    assert_eq!(led.active, true);
}

#[test]
fn toggle_target() {
    let mut led_array = LedArray {
        leds: [
            LedState { target: LedTarget::Network, color: LedColor::Cool, active: false },
            LedState { target: LedTarget::Disk0,   color: LedColor::Hot,  active: false },
            LedState { target: LedTarget::Disk1,   color: LedColor::Hot,  active: false },
        ],
    };

    assert!(!led_array.leds.iter().any(|led| led.target == LedTarget::Network && led.active));
    led_array.toggle_target(LedTarget::Network);
    assert!(led_array.leds.iter().any(|led| led.target == LedTarget::Network && led.active));
    assert!(!led_array.leds.iter().any(|led| led.target == LedTarget::Disk0 && led.active));
    assert!(!led_array.leds.iter().any(|led| led.target == LedTarget::Disk1 && led.active));
}

#[test]
fn toggle_with_apply() {
    let mut led_array = LedArray {
        leds: [
            LedState { target: LedTarget::Network, color: LedColor::Cool, active: false },
            LedState { target: LedTarget::Disk0,   color: LedColor::Hot,  active: false },
            LedState { target: LedTarget::Disk1,   color: LedColor::Hot,  active: false },
        ],
    };

    let mut led_output = FakeLedOutput { last_snapshot: Vec::new() };
    led_output.apply(&led_array);
    assert_eq!(led_output.last_snapshot.len(), 3);
    assert!(led_output.last_snapshot.iter().any(|led| led.target == LedTarget::Network && !led.active));

    led_array.toggle_target(LedTarget::Network);
    led_output.apply(&led_array);
    assert!(led_output.last_snapshot.iter().any(|led| led.target == LedTarget::Network && led.active));
}
