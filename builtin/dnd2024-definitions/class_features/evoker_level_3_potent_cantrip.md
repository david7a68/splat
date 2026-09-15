---
splat_schema: 1
splat_id: 3b4cebb3-bef4-5750-b085-0047a21dc7f1
splat_kind: rule
splat_key: dnd2024:class_features/evoker_level_3_potent_cantrip
splat_name: 'Level 3: Potent Cantrip'
splat_origin:
  key: dnd2024:reference/pages/082
splat_sources:
- title: SRD 5.2.1
  locator: pp. 82
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_features/evoker_level_3_potent_cantrip
  parameters:
    spell_level: 0
    target_type: creature
    triggers:
    - cantrip_attack_misses
    - target_succeeds_cantrip_save
    damage_multiplier: 0.5
    additional_cantrip_effects: false
    requires_cantrip_damage: true
  implementation: dnd2024:class_features_evoker_level_3_potent_cantrip
splat_contracts:
- id: dnd2024:class_features_evoker_level_3_potent_cantrip
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
  - spell.attack_missed
  - spell.save_succeeded
  capabilities:
  - damage
splat_conversion:
  status: specified
  category: class_feature
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 82
  engine_gaps: &id001
  - dnd2024:class_features_evoker_level_3_potent_cantrip
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_1.md
  specification_sha256: 438462fb53ac5024fc0d7a860cfa51e2962210c60127286762f5d12565cc5163
  parent_name: Evoker
  line_span:
    first: 7025
    count: 8
splat_clauses:
- key: source_rules
  text: '# Level 3: Potent Cantrip Your damaging cantrips affect even creatures that avoid the brunt of
    the effect. When you cast a cantrip at a creature and you miss with the attack roll or the target
    succeeds on a saving throw against the cantrip, the target takes half the cantrip’s damage (if any)
    but suffers no additional effect from the cantrip.'
  sha256: f5ee2e4e8c7955babb5a1ffd549434319e17115c1ac21e824848a7a5d65edd2c
  status: contract_specified
splat_requires_mechanics: *id001
---

# Level 3: Potent Cantrip

<!-- SRD page 82 -->

Your damaging cantrips affect even creatures that\
avoid the brunt of the effect. When you cast a can-\
trip at a creature and you miss with the attack roll\
or the target succeeds on a saving throw against the\
cantrip, the target takes half the cantrip’s damage\
(if any) but suffers no additional effect from the\
cantrip.\
