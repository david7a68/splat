---
splat_schema: 1
splat_id: 59dbb391-bf1e-5619-ae77-a55b240910d8
splat_kind: rule
splat_key: dnd2024:class_features/paladin_level_11_radiant_strikes
splat_name: 'Level 11: Radiant Strikes'
splat_origin:
  key: dnd2024:reference/pages/055
splat_sources:
- title: SRD 5.2.1
  locator: pp. 55
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_features/paladin_level_11_radiant_strikes
  parameters:
    trigger: actor.attack_hits_target
    attack_sources:
    - melee_weapon
    - unarmed_strike
    extra_damage:
      dice:
        count: 1
        sides: 8
      type: radiant
    class_scope:
      class: paladin
      minimum_level: 11
  implementation: dnd2024:class_features_paladin_level_11_radiant_strikes
splat_contracts:
- id: dnd2024:class_features_paladin_level_11_radiant_strikes
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
  - attack.hit
  capabilities:
  - character_features
  - damage
splat_conversion:
  status: specified
  category: class_feature
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 55
  engine_gaps: &id001
  - dnd2024:class_features_paladin_level_11_radiant_strikes
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_2.md
  specification_sha256: 0f7662f21672da2248113c456f423fd16b43bd3588d75cfa01f9b3a3f892f1ff
  parent_name: Paladin
  line_span:
    first: 4642
    count: 5
splat_clauses:
- key: source_rules
  text: '# Level 11: Radiant Strikes Your strikes now carry supernatural power. When you hit a target
    with an attack roll using a Melee weapon or an Unarmed Strike, the target takes an extra 1d8 Radiant
    damage.'
  sha256: 32be712ab1786815ce33047e17db67aa1d1b2c2accaf431b9b7f7ca1c1464817
  status: contract_specified
splat_requires_mechanics: *id001
---

# Level 11: Radiant Strikes

<!-- SRD page 55 -->

Your strikes now carry supernatural power. When\
you hit a target with an attack roll using a Melee\
weapon or an Unarmed Strike, the target takes an\
extra 1d8 Radiant damage.\
