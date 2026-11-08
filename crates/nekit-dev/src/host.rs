use std::{fmt, time::Duration};

use dioxus::{fullstack::ServerEvents, prelude::*};

use bytesize::ByteSize as Bytes;
use jiff::{RoundMode, SignedDuration, Span, SpanRound, Unit, Zoned};
use non_empty_str::{EmptyString, NonEmptyString};
use serde::{Deserialize, Deserializer, Serialize, Serializer, de::Error};

#[cfg(feature = "server")]
use crate::monitor::{snapshot, subscribe};

use crate::times::zoned;

pub fn optional() -> Option<Host> {
    cfg_select! {
        feature = "server" => {
            let host = snapshot();

            Some(host)
        }
        _ => {
            None
        }
    }
}

pub const INITIAL_FAILING: bool = false;

pub struct Display<'s, T> {
    option: Option<&'s T>,
}

impl<'s, T> Display<'s, T> {
    pub const fn new(option: Option<&'s T>) -> Self {
        Self { option }
    }
}

impl<T: fmt::Display> fmt::Display for Display<'_, T> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.option {
            Some(value) => value.fmt(formatter),
            None => formatter.write_str(UNAVAILABLE),
        }
    }
}

#[get("/host/events")]
pub async fn events() -> Result<ServerEvents<Host>> {
    let mut receiver = subscribe();

    let server_events = ServerEvents::new(move |mut sender| async move {
        loop {
            // clone out of the borrow before everything else
            let host = receiver.borrow_and_update().clone();

            // fails once the client has disconnected
            if sender.send(host).await.is_err() {
                break;
            }

            // fails only if the monitor is gone
            if receiver.changed().await.is_err() {
                break;
            }
        }
    });

    Ok(server_events)
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Value<T> {
    name: String,
    option: Option<T>,
    updated: Option<Zoned>,
    failing: bool,
}

impl<T> Value<T> {
    pub const fn new(name: String) -> Self {
        Self {
            name,
            option: None,
            updated: None,
            failing: INITIAL_FAILING,
        }
    }

    pub const fn name(&self) -> &str {
        self.name.as_str()
    }

    pub const fn option(&self) -> Option<&T> {
        self.option.as_ref()
    }

    pub const fn is_failing(&self) -> bool {
        self.failing
    }

    pub const fn is_not_failing(&self) -> bool {
        !self.is_failing()
    }

    pub const fn display(&self) -> Display<'_, T> {
        Display::new(self.option())
    }

    pub const fn set_failing(&mut self) {
        self.failing = true;
    }

    pub const fn set_not_failing(&mut self) {
        self.failing = false;
    }

    pub fn record_value(&mut self, value: T) {
        self.option = Some(value);

        // read the current time to record when the last update was made
        self.updated = Some(zoned());

        self.set_not_failing();
    }

    pub fn record_error<E: fmt::Display>(&mut self, error: E) {
        // the last good value is kept along with its instant

        if self.is_not_failing() {
            warn!(%error, "failed to record `{name}`", name = self.name());
        }

        self.set_failing();
    }

    pub fn record<E: fmt::Display>(&mut self, result: Result<T, E>) {
        match result {
            Ok(value) => self.record_value(value),
            Err(error) => self.record_error(error),
        }
    }
}

pub type EmptyName = EmptyString;
pub type Name = NonEmptyString;
pub type Cores = usize;
pub type Float = f64;
pub type TemperatureFloat = f32;

pub const NULL: Float = 0.0;
pub const FULL: Float = 100.0;

pub const ZERO: TemperatureFloat = 0.0;

pub const PRECISION: usize = 2;

pub const NEGATIVE_LOAD: &str = "value less than zero";
pub const PERCENTAGE: &str = "value out of [0, 100] range";

pub const PERCENT: &str = "%";
pub const CELSIUS: &str = "°C";

pub const UNAVAILABLE: &str = "n/a";

pub const WRONG_USAGE: &str = "used < total or total is zero";

pub const DURATION: &str = "duration conversion failed";

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Uptime {
    span: Span,
}

impl PartialEq for Uptime {
    fn eq(&self, other: &Self) -> bool {
        self.get() == other.get().fieldwise()
    }
}

impl fmt::Display for Uptime {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{span:#}", span = self.get())
    }
}

pub fn uptime_span(duration: Duration) -> Option<Span> {
    let signed: SignedDuration = duration.try_into().ok()?;

    let span: Span = signed.try_into().ok()?;

    let span_round = round();

    let uptime = span.round(span_round).ok()?;

    Some(uptime)
}

