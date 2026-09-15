---
splat_schema: 1
splat_id: 8d2df17b-dfa9-5c3e-9f69-1436ccfb79a0
splat_kind: rule
splat_key: dnd2024:class_features/rogue_level_7_evasion
splat_name: 'Level 7: Evasion'
splat_origin:
  key: dnd2024:reference/pages/063
splat_sources:
- title: SRD 5.2.1
  locator: pp. 63
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_features/rogue_level_7_evasion
  parameters:
    when:
      saving_throw_ability: dexterity
      normal_success_damage_multiplier: 0.5
      incapacitated: false
    damage_multipliers:
      success: 0
      failure: 0.5
  implementation: dnd2024:class_features_rogue_level_7_evasion
splat_contracts:
- id: dnd2024:class_features_rogue_level_7_evasion
  inputs:
    actor:
      type: participant
      required: true
    trigger:
      required: false
      type: event
    targets:
      required: false
      type: participant_list
  outputs:
  - feature_state
  - damage
  events:
  - saving_throw.resolved
  capabilities:
  - damage
  - saving_throws
splat_conversion:
  status: specified
  category: class_feature
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 63
  engine_gaps: &id001
  - dnd2024:class_features_rogue_level_7_evasion
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_1.md
  specification_sha256: d316ae3f123b2523242c0d9717ec3623620c29375ff161e6824ee13ff4460cda
  parent_name: Rogue
  line_span:
    first: 5299
    count: 8
splat_clauses:
- key: source_rules
  text: '# Level 7: Evasion You can nimbly dodge out of the way of certain dangers. When you’re subjected
    to an effect that allows you to make a Dexterity saving throw to take only half damage, you instead
    take no damage if you succeed on the saving throw and only half damage if you fail. You can’t use
    this feature if you have the Incapacitated condition.'
  sha256: a99f666db32e8fb21711cc4320b64147d60ad8d059da74ef543f6343225b217e
  status: contract_specified
splat_requires_mechanics: *id001
---

# Level 7: Evasion

<!-- SRD page 63 -->

You can nimbly dodge out of the way of certain dan-\
gers. When you’re subjected to an effect that allows\
you to make a Dexterity saving throw to take only\
half damage, you instead take no damage if you\
succeed on the saving throw and only half damage\
if you fail. You can’t use this feature if you have the\
Incapacitated condition.\
