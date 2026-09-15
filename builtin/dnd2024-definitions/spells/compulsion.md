---
splat_schema: 1
splat_id: fc71ebbf-7389-5ddc-9c61-c6ca3fce7978
splat_kind: spell
splat_key: dnd2024:spells/compulsion
splat_name: Compulsion
splat_origin:
  key: dnd2024:reference/spell/compulsion
splat_sources:
- title: SRD 5.2.1
  locator: pp. 117
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 4
  school: enchantment
  classes:
  - bard
  components:
  - verbal
  - somatic
  concentration: true
  duration: Concentration, up to 1 minute
  casting_time: Action
  ritual: false
  range:
    feet: 30
splat_conversion:
  status: partial
  category: spell
  source_sha256: 5c232aa23c88bac0fdb38dd4323512631a30cb39a9e4fc15ef137478f716941b
  source_path: builtin/srd-5.2.1/spell/compulsion.md
  pages:
  - 117
  engine_gaps: []
  conversion_gaps:
  - source_rules
  execution_profiles: []
splat_clauses:
- key: source_rules
  text: '# Compulsion Level 4 Enchantment (Bard) Casting Time: Action Range: 30 feet Components: V, S
    Duration: Concentration, up to 1 minute Each creature of your choice that you can see within range
    must succeed on a Wisdom saving throw or have the Charmed condition until the spell ends. For the
    duration, you can take a Bonus Action to designate a direction that is horizontal to you. Each Charmed
    target must use as much of its movement as possible to move in that direction on its next turn, taking
    the safest route. After moving in this way, a target repeats the save, ending the spell on itself
    on a success.'
  sha256: 55384efbd2997ffc3c8ffc43ad63da253e87d924233cca58a6a98d4b8f7765f5
  status: needs_semantic_conversion
---

# Compulsion

<!-- SRD page 117 -->

Level 4 Enchantment (Bard)\
Casting Time: Action\
Range: 30 feet\
Components: V, S\
Duration: Concentration, up to 1 minute\
Each creature of your choice that you can see within\
range must succeed on a Wisdom saving throw or\
have the Charmed condition until the spell ends.\
For the duration, you can take a Bonus Action to\
designate a direction that is horizontal to you. Each\
Charmed target must use as much of its movement\
as possible to move in that direction on its next\
turn, taking the safest route. After moving in this\
way, a target repeats the save, ending the spell on\
itself on a success.\

Original layout: [[../../srd-5.2.1/pages/117|page 117]].
