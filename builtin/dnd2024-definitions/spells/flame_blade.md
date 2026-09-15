---
splat_schema: 1
splat_id: 3c2661a3-d5fe-56e7-9c6e-8e5439258de8
splat_kind: spell
splat_key: dnd2024:spells/flame_blade
splat_name: Flame Blade
splat_origin:
  key: dnd2024:reference/spell/flame_blade
splat_sources:
- title: SRD 5.2.1
  locator: pp. 132
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 2
  school: evocation
  classes:
  - druid
  - sorcerer
  components:
  - verbal
  - somatic
  - material
  concentration: true
  duration: Concentration, up to 10 minutes
  casting_time: Bonus Action
  ritual: false
  range:
    mode: self
  material:
    description: a sumac leaf
    consumed: false
splat_conversion:
  status: partial
  category: spell
  source_sha256: c4a5f260527a5ad42b0b4c24ad5734becd8ae21b0cfba58ce6f4f3ccb9641476
  source_path: builtin/srd-5.2.1/spell/flame_blade.md
  pages:
  - 132
  engine_gaps: []
  conversion_gaps:
  - source_rules
  execution_profiles: []
splat_clauses:
- key: source_rules
  text: '# Flame Blade Level 2 Evocation (Druid, Sorcerer) Casting Time: Bonus Action Range: Self Components:
    V, S, M (a sumac leaf) Duration: Concentration, up to 10 minutes You evoke a fiery blade in your free
    hand. The blade is similar in size and shape to a scimitar, and it lasts for the duration. If you
    let go of the blade, it disappears, but you can evoke it again as a Bonus Action. As a Magic action,
    you can make a melee spell attack with the fiery blade. On a hit, the target takes Fire damage equal
    to 3d6 plus your spellcasting ability modifier. The flaming blade sheds Bright Light in a 10-foot
    radius and Dim Light for an additional 10 feet. Using a Higher-Level Spell Slot. The damage increases
    by 1d6 for each spell slot level above 2.'
  sha256: 5835b742b83e01ae30879cb9e715332e960c410d3ab487cf08cdd4ad5fe86686
  status: needs_semantic_conversion
---

# Flame Blade

<!-- SRD page 132 -->

Level 2 Evocation (Druid, Sorcerer)\
Casting Time: Bonus Action\
Range: Self\
Components: V, S, M (a sumac leaf)\
Duration: Concentration, up to 10 minutes\
You evoke a fiery blade in your free hand. The blade\
is similar in size and shape to a scimitar, and it lasts\
for the duration. If you let go of the blade, it disap-\
pears, but you can evoke it again as a Bonus Action.\
As a Magic action, you can make a melee spell at-\
tack with the fiery blade. On a hit, the target takes\
Fire damage equal to 3d6 plus your spellcasting\
ability modifier.\
The flaming blade sheds Bright Light in a 10-foot\
radius and Dim Light for an additional 10 feet.\
Using a Higher-Level Spell Slot. The damage in-\
creases by 1d6 for each spell slot level above 2.\

Original layout: [[../../srd-5.2.1/pages/132|page 132]].
