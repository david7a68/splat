---
splat_schema: 1
splat_id: 97c986c3-b654-5430-ab0f-97c7ecf40a31
splat_kind: spell
splat_key: dnd2024:spells/true_seeing
splat_name: True Seeing
splat_origin:
  key: dnd2024:reference/spell/true_seeing
splat_sources:
- title: SRD 5.2.1
  locator: pp. 171
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 6
  school: divination
  classes:
  - bard
  - cleric
  - sorcerer
  - warlock
  - wizard
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
    description: mushroom powder worth 25+ GP, which the spell consumes
    consumed: true
splat_conversion:
  status: partial
  category: spell
  source_sha256: 3a4cda6445c981ef97e55bf30d517c47b1977f11343d6bc6f7f5bd23a4806412
  source_path: builtin/srd-5.2.1/spell/true_seeing.md
  pages:
  - 171
  engine_gaps: []
  conversion_gaps:
  - source_rules
  execution_profiles: []
splat_clauses:
- key: source_rules
  text: '# True Seeing Level 6 Divination (Bard, Cleric, Sorcerer, Warlock, Wizard) Casting Time: Action
    Range: Touch Components: V, S, M (mushroom powder worth 25+ GP, which the spell consumes) Duration:
    1 hour For the duration, the willing creature you touch has Truesight with a range of 120 feet.'
  sha256: 01a96ed61039183f7afe1a18a2fb13ca42814d21ef80756358977a955d13ee1e
  status: needs_semantic_conversion
---

# True Seeing

<!-- SRD page 171 -->

Level 6 Divination (Bard, Cleric, Sorcerer, Warlock,\
Wizard)\
Casting Time: Action\
Range: Touch\
Components: V, S, M (mushroom powder worth 25+\
GP, which the spell consumes)\
Duration: 1 hour\
For the duration, the willing creature you touch has\
Truesight with a range of 120 feet.\

Original layout: [[../../srd-5.2.1/pages/171|page 171]].
