use std::{
    sync::{LazyLock, PoisonError, RwLock},
    thread,
    time::Duration,
};

use bytesize::ByteSize as Bytes;
use sysinfo::{
    Component, Components, CpuRefreshKind, MINIMUM_CPU_UPDATE_INTERVAL, MemoryRefreshKind,
    RefreshKind, System,
};

use crate::host::{Host, Load, Name, Percent, Temperature, Uptime, Usage, uptime_span};

const READ: &str = "failed to read host info";
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
    system: System,
    components: Components,
}

fn refresh_kind() -> RefreshKind {
    let cpu = CpuRefreshKind::nothing().with_cpu_usage();

    let memory = MemoryRefreshKind::nothing().with_ram().with_swap();

    RefreshKind::nothing().with_cpu(cpu).with_memory(memory)
}

fn name(string: String) -> Option<Name> {
    Name::new(string).ok()
}

impl Monitor {
    fn new() -> Self {
        let specifics = refresh_kind();

        let mut system = System::new_with_specifics(specifics);

        // cpu usage is computed as difference between measurements,
        // so measure baseline via waiting to avoid returning zeroes
        thread::sleep(MINIMUM_CPU_UPDATE_INTERVAL);

        system.refresh_specifics(specifics);

        let components = Components::new_with_refreshed_list();

        Self { system, components }
    }

    fn temperature(&self) -> Option<Temperature> {
        self.components.list().iter().find_map(temperature)
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

    fn read(&self) -> Host {
        self.try_read().expect(READ)
    }

    fn try_read(&self) -> Option<Host> {
        let average = System::load_average();

        let one = Percent::new(average.one)?;
        let five = Percent::new(average.five)?;
        let fifteen = Percent::new(average.fifteen)?;

        let load = Load::new(one, five, fifteen);

        let system = System::long_os_version().and_then(name);

        let name = System::host_name().and_then(name);

        let duration = Duration::from_secs(System::uptime());

        let span = uptime_span(duration)?;

        let uptime = Uptime::new(span);

        let cores = self.system.cpus().len();

        let usage = self.system.global_cpu_usage() as f64;

        let cpu = Percent::new(usage)?;

        let used_memory = Bytes::b(self.system.used_memory());
        let total_memory = Bytes::b(self.system.total_memory());

        let memory = Usage::new(used_memory, total_memory)?;

        let used_swap = Bytes::b(self.system.used_swap());
        let total_swap = Bytes::b(self.system.total_swap());

        let swap = Usage::new(used_swap, total_swap)?;

        let temperature = self.temperature();

        let host = Host {
            name,
            system,
            uptime,
            load,
            cores,
            cpu,
            memory,
            swap,
            temperature,
        };

        Some(host)
    }
}

fn extract<T>(poison_error: PoisonError<T>) -> T {
    poison_error.into_inner()
}

fn setup() -> RwLock<Host> {
    let mut monitor = Monitor::new();

    let initial = monitor.snapshot();

    thread::Builder::new()
        .name(NAME.to_owned())
        .spawn(move || {
            loop {
                thread::sleep(INTERVAL);

                let snapshot = monitor.snapshot();

                let mut latest = LATEST.write().unwrap_or_else(extract);

                *latest = snapshot;
            }
        })
        .expect(SPAWN);

    RwLock::new(initial)
}

static LATEST: LazyLock<RwLock<Host>> = LazyLock::new(setup);

pub fn snapshot() -> Host {
    LATEST.read().unwrap_or_else(extract).clone()
}

pub fn start() {
    LazyLock::force(&LATEST);
}
