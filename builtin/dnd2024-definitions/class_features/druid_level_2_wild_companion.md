---
splat_schema: 1
splat_id: 5fb5c6b0-bcad-5fd8-9885-88f250f23e14
splat_kind: rule
splat_key: dnd2024:class_features/druid_level_2_wild_companion
splat_name: 'Level 2: Wild Companion'
splat_origin:
  key: dnd2024:reference/pages/043
splat_sources:
- title: SRD 5.2.1
  locator: pp. 43
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_features/druid_level_2_wild_companion
  parameters:
    action: magic
    cost_choice:
    - resource: spell_slot
      amount: 1
      minimum_level: 1
    - resource: druid.wild_shape
      amount: 1
    cast:
      spell:
        key: dnd2024:spells/find_familiar
      material_components: false
      creature_type: fey
      familiar_ends: actor.rest.long_completed
    class_scope:
      class: druid
      minimum_level: 2
  implementation: dnd2024:class_features_druid_level_2_wild_companion
splat_contracts:
- id: dnd2024:class_features_druid_level_2_wild_companion
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
  - action.magic
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
  - 43
  engine_gaps: &id001
  - dnd2024:class_features_druid_level_2_wild_companion
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_2.md
  specification_sha256: d2d5a7602b6ddb80ae156d7f79d58688e5a7b828e93230340277bfc4dcbb1774
  parent_name: Druid
  line_span:
    first: 3564
    count: 7
splat_clauses:
- key: source_rules
  text: '# Level 2: Wild Companion You can summon a nature spirit that assumes an animal form to aid you.
    As a Magic action, you can expend a spell slot or a use of Wild Shape to cast the Find Familiar spell
    without Material components. When you cast the spell in this way, the familiar is Fey and disappears
    when you finish a Long Rest.'
  sha256: 7e1fd1ad2ba2569aa694e30bd3c945be2c0700f76d8ca8dd8dc2873ec55c5e69
  status: contract_specified
splat_requires_mechanics: *id001
---

# Level 2: Wild Companion

<!-- SRD page 43 -->

You can summon a nature spirit that assumes an\
animal form to aid you. As a Magic action, you can\
expend a spell slot or a use of Wild Shape to cast the\
Find Familiar spell without Material components.\
When you cast the spell in this way, the familiar is\
Fey and disappears when you finish a Long Rest.\
