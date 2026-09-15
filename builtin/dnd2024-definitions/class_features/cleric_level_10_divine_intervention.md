---
splat_schema: 1
splat_id: e2d8b92f-cc35-512f-b6f9-1566b04c9e8c
splat_kind: rule
splat_key: dnd2024:class_features/cleric_level_10_divine_intervention
splat_name: 'Level 10: Divine Intervention'
splat_origin:
  key: dnd2024:reference/pages/038
splat_sources:
- title: SRD 5.2.1
  locator: pp. 38
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_features/cleric_level_10_divine_intervention
  parameters:
    action: magic
    spell_choice:
      list: cleric
      maximum_level: 5
      exclude_casting_time: reaction
    cast_as_part_of_action: true
    spell_slot_cost: 0
    material_components_required: false
    uses:
      maximum: 1
      recover:
        event: rest.long_rest_completed
        amount: all
  implementation: dnd2024:class_features_cleric_level_10_divine_intervention
splat_contracts:
- id: dnd2024:class_features_cleric_level_10_divine_intervention
  inputs:
    actor:
      type: participant
      required: true
  outputs:
  - feature_state
  - resource_changes
  - spell_grants
  events:
  - action.magic
  capabilities:
  - spellcasting
  - resources
splat_conversion:
  status: specified
  category: class_feature
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 38
  engine_gaps: &id001
  - dnd2024:class_features_cleric_level_10_divine_intervention
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_1.md
  specification_sha256: c09325ba9211e0f5b6163e098f317fa497befbe900e33c93d602a44be92bd513
  parent_name: Cleric
  line_span:
    first: 3120
    count: 8
splat_clauses:
- key: source_rules
  text: '# Level 10: Divine Intervention You can call on your deity or pantheon to intervene on your behalf.
    As a Magic action, choose any Cleric spell of level 5 or lower that doesn’t require a Reaction to
    cast. As part of the same action, you cast that spell without expending a spell slot or needing Material
    components. You can’t use this feature again until you finish a Long Rest.'
  sha256: c73c61371986da923ac034f68737d10f09ac253fd1bce96c04a6440751c77ecb
  status: contract_specified
splat_requires_mechanics: *id001
---

# Level 10: Divine Intervention

<!-- SRD page 38 -->

You can call on your deity or pantheon to inter-\
vene on your behalf. As a Magic action, choose any\
Cleric spell of level 5 or lower that doesn’t require a\
Reaction to cast. As part of the same action, you cast\
that spell without expending a spell slot or needing\
Material components. You can’t use this feature\
again until you finish a Long Rest.\
