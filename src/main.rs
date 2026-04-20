trait LedActivator {
    fn toggle(&self);
}

struct LedActivatorImpl;

impl LedActivator for LedActivatorImpl {
    fn toggle(&self) {
        
    }
}


#[derive(PartialEq)]
enum LedTarget  {
    Network,
    Disk0,
    Disk1,
}

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
}

fn main() {
    let mut led_array = LedArray {
        leds: [
            LedState { target: LedTarget::Network, color: LedColor::Cool, active: false },
            LedState { target: LedTarget::Disk0,   color: LedColor::Hot,  active: false },
            LedState { target: LedTarget::Disk1,   color: LedColor::Hot,  active: false },
        ],
    };

    println!("{}",led_array.describe());
    led_array.toggle_target(LedTarget::Network);
    println!("After toggling the Network LED:");
    println!("{}",led_array.describe());
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

