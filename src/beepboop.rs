/// A string returner that alternates between returning beep and boop
pub struct BeepBooper {
    state: bool,
}

impl BeepBooper {
    /// Returns a new BeepBooper with a state `true`
    /// Must be initialized as mutable
    pub fn new() -> BeepBooper {
        BeepBooper { state: true }
    }

    /// Returns a string based on BeepBooper's state
    pub fn beep(&mut self) -> String {
        let result: String;
        if self.state {
            result = String::from("beep");
        } else {
            result = String::from("boop");
        };
        self.state = !self.state;
        return result;
    }
}

#[test]
fn test_beeper_messages() {
    let mut beeper = BeepBooper::new();
    assert!(beeper.beep().contains("beep"));
    assert!(beeper.beep().contains("boop"));
}
