---
splat_schema: 1
splat_id: af35c9a0-4813-5351-87fd-ba4bb06ae5b8
splat_kind: rule
splat_key: dnd2024:class_features/ranger_level_14_nature_s_veil
splat_name: 'Level 14: Nature’s Veil'
splat_origin:
  key: dnd2024:reference/pages/059
splat_sources:
- title: SRD 5.2.1
  locator: pp. 59
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_features/ranger_level_14_nature_s_veil
  parameters:
    action: bonus_action
    condition: invisible
    until: source.next_turn_end
    uses:
      maximum:
        op: max
        args:
        - 1
        - stat: abilities.wisdom.modifier
      recover:
        event: rest.long_rest_completed
        amount: all
  implementation: dnd2024:class_features_ranger_level_14_nature_s_veil
splat_contracts:
- id: dnd2024:class_features_ranger_level_14_nature_s_veil
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
  - conditions
  events:
  - action.bonus
  capabilities:
  - conditions
  - resources
splat_conversion:
  status: specified
  category: class_feature
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 59
  engine_gaps: &id001
  - dnd2024:class_features_ranger_level_14_nature_s_veil
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_1.md
  specification_sha256: a5bf3f08a2b9e8a33d323b0d0a7fbe97e05b3dca068d3e097c95028b2caaf66c
  parent_name: Ranger
  line_span:
    first: 4991
    count: 8
splat_clauses:
- key: source_rules
  text: '# Level 14: Nature’s Veil You invoke spirits of nature to magically hide yourself. As a Bonus
    Action, you can give yourself the Invisible condition until the end of your next turn. You can use
    this feature a number of times equal to your Wisdom modifier (minimum of once), and you regain all
    expended uses when you finish a Long Rest.'
  sha256: bffcacc2dd3851ff8edc22d2d7c7b0e23e75a37ce8a9806f349acaa6bd7285ee
  status: contract_specified
splat_requires_mechanics: *id001
---

# Level 14: Nature’s Veil

<!-- SRD page 59 -->

You invoke spirits of nature to magically hide your-\
self. As a Bonus Action, you can give yourself the\
Invisible condition until the end of your next turn.\
You can use this feature a number of times equal\
to your Wisdom modifier (minimum of once), and\
you regain all expended uses when you finish a\
Long Rest.\
