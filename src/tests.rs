use super::*;

#[test]
fn fallback_is_symmetric_but_never_drops_a_global_requirement() {
    let v1 = NonZeroU16::MIN;
    let v2 = v1.saturating_add(1);
    let common = Profile {
        version: v1,
        supported: 1,
        required: 0,
    };
    let a = Offer {
        profiles: &[
            Profile {
                version: v2,
                supported: 3,
                required: 2,
            },
            common,
        ],
        required: 1,
    };
    let b = Offer {
        profiles: &[
            common,
            Profile {
                version: v2,
                ..common
            },
        ],
        required: 0,
    };
    let expected = Ok(Agreement {
        version: v1,
        enabled: 1,
    });
    assert_eq!(select(a, b), expected);
    assert_eq!(select(b, a), expected);
    assert_eq!(
        select(Offer { required: 2, ..a }, b),
        Err(Refusal::UnmetRequirements)
    );
    assert_eq!(
        select(
            Offer {
                profiles: &[common, common],
                ..a
            },
            b
        ),
        Err(Refusal::Malformed)
    );
    assert_eq!(
        select(
            Offer {
                profiles: &a.profiles[..1],
                ..a
            },
            Offer {
                profiles: &b.profiles[..1],
                ..b
            }
        ),
        Err(Refusal::NoCommonVersion)
    );
}
