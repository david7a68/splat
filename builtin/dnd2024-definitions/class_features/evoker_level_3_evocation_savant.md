---
splat_schema: 1
splat_id: dd2dee6b-6886-5627-8f43-68108c51a0d0
splat_kind: rule
splat_key: dnd2024:class_features/evoker_level_3_evocation_savant
splat_name: 'Level 3: Evocation Savant'
splat_origin:
  key: dnd2024:reference/pages/082
splat_sources:
- title: SRD 5.2.1
  locator: pp. 82
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_features/evoker_level_3_evocation_savant
  parameters:
    initial_spellbook_additions:
      count: 2
      spell_list: wizard
      school: evocation
      minimum_spell_level: 1
      maximum_spell_level: 2
      cost_gp: 0
    subsequent_spellbook_additions:
      event: wizard.new_spell_slot_level_access
      count: 1
      spell_list: wizard
      school: evocation
      requires_available_slot_level: true
      cost_gp: 0
    class_scope:
      class: wizard
      minimum_level: 3
  implementation: dnd2024:class_features_evoker_level_3_evocation_savant
splat_contracts:
- id: dnd2024:class_features_evoker_level_3_evocation_savant
  inputs:
    actor:
      type: participant
      required: true
    selection:
      required: false
      type: choice
  outputs:
  - spellbook_additions
  events:
  - character.feature_acquired
  - character.level_gained
  capabilities:
  - character_features
  - spellcasting
splat_conversion:
  status: specified
  category: class_feature
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 82
  engine_gaps: &id001
  - dnd2024:class_features_evoker_level_3_evocation_savant
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_2.md
  specification_sha256: b37267e7f34c1f151ae43092211fe270d3a0beade11dad78fb7ba1cfa3253212
  parent_name: Evoker
  line_span:
    first: 7016
    count: 9
splat_clauses:
- key: source_rules
  text: '# Level 3: Evocation Savant Choose two Wizard spells from the Evocation school, each of which
    must be no higher than level 2, and add them to your spellbook for free. In addition, whenever you
    gain access to a new level of spell slots in this class, you can add one Wizard spell from the Evocation
    school to your spellbook for free. The chosen spell must be of a level for which you have spell slots.'
  sha256: b49d2f2a3f82e3ede2d531f893f95a87ea8cbeb7fc2fe8b5860b2d46d37ef7b3
  status: contract_specified
splat_requires_mechanics: *id001
---

# Level 3: Evocation Savant

<!-- SRD page 82 -->

Choose two Wizard spells from the Evocation\
school, each of which must be no higher than level 2,\
and add them to your spellbook for free.\
In addition, whenever you gain access to a new\
level of spell slots in this class, you can add one Wiz-\
ard spell from the Evocation school to your spell-\
book for free. The chosen spell must be of a level for\
which you have spell slots.\
