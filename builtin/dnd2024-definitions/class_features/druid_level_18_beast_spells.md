---
splat_schema: 1
splat_id: 0616e899-1014-5aea-9796-b3832feee43f
splat_kind: rule
splat_key: dnd2024:class_features/druid_level_18_beast_spells
splat_name: 'Level 18: Beast Spells'
splat_origin:
  key: dnd2024:reference/pages/043
splat_sources:
- title: SRD 5.2.1
  locator: pp. 43
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_features/druid_level_18_beast_spells
  parameters:
    while: wild_shape.beast_form
    allow_spellcasting: true
    exclude_spells:
      material_component_cost_specified: true
      or_material_component_consumed: true
    class_scope:
      class: druid
      minimum_level: 18
  implementation: dnd2024:class_features_druid_level_18_beast_spells
splat_contracts:
- id: dnd2024:class_features_druid_level_18_beast_spells
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
  - spell.cast_pending
  capabilities:
  - character_features
  - spellcasting
splat_conversion:
  status: specified
  category: class_feature
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 43
  engine_gaps: &id001
  - dnd2024:class_features_druid_level_18_beast_spells
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_2.md
  specification_sha256: bea8965282c0cca089259f241b8d3ce1420ffccc8a4b0db9ee9ffc9f7a6f7e82
  parent_name: Druid
  line_span:
    first: 3610
    count: 5
splat_clauses:
- key: source_rules
  text: '# Level 18: Beast Spells While using Wild Shape, you can cast spells in Beast form, except for
    any spell that has a Material component with a cost specified or that consumes its Material component.'
  sha256: dae1e40cf5c8c7fa685252111d5433114f2f53e726495905eb12d05152af8c2b
  status: contract_specified
splat_requires_mechanics: *id001
---

# Level 18: Beast Spells

<!-- SRD page 43 -->

While using Wild Shape, you can cast spells in Beast\
form, except for any spell that has a Material com-\
ponent with a cost specified or that consumes its\
Material component.\
