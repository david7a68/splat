---
splat_schema: 1
splat_id: 0c178b9c-6074-5e7b-aaff-f370eb1331f1
splat_kind: rule
splat_key: dnd2024:rules/gaining_spells_preparing_spells
splat_name: Preparing Spells
splat_origin:
  key: dnd2024:reference/pages/104
splat_sources:
- title: SRD 5.2.1
  locator: pp. 104
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/gaining_spells_preparing_spells
  parameters:
    prepared_spells_minimum_level: 1
    class_replacement_rules:
    - class: bard
      event: level_gained
      maximum_replacements: 1
    - class: cleric
      event: long_rest_finished
      maximum_replacements: any
    - class: druid
      event: long_rest_finished
      maximum_replacements: any
    - class: paladin
      event: long_rest_finished
      maximum_replacements: 1
    - class: ranger
      event: long_rest_finished
      maximum_replacements: 1
    - class: sorcerer
      event: level_gained
      maximum_replacements: 1
    - class: warlock
      event: level_gained
      maximum_replacements: 1
    - class: wizard
      event: long_rest_finished
      maximum_replacements: any
    monster_prepared_list_change:
      default: false
      gm_override: true
  implementation: dnd2024:rules_gaining_spells_preparing_spells
splat_contracts:
- id: dnd2024:rules_gaining_spells_preparing_spells
  inputs:
    caster:
      type: participant
      required: true
    spell:
      type: definition
      required: true
    event:
      type: event
      required: true
    choice:
      type: choice
      required: true
  outputs:
  - spell_state
  - casting_requirements
  events: []
  capabilities:
  - spellcasting
  - concentration
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 104
  engine_gaps: &id001
  - dnd2024:rules_gaining_spells_preparing_spells
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_3.md
  specification_sha256: a881fbc1aec37a1a841109252f3063191b7d46691df2d83a46a9c64524879ccb
  parent_name: Gaining Spells
  line_span:
    first: 9038
    count: 18
splat_clauses:
- key: source_rules
  text: '# Preparing Spells If you have a list of level 1+ spells you prepare, your spellcasting feature
    specifies when you can change the list and the number of spells you can change, as summarized in the
    Spell Preparation by Class table. Spell Preparation by Class Class Change When You Number of Spells
    … Bard Gain a level One Cleric Finish a Long Rest Any Druid Finish a Long Rest Any Paladin Finish
    a Long Rest One Ranger Finish a Long Rest One Sorcerer Gain a level One Warlock Gain a level One Wizard
    Finish a Long Rest Any Most spellcasting monsters don’t change their lists of prepared spells, but
    the GM is free to alter them.'
  sha256: f83802b4d8a7fb6001575114691477158e384f98006e8b1874882e6df0adb88c
  status: contract_specified
splat_requires_mechanics: *id001
---

# Preparing Spells

<!-- SRD page 104 -->

If you have a list of level 1+ spells you prepare, your\
spellcasting feature specifies when you can change\
the list and the number of spells you can change, as\
summarized in the Spell Preparation by Class table.\
Spell Preparation by Class\
Class Change When You Number of Spells\
…\
Bard Gain a level One\
Cleric Finish a Long Rest Any\
Druid Finish a Long Rest Any\
Paladin Finish a Long Rest One\
Ranger Finish a Long Rest One\
Sorcerer Gain a level One\
Warlock Gain a level One\
Wizard Finish a Long Rest Any\
Most spellcasting monsters don’t change their lists\
of prepared spells, but the GM is free to alter them.\
