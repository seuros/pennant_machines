# Pennant Machines

> Lowering the pennant doesn't lower the requirement.

Pennant selects the highest compatible profile from two bounded offers,
regardless of which peer runs the selection or how either offer is ordered.
Two peers can speak the same version and still disagree on what it must do.

## The Flight Plan

1. **Agree on the signal book.** Define nonzero numeric versions and a shared
   feature-bit namespace. Build one `Profile` per offered version, with its
   supported and required bits. Each offer holds 1–64 distinct versions.
2. **Keep must-haves on the offer.** Put a requirement in `Profile::required`
   if it applies only to that version. Put it in `Offer::required` if it must
   survive fallback. In Foundation, fenced retirement is an offer-wide
   requirement: an older profile without it cannot start a Splice replacement.
3. **Select before proceeding.** Call `select` with the two offers. Use
   `Agreement::version` and only the shared features in `Agreement::enabled`.
   `Malformed` means an invalid offer, `NoCommonVersion` means no version
   appears in both, and `UnmetRequirements` means none of the common versions
   satisfies both peers' applicable requirements. Don't strip requirements to
   turn a refusal into an agreement.

## What's Yours

Pennant is a `no_std` selector, not a handshake. The application authenticates
and exchanges offers, and decides what to do if negotiation fails. There is no
wire format or session state.

**Status:** Unpublished local core. Foundation demonstrates the selection in
its expand/contract scenario; no production consumer is claimed.
