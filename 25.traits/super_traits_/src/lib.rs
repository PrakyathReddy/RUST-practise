trait Engine {
    fn start(&mut self);
    fn stop(&mut self);
    fn state(&self) -> bool;
}