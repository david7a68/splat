# Read-only D&D Beyond character viewer

Open **Campaign → Party**. Link the active campaign in Settings and connect your
D&D Beyond session to fetch its party. Saved local copies remain available offline.

Select a member and use Overview, Actions, Spells, Equipment, Features, or
Source & parity. Totals and attacks come from D&D Beyond's simplified computed
character service (SCDS), rather than Splat's earlier basic formulas. Detailed
source data comes from the character v5 endpoint. Source & parity explains the
record's provenance, retrieval time, cache path, and remaining gaps.

The viewer uses the campaign event stream while a sheet is open. A character
update triggers a fresh read; reconnecting also rechecks the sheet. A five-minute
reconciliation read covers missed events. If the stream cannot connect, the open
sheet falls back to 15-second checks. Closing the Party view stops synchronization.
**Refresh party** fetches membership and all current characters manually.

Character records are now persistent. This replaces the original memory-only
viewer as explicitly requested. The cache uses the shared creature definition
and fetched state, with stable IDs and complete source records. No character edits
are sent to D&D Beyond. Encounter-builder integration is not part of this stage.

Details:

- [Creature cache schema, storage, parity, and update behavior](design/character-cache.md)
- [Avrae, DDB Importer, Beyond20, and official API references](research/dndbeyond-api.md)
- [Initial viewer screenshots and Claude Fable review](../artifacts/screenshots/party/README.md)

## Validation

Normal tests use synthetic data for cache round trips, supplied totals, inventory
identity, missing data, scope changes, queued responses, and event routing.
The opt-in integration test fetches the configured party, checks normalized values
against SCDS, validates normal creature entries, writes/reads the local cache, and
connects to the official event stream:

```sh
devenv shell cargo test --locked --bin splat live_parity_cache_and_stream -- --ignored --nocapture
```

It passed for Morana, Rohan of Unc, and Tiny in Heralds of the End. The live listener also received both user-initiated temporary-HP update events.
Other edit categories are not yet individually verified.
