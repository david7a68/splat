---
splat_schema: 1
splat_id: 1c902810-86c2-5b9c-a873-5706d7ebec05
splat_kind: rule
splat_key: dnd2024:class_features/bard_level_7_countercharm
splat_name: 'Level 7: Countercharm'
splat_origin:
  key: dnd2024:reference/pages/033
splat_sources:
- title: SRD 5.2.1
  locator: pp. 33
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_features/bard_level_7_countercharm
  parameters:
    trigger:
      saving_throw: failed
      effect_applies_any_condition:
      - charmed
      - frightened
      target: self_or_creature
      target_range_feet: 30
    action: reaction
    reroll:
      advantage: true
  implementation: dnd2024:class_features_bard_level_7_countercharm
splat_contracts:
- id: dnd2024:class_features_bard_level_7_countercharm
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
  events:
  - saving_throw.failed
  capabilities:
  - saving_throws
  - reactions
splat_conversion:
  status: specified
  category: class_feature
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 33
  engine_gaps: &id001
  - dnd2024:class_features_bard_level_7_countercharm
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_1.md
  specification_sha256: 2c679b26d651e4ca5bf8e473a2ed79f2bf3917bf65b804434aa9b27418fa7762
  parent_name: Bard
  line_span:
    first: 2680
    count: 7
splat_clauses:
- key: source_rules
  text: '# Level 7: Countercharm You can use musical notes or words of power to disrupt mind-influencing
    effects. If you or a creature within 30 feet of you fails a saving throw against an effect that applies
    the Charmed or Frightened condition, you can take a Reaction to cause the save to be rerolled, and
    the new roll has Advantage.'
  sha256: c7b2f67f70361b496b8d9c743feb98ed82a4f94bf5ddf1006ad39530b300f115
  status: contract_specified
splat_requires_mechanics: *id001
---

# Level 7: Countercharm

<!-- SRD page 33 -->

You can use musical notes or words of power to dis-\
rupt mind-influencing effects. If you or a creature\
within 30 feet of you fails a saving throw against an\
effect that applies the Charmed or Frightened con-\
dition, you can take a Reaction to cause the save to\
be rerolled, and the new roll has Advantage.\
