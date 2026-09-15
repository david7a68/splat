---
splat_schema: 1
splat_id: 95cc7ac0-5cbd-5c9f-b4c3-ef18cdb1b45b
splat_kind: rule
splat_key: dnd2024:rules/parts_of_a_stat_block_saving_throw_effect_notation
splat_name: Saving Throw Effect Notation
splat_origin:
  key: dnd2024:reference/pages/256
splat_sources:
- title: SRD 5.2.1
  locator: pp. 256
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/parts_of_a_stat_block_saving_throw_effect_notation
  parameters:
    save_fields:
    - ability
    - dc
    - target_eligibility
    - failure_effect
    - success_effect
    half_damage_only:
      damage_fraction: 0.5
      rounding: floor
      other_effects_applied: false
  implementation: dnd2024:rule_parts_of_a_stat_block_saving_throw_effect_notation
splat_contracts:
- id: dnd2024:rule_parts_of_a_stat_block_saving_throw_effect_notation
  inputs:
    actor:
      type: participant
      required: true
    context:
      type: resolution
      required: true
  outputs:
  - monster_parameters
  events: []
  capabilities:
  - monsters
  - saving_throws
  - damage
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 256
  engine_gaps: &id001
  - dnd2024:rule_parts_of_a_stat_block_saving_throw_effect_notation
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_2.md
  specification_sha256: b4d0bc25566b4df106a6e22972c42786dad359c7cfa68cf42913283f4708d2af
  parent_name: Parts of a Stat Block
  line_span:
    first: 11148
    count: 10
splat_clauses:
- key: source_rules
  text: '# Saving Throw Effect Notation If an effect forces a saving throw, the effect identifies the
    kind of save required and then provides the save’s DC, a description of which creatures make the save,
    and an explanation of what happens on a failed or successful save. “Half damage only” on a successful
    save means the target takes half as much damage (round down) as targets that fail the save, while
    also ignoring all other parts of the effect.'
  sha256: c37ad19ee811d4986075a7e19f55f0ad7f1347ddc2f1c3df6812ea003c6d3561
  status: contract_specified
splat_requires_mechanics: *id001
---

# Saving Throw Effect Notation

<!-- SRD page 256 -->

If an effect forces a saving throw, the effect identi-\
fies the kind of save required and then provides the\
save’s DC, a description of which creatures make\
the save, and an explanation of what happens on a\
failed or successful save.\
“Half damage only” on a successful save means\
the target takes half as much damage (round down)\
as targets that fail the save, while also ignoring all\
other parts of the effect.\
