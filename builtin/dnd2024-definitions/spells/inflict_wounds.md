---
splat_schema: 1
splat_id: 0beda0ed-f8cd-515d-9e57-9416c350b41b
splat_kind: spell
splat_key: dnd2024:spells/inflict_wounds
splat_name: Inflict Wounds
splat_origin:
  key: dnd2024:reference/spell/inflict_wounds
splat_sources:
- title: SRD 5.2.1
  locator: pp. 143
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 1
  school: necromancy
  classes:
  - cleric
  components:
  - verbal
  - somatic
  concentration: false
  duration: Instantaneous
  casting_time: Action
  ritual: false
  range:
    mode: touch
splat_conversion:
  status: partial
  category: spell
  source_sha256: c1b84c92fdde772b564dc70768c1ed88caa000242b0c28af403436dbac5253f5
  source_path: builtin/srd-5.2.1/spell/inflict_wounds.md
  pages:
  - 143
  engine_gaps: []
  conversion_gaps:
  - source_rules
  execution_profiles:
  - dnd2024:spells/inflict_wounds
splat_clauses:
- key: source_rules
  text: '# Inflict Wounds Level 1 Necromancy (Cleric) Casting Time: Action Range: Touch Components: V,
    S Duration: Instantaneous A creature you touch makes a Constitution saving throw, taking 2d10 Necrotic
    damage on a failed save or half as much damage on a successful one. Using a Higher-Level Spell Slot.
    The damage increases by 1d10 for each spell slot level above 1.'
  sha256: 372fc04912f0cc2b99a0d4b72c3882069be7802cb4d70aa66fef8be93053ee20
  status: needs_semantic_conversion
---

# Inflict Wounds

<!-- SRD page 143 -->

Level 1 Necromancy (Cleric)\
Casting Time: Action\
Range: Touch\
Components: V, S\
Duration: Instantaneous\
A creature you touch makes a Constitution saving\
throw, taking 2d10 Necrotic damage on a failed save\
or half as much damage on a successful one.\
Using a Higher-Level Spell Slot. The damage in-\
creases by 1d10 for each spell slot level above 1.\

Original layout: [[../../srd-5.2.1/pages/143|page 143]].
