---
splat_schema: 1
splat_id: 01599a29-8e25-5f64-8700-c40a986f7395
splat_kind: rule
splat_key: dnd2024:class_features/monk_level_2_uncanny_metabolism
splat_name: 'Level 2: Uncanny Metabolism'
splat_origin:
  key: dnd2024:reference/pages/051
splat_sources:
- title: SRD 5.2.1
  locator: pp. 51
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_features/monk_level_2_uncanny_metabolism
  parameters:
    optional: true
    recover:
      resource: focus_points
      amount: all_expended
    heal_self:
      op: add
      args:
      - stat: classes.monk.level
      - dice:
          count: 1
          sides:
            stat: martial_arts_die.sides
    uses:
      maximum: 1
      recover:
        event: rest.long_rest_completed
        amount: all
  implementation: dnd2024:class_features_monk_level_2_uncanny_metabolism
splat_contracts:
- id: dnd2024:class_features_monk_level_2_uncanny_metabolism
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
  - resource_changes
  - healing
  events:
  - initiative.rolled
  capabilities:
  - resources
  - healing
splat_conversion:
  status: specified
  category: class_feature
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 51
  engine_gaps: &id001
  - dnd2024:class_features_monk_level_2_uncanny_metabolism
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_1.md
  specification_sha256: 416d965f42be9e9ebe8c9ab3cf62c591487e3d46a91ee39806ebfa6589d480ff
  parent_name: Monk
  line_span:
    first: 4231
    count: 7
splat_clauses:
- key: source_rules
  text: '# Level 2: Uncanny Metabolism When you roll Initiative, you can regain all expended Focus Points.
    When you do so, roll your Martial Arts die, and regain a number of Hit Points equal to your Monk level
    plus the number rolled. Once you use this feature, you can’t use it again until you finish a Long
    Rest.'
  sha256: daf93ac1ee4dc7c8e49d4690282f318d3c2536453b34c527d19962b075ac3eb3
  status: contract_specified
splat_requires_mechanics: *id001
---

# Level 2: Uncanny Metabolism

<!-- SRD page 51 -->

When you roll Initiative, you can regain all ex-\
pended Focus Points. When you do so, roll your\
Martial Arts die, and regain a number of Hit Points\
equal to your Monk level plus the number rolled.\
Once you use this feature, you can’t use it again\
until you finish a Long Rest.\
