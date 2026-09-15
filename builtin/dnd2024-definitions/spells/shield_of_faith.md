---
splat_schema: 1
splat_id: f8a6cb55-7317-5890-9e47-339778a5c94b
splat_kind: spell
splat_key: dnd2024:spells/shield_of_faith
splat_name: Shield of Faith
splat_origin:
  key: dnd2024:reference/spell/shield_of_faith
splat_sources:
- title: SRD 5.2.1
  locator: pp. 162
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 1
  school: abjuration
  classes:
  - cleric
  - paladin
  components:
  - verbal
  - somatic
  - material
  concentration: true
  duration: Concentration, up to 10 minutes
  casting_time: Bonus Action
  ritual: false
  range:
    feet: 60
  material:
    description: a prayer scroll
    consumed: false
splat_conversion:
  status: partial
  category: spell
  source_sha256: ab6704c2d3441843a759aa0da8cf1b60416332b2bb99c1a4f9edfc5d00e4701b
  source_path: builtin/srd-5.2.1/spell/shield_of_faith.md
  pages:
  - 162
  engine_gaps: []
  conversion_gaps:
  - source_rules
  execution_profiles: []
splat_clauses:
- key: source_rules
  text: '# Shield of Faith Level 1 Abjuration (Cleric, Paladin) Casting Time: Bonus Action Range: 60 feet
    Components: V, S, M (a prayer scroll) Duration: Concentration, up to 10 minutes A shimmering field
    surrounds a creature of your choice within range, granting it a +2 bonus to AC for the duration.'
  sha256: a59ec3cdbe9756f219a30113c637553d047418e86a1f08c34a03b64ae890891b
  status: needs_semantic_conversion
---

# Shield of Faith

<!-- SRD page 162 -->

Level 1 Abjuration (Cleric, Paladin)\
Casting Time: Bonus Action\
Range: 60 feet\
Components: V, S, M (a prayer scroll)\
Duration: Concentration, up to 10 minutes\
A shimmering field surrounds a creature of your\
choice within range, granting it a +2 bonus to AC for\
the duration.\

Original layout: [[../../srd-5.2.1/pages/162|page 162]].
