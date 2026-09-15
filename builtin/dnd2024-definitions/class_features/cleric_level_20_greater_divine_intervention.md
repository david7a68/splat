---
splat_schema: 1
splat_id: 040eb293-0e03-5fc0-a3cd-f1fc83aa3792
splat_kind: rule
splat_key: dnd2024:class_features/cleric_level_20_greater_divine_intervention
splat_name: 'Level 20: Greater Divine Intervention'
splat_origin:
  key: dnd2024:reference/pages/038
splat_sources:
- title: SRD 5.2.1
  locator: pp. 38
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_features/cleric_level_20_greater_divine_intervention
  parameters:
    extends_feature:
      key: dnd2024:class_features/cleric_level_10_divine_intervention
    additional_spell_choice:
      key: dnd2024:spells/wish
    if_chosen:
      divine_intervention_recovery:
        event: rest.long_completed
        required_count:
          dice:
            count: 2
            sides: 4
    class_scope:
      class: cleric
      minimum_level: 20
  implementation: dnd2024:class_features_cleric_level_20_greater_divine_intervention
splat_contracts:
- id: dnd2024:class_features_cleric_level_20_greater_divine_intervention
  inputs:
    actor:
      type: participant
      required: true
    selection:
      required: false
      type: choice
  outputs:
  - effects
  events:
  - feature.divine_intervention
  - rest.long_completed
  capabilities:
  - character_features
  - spellcasting
splat_conversion:
  status: specified
  category: class_feature
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 38
  engine_gaps: &id001
  - dnd2024:class_features_cleric_level_20_greater_divine_intervention
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_2.md
  specification_sha256: 68092b6a167490e19af23a220cd06a9165215faadc1f64cbe11924df51cbbdcd
  parent_name: Cleric
  line_span:
    first: 3142
    count: 6
splat_clauses:
- key: source_rules
  text: '# Level 20: Greater Divine Intervention You can call on even more powerful divine intervention.
    When you use your Divine Intervention feature, you can choose Wish when you select a spell. If you
    do so, you can’t use Divine Intervention again until you finish 2d4 Long Rests.'
  sha256: 70a3217ed98b5cf0545d04a29e90d32cbc2c77e48dc0e1e9f8384f14b3b32d10
  status: contract_specified
splat_requires_mechanics: *id001
---

# Level 20: Greater Divine Intervention

<!-- SRD page 38 -->

You can call on even more powerful divine interven-\
tion. When you use your Divine Intervention fea-\
ture, you can choose Wish when you select a spell.\
If you do so, you can’t use Divine Intervention again\
until you finish 2d4 Long Rests.\
