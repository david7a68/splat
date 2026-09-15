---
splat_schema: 1
splat_id: 9b2c8581-02be-5564-b8d4-1b6e6abd3e0c
splat_kind: rule
splat_key: dnd2024:class_features/monk_level_14_disciplined_survivor
splat_name: 'Level 14: Disciplined Survivor'
splat_origin:
  key: dnd2024:reference/pages/052
splat_sources:
- title: SRD 5.2.1
  locator: pp. 52
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_features/monk_level_14_disciplined_survivor
  parameters:
    saving_throw_proficiency: all
    failure_reroll:
      optional: true
      cost:
        resource: monk.focus_points
        amount: 1
      must_use_new_roll: true
    class_scope:
      class: monk
      minimum_level: 14
  implementation: dnd2024:class_features_monk_level_14_disciplined_survivor
splat_contracts:
- id: dnd2024:class_features_monk_level_14_disciplined_survivor
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
  - saving_throw.calculate
  - saving_throw.failed
  capabilities:
  - character_features
  - resources
  - proficiencies
splat_conversion:
  status: specified
  category: class_feature
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 52
  engine_gaps: &id001
  - dnd2024:class_features_monk_level_14_disciplined_survivor
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_2.md
  specification_sha256: 8ea7214e4380a8617ebb296890fd1861250b22ed799262e6365a50fa6c9c7798
  parent_name: Monk
  line_span:
    first: 4326
    count: 6
splat_clauses:
- key: source_rules
  text: '# Level 14: Disciplined Survivor Your physical and mental discipline grant you proficiency in
    all saving throws. Additionally, whenever you make a saving throw and fail, you can expend 1 Focus
    Point to reroll it, and you must use the new roll.'
  sha256: 988247bc469a3581c57c6afdb80324a011f58814af7a9787a46233bd8fbfa037
  status: contract_specified
splat_requires_mechanics: *id001
---

# Level 14: Disciplined Survivor

<!-- SRD page 52 -->

Your physical and mental discipline grant you profi-\
ciency in all saving throws.\
Additionally, whenever you make a saving throw\
and fail, you can expend 1 Focus Point to reroll it,\
and you must use the new roll.\
