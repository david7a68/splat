---
splat_schema: 1
splat_id: 429ab81d-cc32-5595-aeeb-6901f5462992
splat_kind: spell
splat_key: dnd2024:spells/fear
splat_name: Fear
splat_origin:
  key: dnd2024:reference/spell/fear
splat_sources:
- title: SRD 5.2.1
  locator: pp. 130
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 3
  school: illusion
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
    mode: self
  material:
    description: a white feather
    consumed: false
splat_conversion:
  status: partial
  category: spell
  source_sha256: 1aab24a12291968bfd66880de429916d72e8b90499424a12950b2edf3bf779d3
  source_path: builtin/srd-5.2.1/spell/fear.md
  pages:
  - 130
  engine_gaps: []
  conversion_gaps:
  - source_rules
  execution_profiles: []
splat_clauses:
- key: source_rules
  text: '# Fear Level 3 Illusion (Bard, Sorcerer, Warlock, Wizard) Casting Time: Action Range: Self Components:
    V, S, M (a white feather) Duration: Concentration, up to 1 minute Each creature in a 30-foot Cone
    must succeed on a Wisdom saving throw or drop whatever it is holding and have the Frightened condition
    for the duration. A Frightened creature takes the Dash action and moves away from you by the safest
    route on each of its turns unless there is nowhere to move. If the creature ends its turn in a space
    where it doesn’t have line of sight to you, the creature makes a Wisdom saving throw. On a successful
    save, the spell ends on that creature.'
  sha256: 291c65a310068b0626e424f30b2fa09797c4137d6c356c493382e10f7972d7ce
  status: needs_semantic_conversion
---

# Fear

<!-- SRD page 130 -->

Level 3 Illusion (Bard, Sorcerer, Warlock, Wizard)\
Casting Time: Action\
Range: Self\
Components: V, S, M (a white feather)\
Duration: Concentration, up to 1 minute\
Each creature in a 30-foot Cone must succeed on a\
Wisdom saving throw or drop whatever it is holding\
and have the Frightened condition for the duration.\
A Frightened creature takes the Dash action and\
moves away from you by the safest route on each\
of its turns unless there is nowhere to move. If the\
creature ends its turn in a space where it doesn’t\
have line of sight to you, the creature makes a Wis-\
dom saving throw. On a successful save, the spell\
ends on that creature.\

Original layout: [[../../srd-5.2.1/pages/130|page 130]].
