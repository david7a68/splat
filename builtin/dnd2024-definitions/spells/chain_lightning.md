---
splat_schema: 1
splat_id: f2b3e04b-6619-5597-a715-8a7219204e35
splat_kind: spell
splat_key: dnd2024:spells/chain_lightning
splat_name: Chain Lightning
splat_origin:
  key: dnd2024:reference/spell/chain_lightning
splat_sources:
- title: SRD 5.2.1
  locator: pp. 114
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 6
  school: evocation
  classes:
  - sorcerer
  - wizard
  components:
  - verbal
  - somatic
  - material
  concentration: false
  duration: Instantaneous
  casting_time: Action
  ritual: false
  range:
    feet: 150
  material:
    description: three silver pins
    consumed: false
splat_conversion:
  status: partial
  category: spell
  source_sha256: b90daac9573027110bf3df6ace0d1c7f91c7c226109430d86c018bf1bbfc8bec
  source_path: builtin/srd-5.2.1/spell/chain_lightning.md
  pages:
  - 114
  engine_gaps: []
  conversion_gaps:
  - source_rules
  execution_profiles: []
splat_clauses:
- key: source_rules
  text: '# Chain Lightning Level 6 Evocation (Sorcerer, Wizard) Casting Time: Action Range: 150 feet Components:
    V, S, M (three silver pins) Duration: Instantaneous You launch a lightning bolt toward a target you
    can see within range. Three bolts then leap from that target to as many as three other targets of
    your choice, each of which must be within 30 feet of the first target. A target can be a creature
    or an object and can be targeted by only one of the bolts. Each target makes a Dexterity saving throw,
    taking 10d8 Lightning damage on a failed save or half as much damage on a successful one. Using a
    Higher-Level Spell Slot. One additional bolt leaps from the first target to another target for each
    spell slot level above 6.'
  sha256: d2699e63630419cc6473fab2eeb3790756b696b3e5044ccd936c4a65beaf1800
  status: needs_semantic_conversion
---

# Chain Lightning

<!-- SRD page 114 -->

Level 6 Evocation (Sorcerer, Wizard)\
Casting Time: Action\
Range: 150 feet\
Components: V, S, M (three silver pins)\
Duration: Instantaneous\
You launch a lightning bolt toward a target you can\
see within range. Three bolts then leap from that\
target to as many as three other targets of your\
choice, each of which must be within 30 feet of the\
first target. A target can be a creature or an object\
and can be targeted by only one of the bolts.\
Each target makes a Dexterity saving throw, tak-\
ing 10d8 Lightning damage on a failed save or half\
as much damage on a successful one.\
Using a Higher-Level Spell Slot. One additional\
bolt leaps from the first target to another target for\
each spell slot level above 6.\

Original layout: [[../../srd-5.2.1/pages/114|page 114]].
