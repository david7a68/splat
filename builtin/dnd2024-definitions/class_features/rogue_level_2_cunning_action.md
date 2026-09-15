---
splat_schema: 1
splat_id: 6ead4dce-099b-5206-83f0-e8fe0deb917e
splat_kind: rule
splat_key: dnd2024:class_features/rogue_level_2_cunning_action
splat_name: 'Level 2: Cunning Action'
splat_origin:
  key: dnd2024:reference/pages/062
splat_sources:
- title: SRD 5.2.1
  locator: pp. 62
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_features/rogue_level_2_cunning_action
  parameters:
    turn: own
    action: bonus_action
    choose_action:
    - dash
    - disengage
    - hide
  implementation: dnd2024:class_features_rogue_level_2_cunning_action
splat_contracts:
- id: dnd2024:class_features_rogue_level_2_cunning_action
  inputs:
    actor:
      type: participant
      required: true
  outputs:
  - feature_state
  events:
  - action.bonus
  capabilities:
  - action_economy
splat_conversion:
  status: specified
  category: class_feature
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 62
  engine_gaps: &id001
  - dnd2024:class_features_rogue_level_2_cunning_action
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_1.md
  specification_sha256: 99e1e673effa465aaf9e7aaffd58587c60647533b6404017c909fe9e28cf6991
  parent_name: Rogue
  line_span:
    first: 5244
    count: 5
splat_clauses:
- key: source_rules
  text: '# Level 2: Cunning Action Your quick thinking and agility allow you to move and act quickly.
    On your turn, you can take one of the following actions as a Bonus Action: Dash, Disengage, or Hide.'
  sha256: 2ffa58ebedee1460ce410e570c0c5a1d653bf1340f4e35a4f3a540bb7df7fd36
  status: contract_specified
splat_requires_mechanics: *id001
---

# Level 2: Cunning Action

<!-- SRD page 62 -->

Your quick thinking and agility allow you to move\
and act quickly. On your turn, you can take one of\
the following actions as a Bonus Action: Dash, Dis-\
engage, or Hide.\
