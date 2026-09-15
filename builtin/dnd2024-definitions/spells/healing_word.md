---
splat_schema: 1
splat_id: 2108cd7b-cda7-54b0-94f1-220ff3ac75cc
splat_kind: spell
splat_key: dnd2024:spells/healing_word
splat_name: Healing Word
splat_origin:
  key: dnd2024:reference/spell/healing_word
splat_sources:
- title: SRD 5.2.1
  locator: pp. 139
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 1
  school: abjuration
  classes:
  - bard
  - cleric
  - druid
  components:
  - verbal
  concentration: false
  duration: Instantaneous
  casting_time: Bonus Action
  ritual: false
  range:
    feet: 60
splat_conversion:
  status: partial
  category: spell
  source_sha256: 31ccd8b23546e1746e582dd132c98a086e7e26e2442a7bf07afb4bb67d74faff
  source_path: builtin/srd-5.2.1/spell/healing_word.md
  pages:
  - 139
  engine_gaps: []
  conversion_gaps:
  - source_rules
  execution_profiles:
  - dnd2024:spells/healing_word
splat_clauses:
- key: source_rules
  text: '# Healing Word Level 1 Abjuration (Bard, Cleric, Druid) Casting Time: Bonus Action Range: 60
    feet Components: V Duration: Instantaneous A creature of your choice that you can see within range
    regains Hit Points equal to 2d4 plus your spellcasting ability modifier. Using a Higher-Level Spell
    Slot. The healing increases by 2d4 for each spell slot level above 1.'
  sha256: ed1f906fe98f1c0795ca410b04e6e0c66b4f399c91442b330e66edaab549edee
  status: needs_semantic_conversion
---

# Healing Word

<!-- SRD page 139 -->

Level 1 Abjuration (Bard, Cleric, Druid)\
Casting Time: Bonus Action\
Range: 60 feet\
Components: V\
Duration: Instantaneous\
A creature of your choice that you can see within\
range regains Hit Points equal to 2d4 plus your\
spellcasting ability modifier.\
Using a Higher-Level Spell Slot. The healing in-\
creases by 2d4 for each spell slot level above 1.\

Original layout: [[../../srd-5.2.1/pages/139|page 139]].
