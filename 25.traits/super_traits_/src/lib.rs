trait Engine {
    fn start(&mut self);
    fn stop(&mut self);
    fn state(&self) -> bool;
}

trait Transmission {
    fn set_gear(&mut self, _:i32);
    fn gear(&mut self) -> i32;
}

trait Vehicle: Engine+Transmission {
    fn wheel_count(&self) -> i32;
}

trait Car: Vehicle {
    fn fuel_type(&self) -> FuelType;
}