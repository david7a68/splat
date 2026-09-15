---
splat_schema: 1
splat_id: fd9a24ad-a71d-5d25-9333-8fadfd706869
splat_kind: rule
splat_key: dnd2024:rules/casting_spells_spell_slots
splat_name: Spell Slots
splat_origin:
  key: dnd2024:reference/pages/104
splat_sources:
- title: SRD 5.2.1
  locator: pp. 104
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/casting_spells_spell_slots
  parameters:
    spell_slot_required_default_for_spell_levels:
      min: 1
      max: 9
    slot_level_min: spell_level
    slots_expended_per_cast: 1
    slot_capacity_source: class_spellcasting_table
    restore_all_expended_slots: long_rest_finished
    specific_feature_exceptions: true
  implementation: dnd2024:rules_casting_spells_spell_slots
splat_contracts:
- id: dnd2024:rules_casting_spells_spell_slots
  inputs:
    subject:
      type: participant
      required: true
  outputs:
  - effects
  events:
  - spell.cast_pending
  - rest.long_finished
  capabilities:
  - spell_slots
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 104
  engine_gaps: &id001
  - dnd2024:rules_casting_spells_spell_slots
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_1.md
  specification_sha256: 07e8fb93700d597edf470645cc08311bf34800b873fde6b58dbb555bfdaac7fb
  parent_name: Casting Spells
  line_span:
    first: 9078
    count: 19
splat_clauses:
- key: source_rules
  text: '# Spell Slots Spellcasting is taxing, so a spellcaster can cast only a limited number of level
    1+ spells before resting. Spell slots are the main way a spellcaster’s magical potential is represented.
    Each spellcasting class gives its members a limited number of spell slots of certain spell levels.
    For example, a level 3 Wizard has four level 1 spell slots and two level 2 slots. When you cast a
    spell, you expend a slot of that spell’s level or higher, effectively “filling” a slot with the spell.
    Imagine a spell slot is a groove of a certain size—small for a level 1 slot and larger for a higher-level
    spell. A level 1 spell fits into a slot of any size, but a level 2 spell fits only into a slot that’s
    at least level 2. So when a level 3 Wizard casts Magic Missile, a level 1 spell, that Wizard spends
    one of four level 1 slots and has three remaining. Finishing a Long Rest restores any expended spell
    slots.'
  sha256: 0d876e3d05c72b5e53c5f6c4cf4ec72644e496822c8da7d6ab59a1902398a7c5
  status: contract_specified
splat_requires_mechanics: *id001
---

# Spell Slots

<!-- SRD page 104 -->

Spellcasting is taxing, so a spellcaster can cast only\
a limited number of level 1+ spells before resting.\
Spell slots are the main way a spellcaster’s magical\
potential is represented. Each spellcasting class\
gives its members a limited number of spell slots of\
certain spell levels. For example, a level 3 Wizard\
has four level 1 spell slots and two level 2 slots.\
When you cast a spell, you expend a slot of that\
spell’s level or higher, effectively “filling” a slot with\
the spell. Imagine a spell slot is a groove of a certain\
size—small for a level 1 slot and larger for a high-\
er-level spell. A level 1 spell fits into a slot of any\
size, but a level 2 spell fits only into a slot that’s at\
least level 2. So when a level 3 Wizard casts Magic\
Missile, a level 1 spell, that Wizard spends one of\
four level 1 slots and has three remaining.\
Finishing a Long Rest restores any expended spell\
slots.\
