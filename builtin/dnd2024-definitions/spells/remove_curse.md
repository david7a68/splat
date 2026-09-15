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
splat_conversion:
  status: partial
  category: spell
  source_sha256: 091f0406383f20197fb59c13dc156f37007e9b399f8909d3772f2c0a3a72acf7
  source_path: builtin/srd-5.2.1/spell/remove_curse.md
  pages:
  - 158
  engine_gaps: []
  conversion_gaps:
  - source_rules
  execution_profiles: []
splat_clauses:
- key: source_rules
  text: '# Remove Curse Level 3 Abjuration (Cleric, Paladin, Warlock, Wizard) Casting Time: Action Range:
    Touch Components: V, S Duration: Instantaneous At your touch, all curses affecting one creature or
    object end. If the object is a cursed magic item, its curse remains, but the spell breaks its owner’s
    Attunement to the object so it can be removed or discarded.'
  sha256: 1064dc28a1362f7b90804be471af100e7815f03079d2f7166c0590c78b970904
  status: needs_semantic_conversion
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
