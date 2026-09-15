# Schema review checks

These are design checks, not executable simulator tests. YAML/reference checks
can validate the notes, while the state outcomes below are hand-worked against
the specified contracts. Together with [Training Exchange](encounter-trace.md)
they cover the newly completed fields.

| Check | Setup and expected observation |
| --- | --- |
| Reactive budget | Reactive Warden has four heads: capacity three extra opportunity reactions. After spending two and gaining two heads, one extra use remains. Next own-turn refresh with six heads gives five. Protect Ally cannot spend that budget |
| Fear targets | Controller Warden applies Frighten to A and B. One use creates one Concentration dependency and two target effects. A succeeds on its end-turn Wisdom save: only A's effect ends. Losing caster Concentration then ends B's effect |
| Suppression | Two equally potent applications overlap on B; the newest contributes under the same-effect policy. End it before the older one's expiry: the older can contribute again. No target effect is deleted merely because it was suppressed |
| Budget frequency | A Marilith-profile actor reacts on two consecutive creatures' turns; each turn has one permitted use, not unlimited uses. An ordinary reaction profile cannot do this before its own refresh |
| Damage | 28 damage, reduction 5, resistance, vulnerability: 28−5=23; halve and round down to 11; double to 22. Do not cancel resistance/vulnerability before rounding |
| Temporary HP | HP 10, temp HP 5, incoming damage 7 gives temp HP 0 and HP 8. A relevant concentration check still sees damage taken. A new temp-HP grant asks old-or-new, not automatic addition |
| Knockout | A qualifying melee attack would reduce HP to zero. The attacker chooses knockout: target is at 1 HP with the source-specific Unconscious/Short Rest state, not a zero-HP death-save state |
| Grapple | Two different grapplers create distinct relationships to one target. Ending one frees its body part and removes only its condition contribution; the other grapple remains |
| Form | Revised Polymorph retains original HP/Hit Dice and grants temporary HP. Ending that spell removes its specified remaining grant and form, without restoring a separate beast HP pool. A Wild Shape profile does not inherit Polymorph's end-on-no-temp-HP condition |
| Possession | Ghost-profile actor and host keep separate IDs. Host body control changes through a relation; ending it restores control and any specified source-bound immunity, not a replacement host with reset resources |
| Reinforcement | Reinforcement Drill starts with the red actor absent. Required arrival remains pending until round three. It cannot be treated as an early victory. Arrival grants only the next eligible turn under initiative rules |
| Sequence | Two Exchanges binds the same four actors twice. HP, item quantities and pools persist across steps; the explicit rest supplies only its legitimate recovery. Second placement does not reset authored initial HP |
| Inventory | Move a setup-instantiated pack to a bearer that already has another pack from the same setup. IDs/contents/pools stay distinct even though both origins use `pack`/`pouch` local keys |
| Key rename | A reference that resolves to the same UUID before/after a key rename has the same mechanical fingerprint, provided all other resolved inputs are unchanged |
| Formatting | Markdown prose, heading, navigation-link or YAML formatting changes do not change the resolved semantic fingerprint |
| Meaningful edit | A changed HP maximum, charge pool, strategy priority, assumption, engine semantics or enabled relevant rule changes the aggregate fingerprint |
| Live failure | Missing required D&D Beyond data blocks a new affected run; it does not fetch a saved character snapshot. Existing runs retain their in-memory inputs |
| Unknown mechanic | An action naming an unavailable operation remains editable but cannot be simulated. It is never silently treated as a no-op |
| Hidden invalidity | Invalid spell target with apparent successful save creates no actual save roll and does not leak hidden target type to the restricted strategy |
| Readied spell | Manual encounter tracks the spell, stated trigger, undecided targets, resources and Concentration. Release decisions come from player/DM; automatic trials cannot silently choose them |

## Negative validation cases

- Duplicate UUID, resource key, local action key, YAML mapping key or inventory
  row key: report exact conflicting locations; never first/last wins.
- Reference with both `id` and `key`, wrong entry kind, missing pool/roll/binding,
  unsupported future schema, or unknown required operation: affected use blocked.
- Local inventory table under a row referencing a populated container setup:
  reject the dual content authority. A container cycle is also invalid.
- Capacity measurement unknown for a required capacity check: unresolved,
  never zero. Invalid hand occupancy prevents the equipment transition.
- A private campaign member in a shared package reference: invalid ownership
  dependency. A shared party slot is allowed and binds only in the campaign.
- Form/possession policy omits required retained/changed-state behavior: fail
  profile validation; never guess a 2014 HP-replacement or generic stat swap.
- A seed with malformed hashes, missing inputs, unsupported RNG/engine identity,
  or mismatched semantic digest: no exact-replay claim.
- Unsupported source ruling in a multi-effect interaction: require an explicit
  applicable rule before automatic execution. A schema slot alone is insufficient.

## Review result boundary

All original coverage families now have candidate field/operation contracts.
The remaining work is review, implementing those contracts, supplying validated
core/source content and exercising it with real engine tests. No broad schema
branch is intentionally left unnamed. This is not a claim that every published
feature is encoded or that any simulator test has passed.
