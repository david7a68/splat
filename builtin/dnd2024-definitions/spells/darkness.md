---
splat_schema: 1
splat_id: 8ed4e98a-af2e-52fe-9716-7adf6f73930a
splat_kind: spell
splat_key: dnd2024:spells/darkness
splat_name: Darkness
splat_origin:
  key: dnd2024:reference/spell/darkness
splat_sources:
- title: SRD 5.2.1
  locator: pp. 122
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 2
  school: evocation
  classes:
  - sorcerer
  - warlock
  - wizard
  components:
  - verbal
  - material
  concentration: true
  duration: Concentration, up to 10 minutes
  casting_time: Action
  ritual: false
  range:
    feet: 60
  material:
    description: bat fur and a piece of coal
    consumed: false
splat_conversion:
  status: partial
  category: spell
  source_sha256: 7ce1ddc3d08b87b887006b614465db2c7fb0b9bc2bccb3a226ed16cec7566600
  source_path: builtin/srd-5.2.1/spell/darkness.md
  pages:
  - 122
  engine_gaps: []
  conversion_gaps:
  - source_rules
  execution_profiles: []
splat_clauses:
- key: source_rules
  text: '# Darkness Level 2 Evocation (Sorcerer, Warlock, Wizard) Casting Time: Action Range: 60 feet
    Components: V, M (bat fur and a piece of coal) Duration: Concentration, up to 10 minutes For the duration,
    magical Darkness spreads from a point within range and fills a 15-foot-radius Sphere. Darkvision can’t
    see through it, and nonmagical light can’t illuminate it. Alternatively, you cast the spell on an
    object that isn’t being worn or carried, causing the Darkness to fill a 15-foot Emanation originating
    from that object. Covering that object with something opaque, such as a bowl or helm, blocks the Darkness.
    If any of this spell’s area overlaps with an area of Bright Light or Dim Light created by a spell
    of level 2 or lower, that other spell is dispelled.'
  sha256: f8328a2475c1abb142230a7eaecf03437240e266f79b897a83c93e7aabd021d8
  status: needs_semantic_conversion
---

# Darkness

<!-- SRD page 122 -->

Level 2 Evocation (Sorcerer, Warlock, Wizard)\
Casting Time: Action\
Range: 60 feet\
Components: V, M (bat fur and a piece of coal)\
Duration: Concentration, up to 10 minutes\
For the duration, magical Darkness spreads from a\
point within range and fills a 15-foot-radius Sphere.\
Darkvision can’t see through it, and nonmagical\
light can’t illuminate it.\
Alternatively, you cast the spell on an object that\
isn’t being worn or carried, causing the Darkness\
to fill a 15-foot Emanation originating from that\
object. Covering that object with something opaque,\
such as a bowl or helm, blocks the Darkness.\
If any of this spell’s area overlaps with an area of\
Bright Light or Dim Light created by a spell of level\
2 or lower, that other spell is dispelled.\

Original layout: [[../../srd-5.2.1/pages/122|page 122]].
