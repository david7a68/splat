---
splat_schema: 1
splat_id: 323e3e17-d106-5b63-80c4-13a44c6f983b
splat_kind: spell
splat_key: dnd2024:spells/mass_healing_word
splat_name: Mass Healing Word
splat_origin:
  key: dnd2024:reference/spell/mass_healing_word
splat_sources:
- title: SRD 5.2.1
  locator: pp. 148
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 3
  school: abjuration
  classes:
  - bard
  - cleric
  components:
  - verbal
  concentration: false
  duration: Instantaneous
  casting_time: Bonus Action
  ritual: false
  range:
    feet: 60
splat_rule:
  mechanic: dnd2024:spells/mass_healing_word
  implementation: dnd2024:spell_mass_healing_word
  parameters:
    targeting:
      kind: creatures
      max_count: 6
      chosen: true
      visible: true
      within_range: true
    healing:
      dice: 2d4
      add:
        stat: caster_spellcasting_ability_modifier
    slot_scaling:
      above: 3
      healing_per_level: 1d4
    spell_lifetime:
      concentration: false
      maximum:
        kind: instantaneous
splat_contracts:
- id: dnd2024:spell_mass_healing_word
  inputs:
    caster:
      type: participant
      required: true
    cast_level:
      type: integer
      required: true
    targets:
      type: participant_list
      required: true
    choices:
      type: choice
      required: true
  outputs:
  - effects
  - healing_results
  events:
  - spell.cast_completed
  capabilities:
  - spellcasting
  - effects
  - healing
splat_conversion:
  status: specified
  category: spell
  source_sha256: f96bc6b3b8f8e9fcb8d58ee9d3ec4cbaa7243cf7931aae76efe51a749a4587cc
  source_path: builtin/srd-5.2.1/spell/mass_healing_word.md
  pages:
  - 148
  engine_gaps: &id001
  - dnd2024:spell_mass_healing_word
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_2.md
  specification_sha256: 038c05c7b4d99116eb6cb67b1a2b3959d0884add9340d129eab4745eeb11bfde
splat_clauses:
- key: source_rules
  text: '# Mass Healing Word Level 3 Abjuration (Bard, Cleric) Casting Time: Bonus Action Range: 60 feet
    Components: V Duration: Instantaneous Up to six creatures of your choice that you can see within range
    regain Hit Points equal to 2d4 plus your spellcasting ability modifier. Using a Higher-Level Spell
    Slot. The healing increases by 1d4 for each spell slot level above 3.'
  sha256: c6faf2406179c77e1025bcb33f0b73c9cb14414546d1f5d9ca8053077ccf75a1
  status: contract_specified
splat_requires_mechanics: *id001
---

# Mass Healing Word

<!-- SRD page 148 -->

Level 3 Abjuration (Bard, Cleric)\
Casting Time: Bonus Action\
Range: 60 feet\
Components: V\
Duration: Instantaneous\
Up to six creatures of your choice that you can see\
within range regain Hit Points equal to 2d4 plus\
your spellcasting ability modifier.\
Using a Higher-Level Spell Slot. The healing in-\
creases by 1d4 for each spell slot level above 3.\

Original layout: [[../../srd-5.2.1/pages/148|page 148]].
