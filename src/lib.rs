//! Bilateral profile selection; no wire protocol or session state.
#![no_std]

use core::num::NonZeroU16;

/// Maximum profiles in each bounded offer.
pub const MAX_PROFILES: usize = 64;

/// An exact version and its capability sets in a shared bit namespace.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Profile {
    pub version: NonZeroU16,
    pub supported: u64,
    pub required: u64,
}

/// One peer's offers and requirements that also apply on fallback.
#[derive(Debug, Clone, Copy)]
pub struct Offer<'a> {
    pub profiles: &'a [Profile],
    pub required: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Refusal {
    Malformed,
    NoCommonVersion,
    UnmetRequirements,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Agreement {
    pub version: NonZeroU16,
    pub enabled: u64,
}

/// Select the highest compatible version, regardless of offer order.
///
/// # Errors
/// Refuses empty or duplicate-version offers, requirements unsupported by
/// their own profile, and offers without a compatible common profile.
pub fn select(a: Offer<'_>, b: Offer<'_>) -> Result<Agreement, Refusal> {
    for offer in [a, b] {
        if offer.profiles.is_empty()
            || offer.profiles.len() > MAX_PROFILES
            || offer.profiles.iter().enumerate().any(|(i, p)| {
                p.required & !p.supported != 0
                    || offer.profiles[..i].iter().any(|q| q.version == p.version)
            })
        {
            return Err(Refusal::Malformed);
        }
    }
    let mut common = false;
    let mut best: Option<Agreement> = None;
    for left in a.profiles {
        if let Some(right) = b.profiles.iter().find(|p| p.version == left.version) {
            common = true;
            let enabled = left.supported & right.supported;
            let required = a.required | b.required | left.required | right.required;
            if required & !enabled == 0 && best.is_none_or(|pick| pick.version < left.version) {
                best = Some(Agreement {
                    version: left.version,
                    enabled,
                });
            }
        }
    }
    best.ok_or(if common {
        Refusal::UnmetRequirements
    } else {
        Refusal::NoCommonVersion
    })
}

#[cfg(test)]
mod tests;
