---
splat_schema: 1
splat_id: 45a5fea8-8629-5288-ae05-7130668fe6d0
splat_kind: rule
splat_key: dnd2024:class_features/monk_level_13_deflect_energy
splat_name: 'Level 13: Deflect Energy'
splat_origin:
  key: dnd2024:reference/pages/052
splat_sources:
- title: SRD 5.2.1
  locator: pp. 52
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_features/monk_level_13_deflect_energy
  parameters:
    feature:
      key: dnd2024:class_features/monk_level_3_deflect_attacks
    replace_eligible_attack_damage_types: all
  implementation: dnd2024:class_features_monk_level_13_deflect_energy
splat_contracts:
- id: dnd2024:class_features_monk_level_13_deflect_energy
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
  - attack.hit
  capabilities:
  - damage
  - reactions
splat_conversion:
  status: specified
  category: class_feature
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 52
  engine_gaps: &id001
  - dnd2024:class_features_monk_level_13_deflect_energy
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_1.md
  specification_sha256: f6c510e5b7934d58a54da8602abf36f8538abb755b2f8f70dc65a6e2c995e57b
  parent_name: Monk
  line_span:
    first: 4322
    count: 4
splat_clauses:
- key: source_rules
  text: '# Level 13: Deflect Energy You can now use your Deflect Attacks feature against attacks that
    deal any damage type, not just Bludgeoning, Piercing, or Slashing.'
  sha256: 97276d81ef7a01d030cf3c621e7f16adfc5771ad4c4cdf84b4cdf82c06849fce
  status: contract_specified
splat_requires_mechanics: *id001
---

# Level 13: Deflect Energy

<!-- SRD page 52 -->

You can now use your Deflect Attacks feature\
against attacks that deal any damage type, not just\
Bludgeoning, Piercing, or Slashing.\
