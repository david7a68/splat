---
splat_schema: 1
splat_id: 33726bf6-56aa-5b4e-adfd-4bde35115996
splat_kind: spell
splat_key: dnd2024:spells/heal
splat_name: Heal
splat_origin:
  key: dnd2024:reference/spell/heal
splat_sources:
- title: SRD 5.2.1
  locator: pp. 139
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 6
  school: abjuration
  classes:
  - cleric
  - druid
  components:
  - verbal
  - somatic
  concentration: false
  duration: Instantaneous
  casting_time: Action
  ritual: false
  range:
    feet: 60
splat_conversion:
  status: partial
  category: spell
  source_sha256: 7c1f9124376ce5286af7ddd81b799371231ced00cbc2674b40002ccc885f60d7
  source_path: builtin/srd-5.2.1/spell/heal.md
  pages:
  - 139
  engine_gaps: []
  conversion_gaps:
  - source_rules
  execution_profiles: []
splat_clauses:
- key: source_rules
  text: '# Heal Level 6 Abjuration (Cleric, Druid) Casting Time: Action Range: 60 feet Components: V,
    S Duration: Instantaneous Choose a creature that you can see within range. Positive energy washes
    through the target, restoring 70 Hit Points. This spell also ends the Blinded, Deafened, and Poisoned
    conditions on the target. Using a Higher-Level Spell Slot. The healing increases by 10 for each spell
    slot level above 6.'
  sha256: 840d96f7982ad01f0d655e9ab8b2ee7dedc1af283a45a679a322e33b39ac3649
  status: needs_semantic_conversion
---

# Heal

<!-- SRD page 139 -->

Level 6 Abjuration (Cleric, Druid)\
Casting Time: Action\
Range: 60 feet\
Components: V, S\
Duration: Instantaneous\
Choose a creature that you can see within range.\
Positive energy washes through the target, restor-\
ing 70 Hit Points. This spell also ends the Blinded,\
Deafened, and Poisoned conditions on the target.\
Using a Higher-Level Spell Slot. The healing in-\
creases by 10 for each spell slot level above 6.\

Original layout: [[../../srd-5.2.1/pages/139|page 139]].
