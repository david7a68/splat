---
splat_schema: 1
splat_id: c73aa5d8-490d-50b8-b117-d86eccae5c96
splat_kind: spell
splat_key: dnd2024:spells/sunbeam
splat_name: Sunbeam
splat_origin:
  key: dnd2024:reference/spell/sunbeam
splat_sources:
- title: SRD 5.2.1
  locator: pp. 166
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 6
  school: evocation
  classes:
  - cleric
  - druid
  - sorcerer
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
    mode: self
  material:
    description: a magnifying glass
    consumed: false
splat_conversion:
  status: partial
  category: spell
  source_sha256: ce4d665540dac73d8e8450fdb6d9b6d3f3db4bf5c82a01bcbcbfe210e3321022
  source_path: builtin/srd-5.2.1/spell/sunbeam.md
  pages:
  - 166
  engine_gaps: []
  conversion_gaps:
  - source_rules
  execution_profiles: []
splat_clauses:
- key: source_rules
  text: '# Sunbeam Level 6 Evocation (Cleric, Druid, Sorcerer, Wizard) Casting Time: Action Range: Self
    Components: V, S, M (a magnifying glass) Duration: Concentration, up to 1 minute You launch a sunbeam
    in a 5-foot-wide, 60-foot-long Line. Each creature in the Line makes a Constitution saving throw.
    On a failed save, a creature takes 6d8 Radiant damage and has the Blinded condition until the start
    of your next turn. On a successful save, it takes half as much damage only. Until the spell ends,
    you can take a Magic action to create a new Line of radiance. For the duration, a mote of brilliant
    radiance shines above you. It sheds Bright Light in a 30-foot radius and Dim Light for an additional
    30 feet. This light is sunlight.'
  sha256: 8266c26a52af5d69aa8999df7d1ecca9ea1d005cb71f1aa36b26dca8e653d2de
  status: needs_semantic_conversion
---

# Sunbeam

<!-- SRD page 166 -->

Level 6 Evocation (Cleric, Druid, Sorcerer, Wizard)\
Casting Time: Action\
Range: Self\
Components: V, S, M (a magnifying glass)\
Duration: Concentration, up to 1 minute\
You launch a sunbeam in a 5-foot-wide, 60-foot-long\
Line. Each creature in the Line makes a Constitution\
saving throw. On a failed save, a creature takes 6d8\
Radiant damage and has the Blinded condition until\
the start of your next turn. On a successful save, it\
takes half as much damage only.\
Until the spell ends, you can take a Magic action to\
create a new Line of radiance.\
For the duration, a mote of brilliant radiance\
shines above you. It sheds Bright Light in a 30-foot\
radius and Dim Light for an additional 30 feet. This\
light is sunlight.\

Original layout: [[../../srd-5.2.1/pages/166|page 166]].
