---
splat_schema: 1
splat_id: fa9184fd-e422-593a-9aab-2fa65d6b36e5
splat_kind: rule
splat_key: dnd2024:class_features/paladin_level_5_faithful_steed
splat_name: 'Level 5: Faithful Steed'
splat_origin:
  key: dnd2024:reference/pages/055
splat_sources:
- title: SRD 5.2.1
  locator: pp. 55
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_features/paladin_level_5_faithful_steed
  parameters:
    always_prepared:
    - key: dnd2024:spells/find_steed
    counts_against_prepared_limit: false
    free_cast:
      resource:
        id: paladin.faithful_steed
        maximum: 1
        recovery:
        - event: rest.long_completed
          amount: all
      cost: 1
      expend_slot: false
    class_scope:
      class: paladin
      minimum_level: 5
  implementation: dnd2024:class_features_paladin_level_5_faithful_steed
splat_contracts:
- id: dnd2024:class_features_paladin_level_5_faithful_steed
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
  - prepared_spells
  - resources
  events:
  - spell.cast
  - rest.long_completed
  capabilities:
  - character_features
  - resources
  - spellcasting
splat_conversion:
  status: specified
  category: class_feature
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 55
  engine_gaps: &id001
  - dnd2024:class_features_paladin_level_5_faithful_steed
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_2.md
  specification_sha256: bba9e9d0f72d3b8790e00e774f86c666ce78d70509118e280a9c6b3602584527
  parent_name: Paladin
  line_span:
    first: 4609
    count: 6
splat_clauses:
- key: source_rules
  text: '# Level 5: Faithful Steed You can call on the aid of an otherworldly steed. You always have the
    Find Steed spell prepared. You can also cast the spell once without expending a spell slot, and you
    regain the ability to do so when you finish a Long Rest.'
  sha256: 469dcbf26f2f1060e9240f4a1c396469004cabb4a2f6b25ebb2dbf5134a8fee8
  status: contract_specified
splat_requires_mechanics: *id001
---

# Level 5: Faithful Steed

<!-- SRD page 55 -->

You can call on the aid of an otherworldly steed. You\
always have the Find Steed spell prepared.\
You can also cast the spell once without expend-\
ing a spell slot, and you regain the ability to do so\
when you finish a Long Rest.\
