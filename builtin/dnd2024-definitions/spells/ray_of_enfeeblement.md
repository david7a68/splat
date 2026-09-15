---
splat_schema: 1
splat_id: cec90d8e-7a1e-533b-ac52-1cb607f5e548
splat_kind: spell
splat_key: dnd2024:spells/ray_of_enfeeblement
splat_name: Ray of Enfeeblement
splat_origin:
  key: dnd2024:reference/spell/ray_of_enfeeblement
splat_sources:
- title: SRD 5.2.1
  locator: pp. 157
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 2
  school: necromancy
  classes:
  - warlock
  - wizard
  components:
  - verbal
  - somatic
  concentration: true
  duration: Concentration, up to 1 minute
  casting_time: Action
  ritual: false
  range:
    feet: 60
splat_conversion:
  status: partial
  category: spell
  source_sha256: 32ab189bc6709515d3af30883d33907af473a73f52983fcbc68d8205839d83ea
  source_path: builtin/srd-5.2.1/spell/ray_of_enfeeblement.md
  pages:
  - 157
  engine_gaps: []
  conversion_gaps:
  - source_rules
  execution_profiles: []
splat_clauses:
- key: source_rules
  text: '# Ray of Enfeeblement Level 2 Necromancy (Warlock, Wizard) Casting Time: Action Range: 60 feet
    Components: V, S Duration: Concentration, up to 1 minute A beam of enervating energy shoots from you
    toward a creature within range. The target must make a Constitution saving throw. On a successful
    save, the target has Disadvantage on the next attack roll it makes until the start of your next turn.
    On a failed save, the target has Disadvantage on Strength-based D20 Tests for the duration. During
    that time, it also subtracts 1d8 from all its damage rolls. The target repeats the save at the end
    of each of its turns, ending the spell on a success.'
  sha256: 4bab6372bcc1cc72d2449d10735d92698e3c7eb319e01cd5814aba86b494fa77
  status: needs_semantic_conversion
---

# Ray of Enfeeblement

<!-- SRD page 157 -->

Level 2 Necromancy (Warlock, Wizard)\
Casting Time: Action\
Range: 60 feet\
Components: V, S\
Duration: Concentration, up to 1 minute\
A beam of enervating energy shoots from you to-\
ward a creature within range. The target must\
make a Constitution saving throw. On a successful\
save, the target has Disadvantage on the next attack\
roll it makes until the start of your next turn.\
On a failed save, the target has Disadvantage on\
Strength-based D20 Tests for the duration. During\
that time, it also subtracts 1d8 from all its damage\
rolls. The target repeats the save at the end of each\
of its turns, ending the spell on a success.\

Original layout: [[../../srd-5.2.1/pages/157|page 157]].
