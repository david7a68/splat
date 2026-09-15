---
splat_schema: 1
splat_id: fee498a7-0a7e-59f1-aeae-64dacd463972
splat_kind: spell
splat_key: dnd2024:spells/banishment
splat_name: Banishment
splat_origin:
  key: dnd2024:reference/spell/banishment
splat_sources:
- title: SRD 5.2.1
  locator: pp. 112
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 4
  school: abjuration
  classes:
  - cleric
  - paladin
  - sorcerer
  - warlock
  - wizard
  components:
  - verbal
  - somatic
  - material
  concentration: true
  duration: Concentration, up to 1 minute
  casting_time: Action
  ritual: false
  range:
    feet: 30
  material:
    description: a pentacle
    consumed: false
splat_conversion:
  status: partial
  category: spell
  source_sha256: 54084a29a8c3e8e5533d74d006a5fdab90a23f4b0043dc3d29cbb4d61d1ee2bd
  source_path: builtin/srd-5.2.1/spell/banishment.md
  pages:
  - 112
  engine_gaps: []
  conversion_gaps:
  - source_rules
  execution_profiles: []
splat_clauses:
- key: source_rules
  text: '# Banishment Level 4 Abjuration (Cleric, Paladin, Sorcerer, Warlock, Wizard) Casting Time: Action
    Range: 30 feet Components: V, S, M (a pentacle) Duration: Concentration, up to 1 minute One creature
    that you can see within range must succeed on a Charisma saving throw or be transported to a harmless
    demiplane for the duration. While there, the target has the Incapacitated condition. When the spell
    ends, the target reappears in the space it left or in the nearest unoccupied space if that space is
    occupied. If the target is an Aberration, a Celestial, an Elemental, a Fey, or a Fiend, the target
    doesn’t return if the spell lasts for 1 minute. The target is instead transported to a random location
    on a plane (GM’s choice) associated with its creature type. Using a Higher-Level Spell Slot. You can
    target one additional creature for each spell slot level above 4.'
  sha256: a51d3f1c634c05a6117c8dbb9935cf8e014e20d4bfe8b198c4c36c4ad1d89ed1
  status: needs_semantic_conversion
---

# Banishment

<!-- SRD page 112 -->

Level 4 Abjuration (Cleric, Paladin, Sorcerer, Warlock,\
Wizard)\
Casting Time: Action\
Range: 30 feet\
Components: V, S, M (a pentacle)\
Duration: Concentration, up to 1 minute\
One creature that you can see within range must\
succeed on a Charisma saving throw or be trans-\
ported to a harmless demiplane for the duration.\
While there, the target has the Incapacitated con-\
dition. When the spell ends, the target reappears in\
the space it left or in the nearest unoccupied space\
if that space is occupied.\
If the target is an Aberration, a Celestial, an Ele-\
mental, a Fey, or a Fiend, the target doesn’t return\
if the spell lasts for 1 minute. The target is instead\
transported to a random location on a plane (GM’s\
choice) associated with its creature type.\
Using a Higher-Level Spell Slot. You can target\
one additional creature for each spell slot level\
above 4.\

Original layout: [[../../srd-5.2.1/pages/112|page 112]].
