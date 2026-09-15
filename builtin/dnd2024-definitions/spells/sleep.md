---
splat_schema: 1
splat_id: aecb4178-b2fb-5b41-9610-a5ad9a398f94
splat_kind: spell
splat_key: dnd2024:spells/sleep
splat_name: Sleep
splat_origin:
  key: dnd2024:reference/spell/sleep
splat_sources:
- title: SRD 5.2.1
  locator: pp. 163
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 1
  school: enchantment
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
    feet: 60
  material:
    description: a pinch of sand or rose petals
    consumed: false
splat_conversion:
  status: partial
  category: spell
  source_sha256: d76f2a9b6e5a083b2298ee0acae823461ced34bd39c59f0cc309468eab4324f2
  source_path: builtin/srd-5.2.1/spell/sleep.md
  pages:
  - 163
  engine_gaps: []
  conversion_gaps:
  - source_rules
  execution_profiles: []
splat_clauses:
- key: source_rules
  text: '# Sleep Level 1 Enchantment (Bard, Sorcerer, Wizard) Casting Time: Action Range: 60 feet Components:
    V, S, M (a pinch of sand or rose petals) Duration: Concentration, up to 1 minute Each creature of
    your choice in a 5-foot-radius Sphere centered on a point within range must succeed on a Wisdom saving
    throw or have the Incapacitated condition until the end of its next turn, at which point it must repeat
    the save. If the target fails the second save, the target has the Unconscious condition for the duration.
    The spell ends on a target if it takes damage or someone within 5 feet of it takes an action to shake
    it out of the spell’s effect. Creatures that don’t sleep, such as elves, or that have Immunity to
    the Exhaustion condition automatically succeed on saves against this spell.'
  sha256: 3a6b4b37b565fd7816976041ad76275fbb2ebdc41456fafda8158de859356880
  status: needs_semantic_conversion
---

# Sleep

<!-- SRD page 163 -->

Level 1 Enchantment (Bard, Sorcerer, Wizard)\
Casting Time: Action\
Range: 60 feet\
Components: V, S, M (a pinch of sand or rose petals)\
Duration: Concentration, up to 1 minute\
Each creature of your choice in a 5-foot-radius\
Sphere centered on a point within range must suc-\
ceed on a Wisdom saving throw or have the Inca-\
pacitated condition until the end of its next turn,\
at which point it must repeat the save. If the target\
fails the second save, the target has the Unconscious\
condition for the duration. The spell ends on a tar-\
get if it takes damage or someone within 5 feet of it\
takes an action to shake it out of the spell’s effect.\
Creatures that don’t sleep, such as elves, or that\
have Immunity to the Exhaustion condition auto-\
matically succeed on saves against this spell.\

Original layout: [[../../srd-5.2.1/pages/163|page 163]].