pub type Round = SpanRound<'static>;

pub fn round() -> Round {
    SpanRound::new()
        .largest(Unit::Day)
        .smallest(Unit::Minute)
        .mode(RoundMode::Trunc)
        .days_are_24_hours()
}

impl Uptime {
    pub const fn new(span: Span) -> Self {
        Self { span }
    }

    pub const fn get(self) -> Span {
        self.span
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Temperature {
    celsius: TemperatureFloat,
}

impl Default for Temperature {
    fn default() -> Self {
        Self::ZERO
    }
}

impl fmt::Display for Temperature {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{celsius:.0}{CELSIUS}", celsius = self.get())
    }
}

impl Temperature {
    pub const fn new(celsius: TemperatureFloat) -> Self {
        Self { celsius }
    }

    pub const fn get(self) -> TemperatureFloat {
        self.celsius
    }

    pub const ZERO: Self = Self::new(ZERO);
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Percent {
    value: Float,
}

impl fmt::Display for Percent {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{value:.PRECISION$}{PERCENT}",
            value = self.get()
        )
    }
}

impl Default for Percent {
    fn default() -> Self {
        Self::NULL
    }
}

impl Serialize for Percent {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.get().serialize(serializer)
    }
}

fn percentage<E: Error>() -> E {
    E::custom(PERCENTAGE)
}

impl<'de> Deserialize<'de> for Percent {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = Float::deserialize(deserializer)?;

        let percent = Self::new(value).ok_or_else(percentage)?;

        Ok(percent)
    }
}

impl Percent {
    pub const fn new(value: Float) -> Option<Self> {
        if value.is_nan() || value < NULL || value > FULL {
            None
        } else {
            // SAFETY: just checked that `value` is indeed valid
            let percent = unsafe { Self::new_unchecked(value) };

            Some(percent)
        }
    }

    pub const unsafe fn new_unchecked(value: Float) -> Self {
        Self { value }
    }

    pub const fn get(self) -> Float {
        self.value
    }

    pub const fn reverse(self) -> Self {
        let value = FULL - self.get();

        // SAFETY: value in [0, 100] => (100 - value) in [0, 100]
        unsafe { Self::new_unchecked(value) }
    }

    pub const NULL: Self = Self::new(NULL).unwrap();
    pub const FULL: Self = Self::new(FULL).unwrap();
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SpecificLoad {
    value: Float,
}

impl fmt::Display for SpecificLoad {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{value:.PRECISION$}", value = self.get())
    }
}

impl Serialize for SpecificLoad {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.get().serialize(serializer)
    }
}

fn negative_load<E: Error>() -> E {
    E::custom(NEGATIVE_LOAD)
}

impl<'de> Deserialize<'de> for SpecificLoad {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = Float::deserialize(deserializer)?;

        let load = Self::new(value).ok_or_else(negative_load)?;

        Ok(load)
    }
}

impl SpecificLoad {
    pub const fn new(value: Float) -> Option<Self> {
        if value.is_nan() || value < NULL {
            None
        } else {
            // SAFETY: just checked that `value` is indeed valid
            let specific_load = unsafe { Self::new_unchecked(value) };

            Some(specific_load)
        }
    }

    pub const unsafe fn new_unchecked(value: Float) -> Self {
        Self { value }
    }

    pub const fn get(self) -> Float {
        self.value
    }

    pub const NULL: Self = Self::new(NULL).unwrap();
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Load {
    pub one: SpecificLoad,
    pub five: SpecificLoad,
    pub fifteen: SpecificLoad,
}

impl fmt::Display for Load {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{one} {five} {fifteen}",
            one = self.one(),
            five = self.five(),
            fifteen = self.fifteen()
        )
    }
}

impl Load {
    pub fn new(one: SpecificLoad, five: SpecificLoad, fifteen: SpecificLoad) -> Self {
        Self { one, five, fifteen }
    }

    pub const fn one(&self) -> SpecificLoad {
        self.one
    }

    pub const fn five(&self) -> SpecificLoad {
        self.five
    }

