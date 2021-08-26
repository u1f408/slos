use slos::clock;

#[derive(Debug)]
pub enum HostedInterrupt {
	ClockTick,
}

impl HostedInterrupt {
	pub fn dispatch(&self) {
		match self {
			Self::ClockTick => {
				clock::on_tick();
			}
		}
	}
}
