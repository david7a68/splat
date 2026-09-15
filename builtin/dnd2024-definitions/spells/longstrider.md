---
splat_schema: 1
splat_id: fa3e0b6e-00cb-5252-aa37-be08cf4e2434
splat_kind: spell
splat_key: dnd2024:spells/longstrider
splat_name: Longstrider
splat_origin:
  key: dnd2024:reference/spell/longstrider
splat_sources:
- title: SRD 5.2.1
  locator: pp. 145
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 1
  school: transmutation
  classes:
  - bard
  - druid
  - ranger
  - wizard
  components:
  - verbal
  - somatic
  - material
  concentration: false
  duration: 1 hour
  casting_time: Action
  ritual: false
  range:
    mode: touch
  material:
    description: a pinch of dirt
    consumed: false
splat_rule:
  mechanic: dnd2024:spells/longstrider
  implementation: dnd2024:spell_longstrider
  parameters:
    targeting:
      kind: creatures
      touch: true
      count:
        base: 1
        per_slot_above: 1
        slot_base: 1
    effect:
      speed_bonus_feet: 10
      duration: spell_duration
    spell_lifetime:
      concentration: false
      maximum:
        value: 1
        unit: hour
splat_contracts:
- id: dnd2024:spell_longstrider
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
  events:
  - spell.cast_completed
  - effect.ended
  capabilities:
  - spellcasting
  - effects
splat_conversion:
  status: specified
  category: spell
  source_sha256: a8080e023eb8548df61b3e007074d5eb83a02d3370e1d1d2fa460e08559b6f18
  source_path: builtin/srd-5.2.1/spell/longstrider.md
  pages:
  - 145
  engine_gaps: &id001
  - dnd2024:spell_longstrider
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/spells_2.md
  specification_sha256: 5d01e8899bc26abcc20469c8857e882b733d5a0697ac3893a18fbd1737beae4f
splat_clauses:
- key: source_rules
  text: '# Longstrider Level 1 Transmutation (Bard, Druid, Ranger, Wizard) Casting Time: Action Range:
    Touch Components: V, S, M (a pinch of dirt) Duration: 1 hour You touch a creature. The target’s Speed
    increases by 10 feet until the spell ends. Using a Higher-Level Spell Slot. You can target one additional
    creature for each spell slot level above 1.'
  sha256: b3d82feeb2532ea13614e9a84352bbacf3ce46806cdaabdb25db6e5deae43150
  status: contract_specified
splat_requires_mechanics: *id001
---

# Longstrider

<!-- SRD page 145 -->

Level 1 Transmutation (Bard, Druid, Ranger, Wizard)\
Casting Time: Action\
Range: Touch\
Components: V, S, M (a pinch of dirt)\
Duration: 1 hour\
You touch a creature. The target’s Speed increases\
by 10 feet until the spell ends.\
Using a Higher-Level Spell Slot. You can target\
one additional creature for each spell slot level\
above 1.\

Original layout: [[../../srd-5.2.1/pages/145|page 145]].
