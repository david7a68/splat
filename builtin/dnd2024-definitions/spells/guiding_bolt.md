---
splat_schema: 1
splat_id: cdf3eb74-6d96-578f-88ad-9aa3b89bcd14
splat_kind: spell
splat_key: dnd2024:spells/guiding_bolt
splat_name: Guiding Bolt
splat_origin:
  key: dnd2024:reference/spell/guiding_bolt
splat_sources:
- title: SRD 5.2.1
  locator: pp. 138
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 1
  school: evocation
  classes:
  - cleric
  components:
  - verbal
  - somatic
  concentration: false
  duration: 1 round
  casting_time: Action
  ritual: false
  range:
    feet: 120
splat_rule:
  mechanic: dnd2024:spells/guiding_bolt
  implementation: dnd2024:spell_guiding_bolt
  parameters:
    targeting:
      kind: creature
      count: 1
      within_range: true
    attack:
      kind: ranged_spell
      hit:
        damage:
          dice: 4d6
          type: radiant
        rider:
          next_attack_against_target: advantage
          deadline: caster_next_turn_end
          consumes_on: next_attack_roll
    slot_scaling:
      above: 1
      damage_per_level: 1d6
    spell_lifetime:
      concentration: false
      maximum:
        value: 1
        unit: round
splat_contracts:
- id: dnd2024:spell_guiding_bolt
  inputs:
    caster:
      type: participant
      required: true
    cast_level:
      type: integer
      required: true
    targets:
      type: participant_list
      required: true
  outputs:
  - effects
  - damage_results
  events:
  - spell.cast_completed
  - attack.hit
  - attack.roll_completed
  - effect.ended
  capabilities:
  - spellcasting
  - effects
  - damage
  - attacks
splat_conversion:
  status: specified
  category: spell
  source_sha256: a78f3f739c76b234d0aea9bcc40638c1fa4354325c90709f0e725f0afa6d244f
  source_path: builtin/srd-5.2.1/spell/guiding_bolt.md
  pages:
  - 138
  engine_gaps: &id001
  - dnd2024:spell_guiding_bolt
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_2.md
  specification_sha256: 136e9ba8f2f5f1876992ef505b27b2bf302c81c0653e3ab3e837f68dc1400968
splat_clauses:
- key: source_rules
  text: '# Guiding Bolt Level 1 Evocation (Cleric) Casting Time: Action Range: 120 feet Components: V,
    S Duration: 1 round You hurl a bolt of light toward a creature within range. Make a ranged spell attack
    against the target. On a hit, it takes 4d6 Radiant damage, and the next attack roll made against it
    before the end of your next turn has Advantage. Using a Higher-Level Spell Slot. The damage increases
    by 1d6 for each spell slot level above 1.'
  sha256: 1664fcd7d222a4cda6023b0e4253685906e25bad2ef5d5caa0e60c4404105936
  status: contract_specified
splat_requires_mechanics: *id001
---

# Guiding Bolt

<!-- SRD page 138 -->

Level 1 Evocation (Cleric)\
Casting Time: Action\
Range: 120 feet\
Components: V, S\
Duration: 1 round\
You hurl a bolt of light toward a creature within\
range. Make a ranged spell attack against the tar-\
get. On a hit, it takes 4d6 Radiant damage, and the\
next attack roll made against it before the end of\
your next turn has Advantage.\
Using a Higher-Level Spell Slot. The damage in-\
creases by 1d6 for each spell slot level above 1.\

Original layout: [[../../srd-5.2.1/pages/138|page 138]].
