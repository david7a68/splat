---
splat_schema: 1
splat_id: b328bae9-3a97-5fa7-8ea8-29dc2f473890
splat_kind: rule
splat_key: dnd2024:rules/weapons_slow
splat_name: Slow
splat_origin:
  key: dnd2024:reference/pages/090
splat_sources:
- title: SRD 5.2.1
  locator: pp. 90
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/weapons_slow
  parameters:
    trigger:
      hit_creature: true
      damage_dealt: true
    optional: true
    speed_reduction_feet: 10
    duration:
      until: attacker_next_turn_start
    maximum_combined_slow_reduction_feet: 10
  implementation: dnd2024:rules_weapons_slow
splat_contracts:
- id: dnd2024:rules_weapons_slow
  inputs:
    actor:
      type: participant
      required: true
    targets:
      type: participant_list
      required: true
    event:
      type: event
      required: true
    choice:
      type: choice
      required: true
  outputs:
  - action_options
  - effects
  - resource_updates
  events:
  - attack.hit
  capabilities:
  - action_economy
  - movement
  - combat
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 90
  engine_gaps: &id001
  - dnd2024:rules_weapons_slow
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_3.md
  specification_sha256: 42c6e62c7fdf3c1e2dadb87b88064ae8f96d990d8e873d6c340ee1220a97a0ef
  parent_name: Weapons
  line_span:
    first: 7801
    count: 6
splat_clauses:
- key: source_rules
  text: '# Slow If you hit a creature with this weapon and deal damage to it, you can reduce its Speed
    by 10 feet until the start of your next turn. If the creature is hit more than once by weapons that
    have this property, the Speed reduction doesn’t exceed 10 feet.'
  sha256: 867d11b7992d15606183c52b4296933fc6bafde6f8f206dcee3fd860c665ac86
  status: contract_specified
splat_requires_mechanics: *id001
---

# Slow

<!-- SRD page 90 -->

If you hit a creature with this weapon and deal\
damage to it, you can reduce its Speed by 10 feet un-\
til the start of your next turn. If the creature is hit\
more than once by weapons that have this property,\
the Speed reduction doesn’t exceed 10 feet.\
