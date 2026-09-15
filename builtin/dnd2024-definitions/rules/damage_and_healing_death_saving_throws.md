---
splat_schema: 1
splat_id: 27e4fbaf-5e7c-5c22-99b0-3bcd2ba57d77
splat_kind: rule
splat_key: dnd2024:rules/damage_and_healing_death_saving_throws
splat_name: Death Saving Throws
splat_origin:
  key: dnd2024:reference/pages/017
splat_sources:
- title: SRD 5.2.1
  locator: pp. 17, 18
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/damage_and_healing_death_saving_throws
  parameters: {}
splat_conversion:
  status: partial
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 17
  - 18
  engine_gaps: []
  conversion_gaps:
  - source_rules
  execution_profiles: []
  parent_name: Damage and Healing
  line_span:
    first: 1255
    count: 23
splat_clauses:
- key: source_rules
  text: '# Death Saving Throws Whenever you start your turn with 0 Hit Points, you must make a Death Saving
    Throw to determine whether you creep closer to death or hang on to life. Unlike other saving throws,
    this one isn’t tied to an ability score. You’re in the hands of fate now. Three Successes/Failures.
    Roll 1d20. If the roll is 10 or higher, you succeed. Otherwise, you fail. A success or failure has
    no effect by itself. On your third success, you become Stable (see “Stabilizing a Character” below).
    On your third failure, you die. The successes and failures don’t need to be consecutive; keep track
    of both until you collect three of a kind. The number of both is reset to zero when you regain any
    Hit Points or become Stable. Rolling a 1 or 20. When you roll a 1 on the d20 for a Death Saving Throw,
    you suffer two failures. If you roll a 20 on the d20, you regain 1 Hit Point. Damage at 0 Hit Points.
    If you take any damage while you have 0 Hit Points, you suffer a Death Saving Throw failure. If the
    damage is from a Critical Hit, you suffer two failures instead. If the damage equals or exceeds your
    Hit Point maximum, you die.'
  sha256: e8d3712a91276798e2257080b356f5c5cfee7ae25c16a3e75d9c8eb95bf817c3
  status: needs_semantic_conversion
---

# Death Saving Throws

<!-- SRD page 17 -->

Whenever you start your turn with 0 Hit Points,\
you must make a Death Saving Throw to determine\
whether you creep closer to death or hang on to life.\
Unlike other saving throws, this one isn’t tied to an\
ability score. You’re in the hands of fate now.\
Three Successes/Failures. Roll 1d20. If the roll\
is 10 or higher, you succeed. Otherwise, you fail. A\
success or failure has no effect by itself. On your\
third success, you become Stable (see “Stabilizing a\
Character” below). On your third failure, you die.\
The successes and failures don’t need to be con-\
secutive; keep track of both until you collect three\
of a kind. The number of both is reset to zero when\
you regain any Hit Points or become Stable.\

<!-- SRD page 18 -->

Rolling a 1 or 20. When you roll a 1 on the d20\
for a Death Saving Throw, you suffer two failures. If\
you roll a 20 on the d20, you regain 1 Hit Point.\
Damage at 0 Hit Points. If you take any damage\
while you have 0 Hit Points, you suffer a Death Sav-\
ing Throw failure. If the damage is from a Critical\
Hit, you suffer two failures instead. If the damage\
equals or exceeds your Hit Point maximum, you die.\
