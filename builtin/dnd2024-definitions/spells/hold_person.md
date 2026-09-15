---
splat_schema: 1
splat_id: ca888eaa-780b-5c8c-95ae-229ca74c404a
splat_kind: spell
splat_key: dnd2024:spells/hold_person
splat_name: Hold Person
splat_origin:
  key: dnd2024:reference/spell/hold_person
splat_sources:
- title: SRD 5.2.1
  locator: pp. 141
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 2
  school: enchantment
  classes:
  - bard
  - cleric
  - druid
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
    feet: 60
  material:
    description: a straight piece of iron
    consumed: false
splat_conversion:
  status: partial
  category: spell
  source_sha256: f3d0b49e8296e77f801eb3a53a6080b7971575b56235ec0106030ba780379652
  source_path: builtin/srd-5.2.1/spell/hold_person.md
  pages:
  - 141
  engine_gaps: []
  conversion_gaps:
  - source_rules
  execution_profiles: []
splat_clauses:
- key: source_rules
  text: '# Hold Person Level 2 Enchantment (Bard, Cleric, Druid, Sorcerer, Warlock, Wizard) Casting Time:
    Action Range: 60 feet Components: V, S, M (a straight piece of iron) Duration: Concentration, up to
    1 minute Choose a Humanoid that you can see within range. The target must succeed on a Wisdom saving
    throw or have the Paralyzed condition for the duration. At the end of each of its turns, the target
    repeats the save, ending the spell on itself on a success. Using a Higher-Level Spell Slot. You can
    target one additional Humanoid for each spell slot level above 2.'
  sha256: 02d56eb7cda1ef11211ad0509376bb0b610d8b3c25a689809d44769b4ec7472f
  status: needs_semantic_conversion
---

# Hold Person

<!-- SRD page 141 -->

Level 2 Enchantment (Bard, Cleric, Druid, Sorcerer,\
Warlock, Wizard)\
Casting Time: Action\
Range: 60 feet\
Components: V, S, M (a straight piece of iron)\
Duration: Concentration, up to 1 minute\
Choose a Humanoid that you can see within range.\
The target must succeed on a Wisdom saving throw\
or have the Paralyzed condition for the duration. At\
the end of each of its turns, the target repeats the\
save, ending the spell on itself on a success.\
Using a Higher-Level Spell Slot. You can target\
one additional Humanoid for each spell slot level\
above 2.\

Original layout: [[../../srd-5.2.1/pages/141|page 141]].
