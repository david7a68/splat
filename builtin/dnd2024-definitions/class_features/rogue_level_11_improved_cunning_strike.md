---
splat_schema: 1
splat_id: 4336c31c-9da8-5da5-81ca-ef13a7d6f75e
splat_kind: rule
splat_key: dnd2024:class_features/rogue_level_11_improved_cunning_strike
splat_name: 'Level 11: Improved Cunning Strike'
splat_origin:
  key: dnd2024:reference/pages/063
splat_sources:
- title: SRD 5.2.1
  locator: pp. 63
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_features/rogue_level_11_improved_cunning_strike
  parameters:
    feature: cunning_strike
    replace_maximum_effect_choices: 2
    pay_each_effect_die_cost: true
  implementation: dnd2024:class_features_rogue_level_11_improved_cunning_strike
splat_contracts:
- id: dnd2024:class_features_rogue_level_11_improved_cunning_strike
  inputs:
    actor:
      type: participant
      required: true
  outputs:
  - feature_state
  events:
  - sneak_attack.damage_pending
  capabilities:
  - damage_dice_costs
splat_conversion:
  status: specified
  category: class_feature
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 63
  engine_gaps: &id001
  - dnd2024:class_features_rogue_level_11_improved_cunning_strike
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_1.md
  specification_sha256: fd5d614099ccc55c195cc95d05cd8f63d11fdf25b183c8b7b9b1ba424bf7c348
  parent_name: Rogue
  line_span:
    first: 5311
    count: 4
splat_clauses:
- key: source_rules
  text: '# Level 11: Improved Cunning Strike You can use up to two Cunning Strike effects when you deal
    Sneak Attack damage, paying the die cost for each effect.'
  sha256: 542ebbe4e939113ff24ef442729202af81ba148045007da18cf74447d689c1a9
  status: contract_specified
splat_requires_mechanics: *id001
---

# Level 11: Improved Cunning Strike

<!-- SRD page 63 -->

You can use up to two Cunning Strike effects when\
you deal Sneak Attack damage, paying the die cost\
for each effect.\
