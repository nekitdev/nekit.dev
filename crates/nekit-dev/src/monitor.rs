use std::{sync::LazyLock, thread, time::Duration};

use bytesize::ByteSize as Bytes;
use sysinfo::{
    Component, Components, CpuRefreshKind, LoadAvg, MINIMUM_CPU_UPDATE_INTERVAL, MemoryRefreshKind,
    RefreshKind, System,
};
use tokio::sync::watch;

use crate::host::{
    DURATION, Float, Host, Load, NEGATIVE_LOAD, Name, PERCENT, Percent, SpecificLoad, Temperature,
    Uptime, Usage, WRONG_USAGE, uptime_span,
};

const READ: &str = "failed to read host info";
const COMPONENTS: &str = "failed to read host components";
const SPAWN: &str = "failed to spawn host monitor thread";

const NAME: &str = "monitor";

const INTERVAL: Duration = Duration::from_secs(2);

const COUNT: usize = 3;

const TARGET: [&str; COUNT] = ["cpu", "package", "soc"];

fn is_target<L: AsRef<str>>(label: L) -> bool {
    let string = label.as_ref();

    TARGET.into_iter().any(|target| string.contains(target))
}

fn temperature(component: &Component) -> Option<Temperature> {
    let label = component.label().to_lowercase();

    if is_target(label) {
        component.temperature().map(Temperature::new)
    } else {
        None
    }
}

struct Monitor {
    host: Host,
    system: System,
    components: Components,
}

fn refresh_kind() -> RefreshKind {
    let cpu = CpuRefreshKind::nothing().with_usage();

    let memory = MemoryRefreshKind::nothing().with_ram().with_swap();

    RefreshKind::nothing().with_cpu(cpu).with_memory(memory)
}

fn load(average: LoadAvg) -> Option<Load> {
    let one = SpecificLoad::new(average.one)?;
    let five = SpecificLoad::new(average.five)?;
    let fifteen = SpecificLoad::new(average.fifteen)?;

    let load = Load::new(one, five, fifteen);

    Some(load)
}

fn uptime(duration: Duration) -> Option<Uptime> {
    let span = uptime_span(duration)?;

    let uptime = Uptime::new(span);

    Some(uptime)
}

pub const TEMPERATURE: &str = "temperature not found";

impl Monitor {
    fn new() -> Self {
        let host = Host::new();

        let specifics = refresh_kind();

        let mut system = System::new_with_specifics(specifics).expect(READ);

        // cpu usage is computed as difference between measurements,
        // so measure baseline via waiting to avoid returning zeroes
        thread::sleep(MINIMUM_CPU_UPDATE_INTERVAL);

        system.refresh_specifics(specifics);

        let components = Components::new_with_refreshed_list().expect(COMPONENTS);

        Self {
            host,
            system,
            components,
        }
    }

    fn refresh(&mut self) {
        let specifics = refresh_kind();

        self.system.refresh_specifics(specifics);

        self.components.refresh(false); // do not remove not listed components
    }

    fn snapshot(&mut self) -> Host {
        self.refresh();

        self.read()
    }

    // BEWARE OF THE LONGEST FUNCTION!
    fn read(&mut self) -> Host {
        let host = &mut self.host;

        // load

        match System::load_average() {
            Ok(average) => {
                let result = load(average).ok_or(NEGATIVE_LOAD);

                host.load.record(result);
            }
            Err(error) => {
                host.load.record_error(error);
            }
        }

        // system

        match System::long_os_version() {
            Ok(string) => {
                let result = Name::new(string);

                host.system.record(result);
            }
            Err(error) => {
                host.system.record_error(error);
            }
        }

        // name

        match System::host_name() {
            Ok(string) => {
                let result = Name::new(string);

                host.name.record(result);
            }
            Err(error) => {
                host.name.record_error(error);
            }
        }

        // uptime

        match System::uptime() {
            Ok(seconds) => {
                let duration = Duration::from_secs(seconds);

                let result = uptime(duration).ok_or(DURATION);

                host.uptime.record(result);
            }
            Err(error) => {
                host.uptime.record_error(error);
            }
        }

        // cores

        let cores = self.system.cpus().len();

        host.cores.record_value(cores);

        // usage

        let usage = self.system.global_cpu_usage() as Float;

        let result = Percent::new(usage).ok_or(PERCENT);

        host.cpu.record(result);

        // memory

        let used_memory = Bytes::b(self.system.used_memory());
        let total_memory = Bytes::b(self.system.total_memory());

        let result = Usage::new(used_memory, total_memory).ok_or(WRONG_USAGE);

        host.memory.record(result);

        // swap

        let used_swap = Bytes::b(self.system.used_swap());
        let total_swap = Bytes::b(self.system.total_swap());

        let result = Usage::new(used_swap, total_swap).ok_or(WRONG_USAGE);

        host.swap.record(result);

        // temperature

        let result = self
            .components
            .iter()
            .find_map(temperature)
            .ok_or(TEMPERATURE);

        host.temperature.record(result);

        host.clone()
    }
}

fn setup() -> watch::Sender<Host> {
    let mut monitor = Monitor::new();

    let (sender, _) = watch::channel(monitor.snapshot());

    thread::Builder::new()
        .name(NAME.to_owned())
        .spawn(move || {
            loop {
                thread::sleep(INTERVAL);

                let snapshot = monitor.snapshot();

                CHANNEL.send_replace(snapshot);
            }
        })
        .expect(SPAWN);

    sender
}

static CHANNEL: LazyLock<watch::Sender<Host>> = LazyLock::new(setup);

pub fn snapshot() -> Host {
    CHANNEL.borrow().clone()
}

pub fn subscribe() -> watch::Receiver<Host> {
    CHANNEL.subscribe()
}

pub fn start() {
    LazyLock::force(&CHANNEL);
}
