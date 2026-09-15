---
splat_schema: 1
splat_id: 5ed192df-06c6-57b6-872a-dae32109eb4c
splat_kind: rule
splat_key: dnd2024:feats/fighting_style_feats_great_weapon_fighting
splat_name: Great Weapon Fighting
splat_origin:
  key: dnd2024:reference/pages/088
splat_sources:
- title: SRD 5.2.1
  locator: pp. 88
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:feats/fighting_style_feats_great_weapon_fighting
  parameters:
    feat_category: fighting_style
    prerequisites:
      feature: fighting_style
    requires:
      weapon_category: melee
      held_with_hands: 2
      weapon_property_any:
      - two_handed
      - versatile
    optional_damage_die_floor: 3
    scope: attack_damage_dice
  implementation: dnd2024:feats_fighting_style_feats_great_weapon_fighting
splat_contracts:
- id: dnd2024:feats_fighting_style_feats_great_weapon_fighting
  inputs:
    actor:
      type: participant
      required: true
    event:
      required: false
      type: event
    selection:
      required: false
      type: choice
  outputs:
  - effects
  events:
  - damage.rolled
  capabilities:
  - character_features
  - damage
splat_conversion:
  status: specified
  category: feat
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 88
  engine_gaps: &id001
  - dnd2024:feats_fighting_style_feats_great_weapon_fighting
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_2.md
  specification_sha256: 3cbf9fd9ed6ea902d0165c477129e8e8d35dfa22c684d3e6131a16ce1bee101b
  parent_name: Fighting Style Feats
  line_span:
    first: 7554
    count: 8
splat_clauses:
- key: source_rules
  text: '# Great Weapon Fighting Fighting Style Feat (Prerequisite: Fighting Style Feature) When you roll
    damage for an attack you make with a Melee weapon that you are holding with two hands, you can treat
    any 1 or 2 on a damage die as a 3. The weapon must have the Two-Handed or Versatile property to gain
    this benefit.'
  sha256: 2565f53094b432dd3e8d7368c56aefbaf109c590e430491d495bcb8883bb1161
  status: contract_specified
splat_requires_mechanics: *id001
---

# Great Weapon Fighting

<!-- SRD page 88 -->

Fighting Style Feat (Prerequisite: Fighting Style\
Feature)\
When you roll damage for an attack you make with\
a Melee weapon that you are holding with two\
hands, you can treat any 1 or 2 on a damage die as a\
3. The weapon must have the Two-Handed or Versa-\
tile property to gain this benefit.\
