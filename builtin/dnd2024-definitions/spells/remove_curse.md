---
splat_schema: 1
splat_id: 4d28de7e-cb68-587c-8897-1ccc428b74a1
splat_kind: spell
splat_key: dnd2024:spells/remove_curse
splat_name: Remove Curse
splat_origin:
  key: dnd2024:reference/spell/remove_curse
splat_sources:
- title: SRD 5.2.1
  locator: pp. 158
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 3
  school: abjuration
  classes:
  - cleric
  - paladin
  - warlock
  - wizard
  components:
  - verbal
  - somatic
  concentration: false
  duration: Instantaneous
  casting_time: Action
  ritual: false
  range:
    mode: touch
splat_rule:
  mechanic: dnd2024:spells/remove_curse
  implementation: dnd2024:spell_remove_curse
  parameters:
    spell_lifetime:
      concentration: false
      instantaneous: true
    target:
      kinds:
      - creature
      - object
      touch: true
      count: 1
    normal:
      end: all_curses_affecting_target
    cursed_magic_item_exception:
      item_curse: remains
      owner_attunement: broken
      item_removal_and_discard: permitted
    target_binding:
      exactly_one:
      - target_creature
      - target_object
      must_match_selected_mode: true
splat_contracts:
- id: dnd2024:spell_remove_curse
  inputs:
    caster:
      type: participant
      required: true
    cast_level:
      type: integer
      required: true
    target_creature:
      required: false
      type: participant
    target_object:
      required: false
      type: object
  outputs:
  - effects
  events:
  - spell.cast_completed
  capabilities:
  - spellcasting
  - effects
  - inventory
splat_conversion:
  status: specified
  category: spell
  source_sha256: 091f0406383f20197fb59c13dc156f37007e9b399f8909d3772f2c0a3a72acf7
  source_path: builtin/srd-5.2.1/spell/remove_curse.md
  pages:
  - 158
  engine_gaps: &id001
  - dnd2024:spell_remove_curse
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_4.md
  specification_sha256: e3b08070a520ac1aa354ac13ebe10d2290cf2fee49e5cb21d5e4497593271a8e
splat_clauses:
- key: source_rules
  text: '# Remove Curse Level 3 Abjuration (Cleric, Paladin, Warlock, Wizard) Casting Time: Action Range:
    Touch Components: V, S Duration: Instantaneous At your touch, all curses affecting one creature or
    object end. If the object is a cursed magic item, its curse remains, but the spell breaks its owner’s
    Attunement to the object so it can be removed or discarded.'
  sha256: 1064dc28a1362f7b90804be471af100e7815f03079d2f7166c0590c78b970904
  status: contract_specified
splat_requires_mechanics: *id001
---

# Remove Curse

<!-- SRD page 158 -->

Level 3 Abjuration (Cleric, Paladin, Warlock, Wizard)\
Casting Time: Action\
Range: Touch\
Components: V, S\
Duration: Instantaneous\
At your touch, all curses affecting one creature\
or object end. If the object is a cursed magic item,\
its curse remains, but the spell breaks its owner’s\
Attunement to the object so it can be removed or\
discarded.\

Original layout: [[../../srd-5.2.1/pages/158|page 158]].
