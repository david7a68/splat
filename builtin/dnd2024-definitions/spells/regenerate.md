---
splat_schema: 1
splat_id: 4cac4ccd-a035-56ef-accb-deae22998694
splat_kind: spell
splat_key: dnd2024:spells/regenerate
splat_name: Regenerate
splat_origin:
  key: dnd2024:reference/spell/regenerate
splat_sources:
- title: SRD 5.2.1
  locator: pp. 158
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 7
  school: transmutation
  classes:
  - bard
  - cleric
  - druid
  components:
  - verbal
  - somatic
  - material
  concentration: false
  duration: 1 hour
  casting_time: 1 minute
  ritual: false
  range:
    mode: touch
  material:
    description: a prayer wheel
    consumed: false
splat_conversion:
  status: partial
  category: spell
  source_sha256: 086f00824adcc857f17cb591652b676189219419ecdd3be41644371487be5d0c
  source_path: builtin/srd-5.2.1/spell/regenerate.md
  pages:
  - 158
  engine_gaps: []
  conversion_gaps:
  - source_rules
  execution_profiles: []
splat_clauses:
- key: source_rules
  text: '# Regenerate Level 7 Transmutation (Bard, Cleric, Druid) Casting Time: 1 minute Range: Touch
    Components: V, S, M (a prayer wheel) Duration: 1 hour A creature you touch regains 4d8 + 15 Hit Points.
    For the duration, the target regains 1 Hit Point at the start of each of its turns, and any severed
    body parts regrow after 2 minutes.'
  sha256: f3a7b29cf2f9fc7134ffe931bb02eb3bcba54f9c70df3e5eaf7822d65969617b
  status: needs_semantic_conversion
---

# Regenerate

<!-- SRD page 158 -->

Level 7 Transmutation (Bard, Cleric, Druid)\
Casting Time: 1 minute\
Range: Touch\
Components: V, S, M (a prayer wheel)\
Duration: 1 hour\
A creature you touch regains 4d8 + 15 Hit Points.\
For the duration, the target regains 1 Hit Point at\
the start of each of its turns, and any severed body\
parts regrow after 2 minutes.\

Original layout: [[../../srd-5.2.1/pages/158|page 158]].
