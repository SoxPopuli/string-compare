
trait Method {
    fn name() -> String;
    fn is_normalized() -> bool;
    fn distance(&self, reference: &str, value: &str) -> f32;

    fn normalize(value: f32, max: f32) -> f32 {
        if Self::is_normalized() {
            value
        } else {
            value / max
        }
    }
}

