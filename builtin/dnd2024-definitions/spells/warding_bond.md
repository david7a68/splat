---
splat_schema: 1
splat_id: 4a37cfcf-025d-51f5-9a98-be41338641fe
splat_kind: spell
splat_key: dnd2024:spells/warding_bond
splat_name: Warding Bond
splat_origin:
  key: dnd2024:reference/spell/warding_bond
splat_sources:
- title: SRD 5.2.1
  locator: pp. 173
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 2
  school: abjuration
  classes:
  - cleric
  - paladin
  components:
  - verbal
  - somatic
  - material
  concentration: false
  duration: 1 hour
  casting_time: Action
  ritual: false
  range:
    mode: touch
  material:
    description: a pair of platinum rings worth 50+ GP each, which you and the target must wear for the
      duration
    consumed: false
splat_conversion:
  status: partial
  category: spell
  source_sha256: 23cf3b80dbe2d36f17a68c90594aeb9822e612b47cf77a9208b8cadbd8e101da
  source_path: builtin/srd-5.2.1/spell/warding_bond.md
  pages:
  - 173
  engine_gaps: []
  conversion_gaps:
  - source_rules
  execution_profiles: []
splat_clauses:
- key: source_rules
  text: '# Warding Bond Level 2 Abjuration (Cleric, Paladin) Casting Time: Action Range: Touch Components:
    V, S, M (a pair of platinum rings worth 50+ GP each, which you and the target must wear for the duration)
    Duration: 1 hour You touch another creature that is willing and create a mystic connection between
    you and the target until the spell ends. While the target is within 60 feet of you, it gains a +1
    bonus to AC and saving throws, and it has Resistance to all damage. Also, each time it takes damage,
    you take the same amount of damage. The spell ends if you drop to 0 Hit Points or if you and the target
    become separated by more than 60 feet. It also ends if the spell is cast again on either of the connected
    creatures.'
  sha256: 9c7921623fa12be3a5392d87c642a3ffcc576b4a1308d1d229636258a8ed94fe
  status: needs_semantic_conversion
---

# Warding Bond

<!-- SRD page 173 -->

Level 2 Abjuration (Cleric, Paladin)\
Casting Time: Action\
Range: Touch\
Components: V, S, M (a pair of platinum rings worth\
50+ GP each, which you and the target must wear for\
the duration)\
Duration: 1 hour\
You touch another creature that is willing and\
create a mystic connection between you and the\
target until the spell ends. While the target is\
within 60 feet of you, it gains a +1 bonus to AC and\
saving throws, and it has Resistance to all damage.\
Also, each time it takes damage, you take the same\
amount of damage.\
The spell ends if you drop to 0 Hit Points or if you\
and the target become separated by more than 60\
feet. It also ends if the spell is cast again on either of\
the connected creatures.\

Original layout: [[../../srd-5.2.1/pages/173|page 173]].
