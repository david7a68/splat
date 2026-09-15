---
splat_schema: 1
splat_id: e9180ba2-3083-50d8-8054-9cd64e6f8d02
splat_kind: spell
splat_key: dnd2024:spells/confusion
splat_name: Confusion
splat_origin:
  key: dnd2024:reference/spell/confusion
splat_sources:
- title: SRD 5.2.1
  locator: pp. 117
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 4
  school: enchantment
  classes:
  - bard
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
    feet: 90
  material:
    description: three nut shells
    consumed: false
splat_conversion:
  status: partial
  category: spell
  source_sha256: df8af5f02e11ccf42efeeaa20d5998a8f6e6feb84ed5eb17e6d5b915cc891473
  source_path: builtin/srd-5.2.1/spell/confusion.md
  pages:
  - 117
  engine_gaps: []
  conversion_gaps:
  - source_rules
  execution_profiles: []
splat_clauses:
- key: source_rules
  text: '# Confusion Level 4 Enchantment (Bard, Druid, Sorcerer, Wizard) Casting Time: Action Range: 90
    feet Components: V, S, M (three nut shells) Duration: Concentration, up to 1 minute Each creature
    in a 10-foot-radius Sphere centered on a point you choose within range must succeed on a Wisdom saving
    throw, or that target can’t take Bonus Actions or Reactions and must roll 1d10 at the start of each
    of its turns to determine its behavior for that turn, consulting the table below. 1d10 Behavior for
    the Turn 1 The target doesn’t take an action, and it uses all its movement to move. Roll 1d4 for the
    direction: 1, north; 2, east; 3, south; or 4, west. 2–6 The target doesn’t move or take actions. 7–8
    The target doesn’t move, and it takes the Attack action to make one melee attack against a random
    creature within reach. If none are within reach, the target takes no action. 9–10 The target chooses
    its behavior. At the end of each of its turns, an affected target repeats the save, ending the spell
    on itself on a success. Using a Higher-Level Spell Slot. The Sphere’s radius increases by 5 feet for
    each spell slot level above 4.'
  sha256: f47bc7292a5ac986f5e9c23ccee44c83a1f9bac93d723846df8b968c4d227e31
  status: needs_semantic_conversion
---

# Confusion

<!-- SRD page 117 -->

Level 4 Enchantment (Bard, Druid, Sorcerer, Wizard)\
Casting Time: Action\
Range: 90 feet\
Components: V, S, M (three nut shells)\
Duration: Concentration, up to 1 minute\
Each creature in a 10-foot-radius Sphere centered\
on a point you choose within range must succeed on\
a Wisdom saving throw, or that target can’t take Bo-\
nus Actions or Reactions and must roll 1d10 at the\
start of each of its turns to determine its behavior\
for that turn, consulting the table below.\
1d10 Behavior for the Turn\
1 The target doesn’t take an action, and it uses all\
its movement to move. Roll 1d4 for the direc-\
tion: 1, north; 2, east; 3, south; or 4, west.\
2–6 The target doesn’t move or take actions.\
7–8 The target doesn’t move, and it takes the At-\
tack action to make one melee attack against\
a random creature within reach. If none are\
within reach, the target takes no action.\
9–10 The target chooses its behavior.\
At the end of each of its turns, an affected target\
repeats the save, ending the spell on itself on a\
success.\
Using a Higher-Level Spell Slot. The Sphere’s\
radius increases by 5 feet for each spell slot level\
above 4.\

Original layout: [[../../srd-5.2.1/pages/117|page 117]].
