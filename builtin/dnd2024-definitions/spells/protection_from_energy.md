---
splat_schema: 1
splat_id: ae6847a4-e9e0-513b-b60d-6ea97b144ca6
splat_kind: spell
splat_key: dnd2024:spells/protection_from_energy
splat_name: Protection from Energy
splat_origin:
  key: dnd2024:reference/spell/protection_from_energy
splat_sources:
- title: SRD 5.2.1
  locator: pp. 157
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 3
  school: abjuration
  classes:
  - cleric
  - druid
  - ranger
  - sorcerer
  - wizard
  components:
  - verbal
  - somatic
  concentration: true
  duration: Concentration, up to 1 hour
  casting_time: Action
  ritual: false
  range:
    mode: touch
splat_conversion:
  status: partial
  category: spell
  source_sha256: 976b5375e1dac1290af23235e6dda78a42011b8744ff8dcd0c8da62df565437a
  source_path: builtin/srd-5.2.1/spell/protection_from_energy.md
  pages:
  - 157
  engine_gaps: []
  conversion_gaps:
  - source_rules
  execution_profiles: []
splat_clauses:
- key: source_rules
  text: '# Protection from Energy Level 3 Abjuration (Cleric, Druid, Ranger, Sorcerer, Wizard) Casting
    Time: Action Range: Touch Components: V, S Duration: Concentration, up to 1 hour For the duration,
    the willing creature you touch has Resistance to one damage type of your choice: Acid, Cold, Fire,
    Lightning, or Thunder.'
  sha256: bcfa76f7980b74e7a9f371d7c314b6fd122d973b3ca6bac5d7a16758e59483e5
  status: needs_semantic_conversion
---

# Protection from Energy

<!-- SRD page 157 -->

Level 3 Abjuration (Cleric, Druid, Ranger, Sorcerer,\
Wizard)\
Casting Time: Action\
Range: Touch\
Components: V, S\
Duration: Concentration, up to 1 hour\
For the duration, the willing creature you touch has\
Resistance to one damage type of your choice: Acid,\
Cold, Fire, Lightning, or Thunder.\

Original layout: [[../../srd-5.2.1/pages/157|page 157]].
