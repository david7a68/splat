---
splat_schema: 1
splat_id: 889ac3dd-1fb5-5a73-b569-024cc095c8e1
splat_kind: rule
splat_key: dnd2024:feats/fighting_style_feats_two_weapon_fighting
splat_name: Two-Weapon Fighting
splat_origin:
  key: dnd2024:reference/pages/088
splat_sources:
- title: SRD 5.2.1
  locator: pp. 88
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:feats/fighting_style_feats_two_weapon_fighting
  parameters:
    feat_category: fighting_style
    prerequisite_feature: fighting_style
    attack_is_extra_from_light_property: true
    optional_add_ability_modifier_to_damage: true
    unless_modifier_already_added: true
  implementation: dnd2024:feats_fighting_style_feats_two_weapon_fighting
splat_contracts:
- id: dnd2024:feats_fighting_style_feats_two_weapon_fighting
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
  - attack.damage_pending
  capabilities:
  - feats
  - damage
splat_conversion:
  status: specified
  category: feat
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 88
  engine_gaps: &id001
  - dnd2024:feats_fighting_style_feats_two_weapon_fighting
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_1.md
  specification_sha256: c1c8b69d414674fb3d69e6a8a615eca6d8de93bc8d60889d2edc927345b1eec9
  parent_name: Fighting Style Feats
  line_span:
    first: 7562
    count: 7
splat_clauses:
- key: source_rules
  text: '# Two-Weapon Fighting Fighting Style Feat (Prerequisite: Fighting Style Feature) When you make
    an extra attack as a result of using a weapon that has the Light property, you can add your ability
    modifier to the damage of that attack if you aren’t already adding it to the damage.'
  sha256: eeebb6498f123e951633e59d2cfefeb3f210343860ff5f457d1d71d58f4b990e
  status: contract_specified
splat_requires_mechanics: *id001
---

# Two-Weapon Fighting

<!-- SRD page 88 -->

Fighting Style Feat (Prerequisite: Fighting Style\
Feature)\
When you make an extra attack as a result of using\
a weapon that has the Light property, you can add\
your ability modifier to the damage of that attack if\
you aren’t already adding it to the damage.\
