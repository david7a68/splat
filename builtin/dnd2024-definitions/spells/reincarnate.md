---
splat_schema: 1
splat_id: 31f53b8a-2be8-58e4-a533-fd0b381f61f3
splat_kind: spell
splat_key: dnd2024:spells/reincarnate
splat_name: Reincarnate
splat_origin:
  key: dnd2024:reference/spell/reincarnate
splat_sources:
- title: SRD 5.2.1
  locator: pp. 158
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 5
  school: necromancy
  classes:
  - druid
  components:
  - verbal
  - somatic
  - material
  concentration: false
  duration: Instantaneous
  casting_time: 1 hour
  ritual: false
  range:
    mode: touch
  material:
    description: rare oils worth 1,000+ GP, which the spell consumes
    consumed: true
splat_rule:
  mechanic: dnd2024:spells/reincarnate
  implementation: dnd2024:spell_reincarnate
  parameters:
    targets:
      kind: dead_humanoid_or_piece
      touched: true
      dead_days_maximum: 10
    revive:
      create_new_body: true
      call_soul_into_body: true
      species_selection:
        methods:
        - roll
        - gm_choose_another_playable_species
        roll:
          dice:
            count: 1
            sides: 10
            add: 0
        faces:
          '1': reroll
          '2': dragonborn
          '3': dwarf
          '4': elf
          '5': gnome
          '6': goliath
          '7': halfling
          '8': human
          '9': orc
          '10': tiefling
      species_options_chooser: reincarnated_creature
      retains:
      - former_life_memories
      - original_capabilities_except_species_traits
      remove: previous_species_traits
      gain: new_species_traits
splat_contracts:
- id: dnd2024:spell_reincarnate
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
      required: false
      type: choice
  outputs:
  - effects
  - health
  events:
  - spell.cast_completed
  - time.elapsed
  capabilities:
  - spellcasting
  - effects
  - dice
  - health
  - forms
splat_conversion:
  status: specified
  category: spell
  source_sha256: d84dbfdca80386a542bab83277cb0de3d47660e58f48a9084582fa9c6ed1948d
  source_path: builtin/srd-5.2.1/spell/reincarnate.md
  pages:
  - 158
  engine_gaps: &id001
  - dnd2024:spell_reincarnate
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_3.md
  specification_sha256: fa9d5e6e0755174c3bf76d8466c0b5abf138298e529904da359c44cc734630bc
splat_clauses:
- key: source_rules
  text: '# Reincarnate Level 5 Necromancy (Druid) Casting Time: 1 hour Range: Touch Components: V, S,
    M (rare oils worth 1,000+ GP, which the spell consumes) Duration: Instantaneous You touch a dead Humanoid
    or a piece of one. If the creature has been dead no longer than 10 days, the spell forms a new body
    for it and calls the soul to enter that body. Roll 1d10 and consult the table below to determine the
    body’s species, or the GM chooses another playable species. 1d10 Species 1d10 Species 1 Roll again.
    6 Goliath 2 Dragonborn 7 Halfling 3 Dwarf 8 Human 4 Elf 9 Orc 5 Gnome 10 Tiefling The reincarnated
    creature makes any choices that a species’ description offers, and the creature recalls its former
    life. It retains the capabilities it had in its original form, except it loses the traits of its previous
    species and gains the traits of its new one.'
  sha256: 25c912b86b8705e6dca46621616b70d1e99343ff728e1445d5b24ff12f10d3de
  status: contract_specified
splat_requires_mechanics: *id001
---

# Reincarnate

<!-- SRD page 158 -->

Level 5 Necromancy (Druid)\
Casting Time: 1 hour\
Range: Touch\
Components: V, S, M (rare oils worth 1,000+ GP, which\
the spell consumes)\
Duration: Instantaneous\
You touch a dead Humanoid or a piece of one. If the\
creature has been dead no longer than 10 days,\
the spell forms a new body for it and calls the soul\
to enter that body. Roll 1d10 and consult the table\
below to determine the body’s species, or the GM\
chooses another playable species.\
1d10 Species 1d10 Species\
1 Roll again. 6 Goliath\
2 Dragonborn 7 Halfling\
3 Dwarf 8 Human\
4 Elf 9 Orc\
5 Gnome 10 Tiefling\
The reincarnated creature makes any choices that\
a species’ description offers, and the creature re-\
calls its former life. It retains the capabilities it had\
in its original form, except it loses the traits of its\
previous species and gains the traits of its new one.\

Original layout: [[../../srd-5.2.1/pages/158|page 158]].
