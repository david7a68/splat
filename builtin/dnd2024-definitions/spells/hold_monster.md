---
splat_schema: 1
splat_id: e6dcdfeb-2dbf-5311-90d9-0de9ab82d5f8
splat_kind: spell
splat_key: dnd2024:spells/hold_monster
splat_name: Hold Monster
splat_origin:
  key: dnd2024:reference/spell/hold_monster
splat_sources:
- title: SRD 5.2.1
  locator: pp. 140, 141
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 5
  school: enchantment
  classes:
  - bard
  - sorcerer
  - warlock
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
    feet: 90
  material:
    description: a straight piece of iron
    consumed: false
splat_conversion:
  status: partial
  category: spell
  source_sha256: 18c1656b1e6555c98a13304c357633c785271302547955b754bfbfa013955498
  source_path: builtin/srd-5.2.1/spell/hold_monster.md
  pages:
  - 140
  - 141
  engine_gaps: []
  conversion_gaps:
  - source_rules
  execution_profiles: []
splat_clauses:
- key: source_rules
  text: '# Hold Monster Level 5 Enchantment (Bard, Sorcerer, Warlock, Wizard) Casting Time: Action Range:
    90 feet Components: V, S, M (a straight piece of iron) Duration: Concentration, up to 1 minute Choose
    a creature that you can see within range. The target must succeed on a Wisdom saving throw or have
    the Paralyzed condition for the duration. At the end of each of its turns, the target repeats the
    save, ending the spell on itself on a success. Using a Higher-Level Spell Slot. You can target one
    additional creature for each spell slot level above 5.'
  sha256: 5af39cb8d361aefe1cd87f005da2a69ce0245ab2e0e10d9bdaf078023511ab7f
  status: needs_semantic_conversion
---

# Hold Monster

<!-- SRD page 140 -->

Level 5 Enchantment (Bard, Sorcerer, Warlock,\
Wizard)\
Casting Time: Action\
Range: 90 feet\

<!-- SRD page 141 -->

Components: V, S, M (a straight piece of iron)\
Duration: Concentration, up to 1 minute\
Choose a creature that you can see within range.\
The target must succeed on a Wisdom saving throw\
or have the Paralyzed condition for the duration. At\
the end of each of its turns, the target repeats the\
save, ending the spell on itself on a success.\
Using a Higher-Level Spell Slot. You can target\
one additional creature for each spell slot level\
above 5.\

Original layout: [[../../srd-5.2.1/pages/140|page 140]], [[../../srd-5.2.1/pages/141|page 141]].
