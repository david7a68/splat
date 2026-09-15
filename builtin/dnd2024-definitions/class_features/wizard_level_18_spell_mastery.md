---
splat_schema: 1
splat_id: 8d2200b6-0421-535c-bf26-c7b29fc62227
splat_kind: rule
splat_key: dnd2024:class_features/wizard_level_18_spell_mastery
splat_name: 'Level 18: Spell Mastery'
splat_origin:
  key: dnd2024:reference/pages/079
splat_sources:
- title: SRD 5.2.1
  locator: pp. 79
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_features/wizard_level_18_spell_mastery
  parameters:
    choose_spells:
    - count: 1
      level: 1
    - count: 1
      level: 2
    eligibility:
      in_spellbook: true
      casting_time: action
    always_prepared: true
    counts_against_prepared_limit: false
    free_cast:
      at_will: true
      cast_level: base_spell_level
      expend_slot: false
    higher_level_casting:
      expend_slot: true
    replace:
      event: rest.long_completed
      requires: study_spellbook
      maximum: 1
      same_level: true
      same_eligibility: true
    class_scope:
      class: wizard
      minimum_level: 18
  implementation: dnd2024:class_features_wizard_level_18_spell_mastery
splat_contracts:
- id: dnd2024:class_features_wizard_level_18_spell_mastery
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
  - character.feature_acquired
  - rest.long_completed
  - spell.cast
  capabilities:
  - character_features
  - spellcasting
splat_conversion:
  status: specified
  category: class_feature
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 79
  engine_gaps: &id001
  - dnd2024:class_features_wizard_level_18_spell_mastery
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_2.md
  specification_sha256: eb785bbe3e08e00464f2d2ba2f44368dc6e5fe6cf240f95018af555e0761766a
  parent_name: Wizard
  line_span:
    first: 6737
    count: 11
splat_clauses:
- key: source_rules
  text: '# Level 18: Spell Mastery You have achieved such mastery over certain spells that you can cast
    them at will. Choose a level 1 and a level 2 spell in your spellbook that have a casting time of an
    action. You always have those spells prepared, and you can cast them at their lowest level without
    expending a spell slot. To cast either spell at a higher level, you must expend a spell slot. Whenever
    you finish a Long Rest, you can study your spellbook and replace one of those spells with an eligible
    spell of the same level from the book.'
  sha256: b78796f181e67ad8031d848f6737b075f876197c16f546c39a2b041d41ac2892
  status: contract_specified
splat_requires_mechanics: *id001
---

# Level 18: Spell Mastery

<!-- SRD page 79 -->

You have achieved such mastery over certain spells\
that you can cast them at will. Choose a level 1 and\
a level 2 spell in your spellbook that have a casting\
time of an action. You always have those spells pre-\
pared, and you can cast them at their lowest level\
without expending a spell slot. To cast either spell at\
a higher level, you must expend a spell slot.\
Whenever you finish a Long Rest, you can study\
your spellbook and replace one of those spells with\
an eligible spell of the same level from the book.\
