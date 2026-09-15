---
splat_schema: 1
splat_id: 1ce5ee88-00e0-5c6b-ac9c-36b851c8227b
splat_kind: spell
splat_key: dnd2024:spells/slow
splat_name: Slow
splat_origin:
  key: dnd2024:reference/spell/slow
splat_sources:
- title: SRD 5.2.1
  locator: pp. 163
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 3
  school: transmutation
  classes:
  - bard
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
    feet: 120
  material:
    description: a drop of molasses
    consumed: false
splat_conversion:
  status: partial
  category: spell
  source_sha256: 250497db2f0dfbf1e61fb69af0605c082bea6012b8a8e2b1af5120bc9e7ad8ce
  source_path: builtin/srd-5.2.1/spell/slow.md
  pages:
  - 163
  engine_gaps: []
  conversion_gaps:
  - source_rules
  execution_profiles: []
splat_clauses:
- key: source_rules
  text: '# Slow Level 3 Transmutation (Bard, Sorcerer, Wizard) Casting Time: Action Range: 120 feet Components:
    V, S, M (a drop of molasses) Duration: Concentration, up to 1 minute You alter time around up to six
    creatures of your choice in a 40-foot Cube within range. Each target must succeed on a Wisdom saving
    throw or be affected by this spell for the duration. An affected target’s Speed is halved, it takes
    a -2 penalty to AC and Dexterity saving throws, and it can’t take Reactions. On its turns, it can
    take either an action or a Bonus Action, not both, and it can make only one attack if it takes the
    Attack action. If it casts a spell with a Somatic component, there is a 25 percent chance the spell
    fails as a result of the target making the spell’s gestures too slowly. An affected target repeats
    the save at the end of each of its turns, ending the spell on itself on a success.'
  sha256: baaac85f48c7bc958c4028e70056052ebeba5f424e5b2122083c127177d39be9
  status: needs_semantic_conversion
---

# Slow

<!-- SRD page 163 -->

Level 3 Transmutation (Bard, Sorcerer, Wizard)\
Casting Time: Action\
Range: 120 feet\
Components: V, S, M (a drop of molasses)\
Duration: Concentration, up to 1 minute\
You alter time around up to six creatures of your\
choice in a 40-foot Cube within range. Each target\
must succeed on a Wisdom saving throw or be af-\
fected by this spell for the duration.\
An affected target’s Speed is halved, it takes a −2\
penalty to AC and Dexterity saving throws, and it\
can’t take Reactions. On its turns, it can take either\
an action or a Bonus Action, not both, and it can\
make only one attack if it takes the Attack action. If\
it casts a spell with a Somatic component, there is\
a 25 percent chance the spell fails as a result of the\
target making the spell’s gestures too slowly.\
An affected target repeats the save at the end\
of each of its turns, ending the spell on itself on a\
success.\

Original layout: [[../../srd-5.2.1/pages/163|page 163]].