    pub const fn fifteen(&self) -> SpecificLoad {
        self.fifteen
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
struct UncheckedUsage {
    used: Bytes,
    total: Bytes,
}

impl UncheckedUsage {
    const fn new(used: Bytes, total: Bytes) -> Self {
        Self { used, total }
    }

    const fn used(self) -> Bytes {
        self.used
    }

    const fn total(self) -> Bytes {
        self.total
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Usage {
    pub used: Bytes,
    pub total: Bytes,
}

impl Serialize for Usage {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.unchecked().serialize(serializer)
    }
}

fn wrong_usage<E: Error>() -> E {
    E::custom(WRONG_USAGE)
}

impl<'de> Deserialize<'de> for Usage {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let unchecked = UncheckedUsage::deserialize(deserializer)?;

        let usage = Self::try_unchecked(unchecked).ok_or_else(wrong_usage)?;

        Ok(usage)
    }
}

impl fmt::Display for Usage {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{used} / {total} ({percent:#})",
            used = self.used(),
            total = self.total(),
            percent = self.percent()
        )
    }
}

pub const ZERO_BYTES: Bytes = Bytes::b(0);

fn non_zero_bytes(bytes: Bytes) -> bool {
    bytes != ZERO_BYTES
}

impl Usage {
    pub fn new(used: Bytes, total: Bytes) -> Option<Self> {
        if used < total && non_zero_bytes(total) {
            // SAFETY: just checked the invariants are indeed upheld
            let usage = unsafe { Self::new_unchecked(used, total) };

            Some(usage)
        } else {
            None
        }
    }

    pub const unsafe fn new_unchecked(used: Bytes, total: Bytes) -> Self {
        Self { used, total }
    }

    fn try_unchecked(unchecked: UncheckedUsage) -> Option<Self> {
        Self::new(unchecked.used(), unchecked.total())
    }

    const fn unchecked(self) -> UncheckedUsage {
        UncheckedUsage::new(self.used(), self.total())
    }

    pub const fn used(self) -> Bytes {
        self.used
    }

    pub const fn total(self) -> Bytes {
        self.total
    }

    pub const fn percent(self) -> Percent {
        let value = self.used().as_u64() as Float / self.total().as_u64() as Float * FULL;

        // SAFETY: used < total => (used / total) in [0, 1] => (used / total * 100) in [0, 100]
        unsafe { Percent::new_unchecked(value) }
    }
}

pub const NAME: &str = "name";
pub const SYSTEM: &str = "system";
pub const UPTIME: &str = "uptime";
pub const LOAD: &str = "load";
pub const CORES: &str = "cores";
pub const CPU: &str = "cpu";
pub const MEMORY: &str = "memory";
pub const SWAP: &str = "swap";
pub const TEMPERATURE: &str = "temperature";

pub fn value<T>(name: &str) -> Value<T> {
    Value::new(name.to_owned())
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Host {
    pub name: Value<Name>,
    pub system: Value<Name>,
    pub uptime: Value<Uptime>,
    pub load: Value<Load>,
    pub cores: Value<Cores>,
    pub cpu: Value<Percent>,
    pub memory: Value<Usage>,
    pub swap: Value<Usage>,
    pub temperature: Value<Temperature>,
}

impl Default for Host {
    fn default() -> Self {
        Self::new()
    }
}

impl Host {
    pub fn new() -> Self {
        Self {
            name: value(NAME),
            system: value(SYSTEM),
            uptime: value(UPTIME),
            load: value(LOAD),
            cores: value(CORES),
            cpu: value(CPU),
            memory: value(MEMORY),
            swap: value(SWAP),
            temperature: value(TEMPERATURE),
        }
    }

    pub const fn display_name(&self) -> Display<'_, Name> {
        self.name.display()
    }

    pub const fn display_system(&self) -> Display<'_, Name> {
        self.system.display()
    }

    pub const fn display_uptime(&self) -> Display<'_, Uptime> {
        self.uptime.display()
    }

    pub const fn display_load(&self) -> Display<'_, Load> {
        self.load.display()
    }

    pub const fn display_cores(&self) -> Display<'_, Cores> {
        self.cores.display()
    }

    pub const fn display_cpu(&self) -> Display<'_, Percent> {
        self.cpu.display()
    }

    pub const fn display_memory(&self) -> Display<'_, Usage> {
        self.memory.display()
    }

    pub const fn display_swap(&self) -> Display<'_, Usage> {
        self.swap.display()
    }

    pub const fn display_temperature(&self) -> Display<'_, Temperature> {
        self.temperature.display()
    }

    pub const fn cpu_percent(&self) -> Option<Percent> {
        self.cpu.option().copied()
    }

    pub const fn memory_percent(&self) -> Option<Percent> {
        if let Some(usage) = self.memory.option() {
            Some(usage.percent())
        } else {
            None
        }
    }

    pub const fn swap_percent(&self) -> Option<Percent> {
        if let Some(usage) = self.swap.option() {
            Some(usage.percent())
        } else {
            None
        }
    }
}
