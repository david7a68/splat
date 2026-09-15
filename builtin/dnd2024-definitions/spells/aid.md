---
splat_schema: 1
splat_id: 3a6f1cf5-22be-59da-82ea-0c3bdcb50da5
splat_kind: spell
splat_key: dnd2024:spells/aid
splat_name: Aid
splat_origin:
  key: dnd2024:reference/spell/aid
splat_sources:
- title: SRD 5.2.1
  locator: pp. 107
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 2
  school: abjuration
  classes:
  - bard
  - cleric
  - druid
  - paladin
  - ranger
  components:
  - verbal
  - somatic
  - material
  concentration: false
  duration: 8 hours
  casting_time: Action
  ritual: false
  range:
    feet: 30
  material:
    description: a strip of white cloth
    consumed: false
splat_conversion:
  status: partial
  category: spell
  source_sha256: 27aad60223873b3231860e3e6d02a3eeaf384657afcc688462bcee0b7ccbcfaa
  source_path: builtin/srd-5.2.1/spell/aid.md
  pages:
  - 107
  engine_gaps: []
  conversion_gaps:
  - source_rules
  execution_profiles: []
splat_clauses:
- key: source_rules
  text: '# Aid Level 2 Abjuration (Bard, Cleric, Druid, Paladin, Ranger) Casting Time: Action Range: 30
    feet Components: V, S, M (a strip of white cloth) Duration: 8 hours Choose up to three creatures within
    range. Each target’s Hit Point maximum and current Hit Points increase by 5 for the duration. Using
    a Higher-Level Spell Slot. Each target’s Hit Points increase by 5 for each spell slot level above
    2.'
  sha256: 14735657e65cb0a898b4a5469896aaffbf2f07c380cb9d861ce5624eeccf8cc3
  status: needs_semantic_conversion
---

# Aid

<!-- SRD page 107 -->

Level 2 Abjuration (Bard, Cleric, Druid, Paladin,\
Ranger)\
Casting Time: Action\
Range: 30 feet\
Components: V, S, M (a strip of white cloth)\
Duration: 8 hours\
Choose up to three creatures within range. Each\
target’s Hit Point maximum and current Hit Points\
increase by 5 for the duration.\
Using a Higher-Level Spell Slot. Each target’s Hit\
Points increase by 5 for each spell slot level above 2.\

Original layout: [[../../srd-5.2.1/pages/107|page 107]].
