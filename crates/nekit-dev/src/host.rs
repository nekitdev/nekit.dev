use std::{fmt, time::Duration};

use dioxus::{fullstack::ServerEvents, prelude::*};

use bytesize::ByteSize as Bytes;
use jiff::{RoundMode, SignedDuration, Span, SpanRound, Unit};
use non_empty_str::{NonEmptyStr, NonEmptyString};
use serde::{Deserialize, Deserializer, Serialize, Serializer, de::Error};

#[cfg(feature = "server")]
use crate::monitor::{snapshot, subscribe};

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

pub type Name = NonEmptyString;
pub type NameRef<'n> = &'n NonEmptyStr;
pub type Cores = usize;

pub const NULL: f64 = 0.0;
pub const FULL: f64 = 100.0;

pub const ZERO: f32 = 0.0;

pub const PRECISION: usize = 2;

pub const PERCENTAGE: &str = "value out of [0, 100] range";

pub const PERCENT: &str = "%";
pub const CELSIUS: &str = "°C";

pub const UNAVAILABLE: &str = "?";

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
    celsius: f32,
}

impl Default for Temperature {
    fn default() -> Self {
        Self::ZERO
    }
}

impl fmt::Display for Temperature {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{celsius:.0}", celsius = self.get())?;

        if formatter.alternate() {
            formatter.write_str(CELSIUS)?;
        }

        Ok(())
    }
}

impl Temperature {
    pub const fn new(celsius: f32) -> Self {
        Self { celsius }
    }

    pub const fn get(self) -> f32 {
        self.celsius
    }

    pub const ZERO: Self = Self::new(ZERO);
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Percent {
    value: f64,
}

impl fmt::Display for Percent {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{value:.PRECISION$}", value = self.get())?;

        if formatter.alternate() {
            formatter.write_str(PERCENT)?;
        }

        Ok(())
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
        let value = f64::deserialize(deserializer)?;

        let percent = Self::new(value).ok_or_else(percentage)?;

        Ok(percent)
    }
}

impl Percent {
    pub const fn new(value: f64) -> Option<Self> {
        if value < NULL || value > FULL {
            None
        } else {
            // SAFETY: just checked that `value` is indeed valid
            let percent = unsafe { Self::new_unchecked(value) };

            Some(percent)
        }
    }

    pub const unsafe fn new_unchecked(value: f64) -> Self {
        Self { value }
    }

    pub const fn get(self) -> f64 {
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

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Load {
    pub one: Percent,
    pub five: Percent,
    pub fifteen: Percent,
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
    pub const fn new(one: Percent, five: Percent, fifteen: Percent) -> Self {
        Self { one, five, fifteen }
    }

    pub const fn one(self) -> Percent {
        self.one
    }

    pub const fn five(self) -> Percent {
        self.five
    }

    pub const fn fifteen(self) -> Percent {
        self.fifteen
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Usage {
    pub used: Bytes,
    pub total: Bytes,
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

    pub const fn used(self) -> Bytes {
        self.used
    }

    pub const fn total(self) -> Bytes {
        self.total
    }

    pub const fn percent(self) -> Percent {
        let value = self.used().as_u64() as f64 / self.total().as_u64() as f64 * FULL;

        // SAFETY: used < total => (used / total) in [0, 1] => (used / total * 100) in [0, 100]
        unsafe { Percent::new_unchecked(value) }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Host {
    pub name: Option<Name>,
    pub system: Option<Name>,
    pub uptime: Uptime,
    pub load: Load,
    pub cores: Cores,
    pub cpu: Percent,
    pub memory: Usage,
    pub swap: Usage,
    pub temperature: Option<Temperature>,
}

impl Host {
    pub fn name(&self) -> Option<NameRef<'_>> {
        self.name.as_deref()
    }

    pub fn system(&self) -> Option<NameRef<'_>> {
        self.system.as_deref()
    }

    pub fn uptime(&self) -> Uptime {
        self.uptime
    }

    pub fn load(&self) -> Load {
        self.load
    }

    pub fn cores(&self) -> Cores {
        self.cores
    }

    pub fn cpu(&self) -> Percent {
        self.cpu
    }

    pub fn memory(&self) -> Usage {
        self.memory
    }

    pub fn swap(&self) -> Usage {
        self.swap
    }

    pub fn temperature(&self) -> Option<Temperature> {
        self.temperature
    }
}
