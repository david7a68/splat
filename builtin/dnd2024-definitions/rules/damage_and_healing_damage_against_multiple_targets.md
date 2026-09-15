---
splat_schema: 1
splat_id: 26ef7018-a0f9-5fcb-b61c-9e615779d05f
splat_kind: rule
splat_key: dnd2024:rules/damage_and_healing_damage_against_multiple_targets
splat_name: Damage against Multiple Targets
splat_origin:
  key: dnd2024:reference/pages/016
splat_sources:
- title: SRD 5.2.1
  locator: pp. 16
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/damage_and_healing_damage_against_multiple_targets
  parameters:
    shared_damage_roll:
      minimum_targets: 2
      requires_simultaneous_saving_throws: true
      roll_count: 1
  implementation: dnd2024:rules_damage_and_healing_damage_against_multiple_targets
splat_contracts:
- id: dnd2024:rules_damage_and_healing_damage_against_multiple_targets
  inputs:
    target:
      type: participant
      required: true
    damage:
      required: false
      type: resolution
    event:
      type: event
      required: true
  outputs:
  - hit_point_updates
  - condition_updates
  - death_save_updates
  events: []
  capabilities:
  - damage
  - healing
  - death_saves
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 16
  engine_gaps: &id001
  - dnd2024:rules_damage_and_healing_damage_against_multiple_targets
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_3.md
  specification_sha256: ed3e423511d1beeffc27111a7dd7d35946bc575eb3c662a69f72baf2ce03a70d
  parent_name: Damage and Healing
  line_span:
    first: 1153
    count: 7
splat_clauses:
- key: source_rules
  text: '# Damage against Multiple Targets When you create a damaging effect that forces two or more targets
    to make saving throws against it at the same time, roll the damage once for all the targets. For example,
    when a wizard casts Fireball, the spell’s damage is rolled once for all creatures caught in the blast.'
  sha256: 61fa5e96fb558790a9b865b8453fc77999c2640dc3f9cc66d6ef91d6e66b3b6f
  status: contract_specified
splat_requires_mechanics: *id001
---

# Damage against Multiple Targets

<!-- SRD page 16 -->

When you create a damaging effect that forces two\
or more targets to make saving throws against it at\
the same time, roll the damage once for all the tar-\
gets. For example, when a wizard casts Fireball, the\
spell’s damage is rolled once for all creatures caught\
in the blast.\
