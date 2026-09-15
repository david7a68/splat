---
splat_schema: 1
splat_id: 4ff38385-cf51-5684-abd1-6ed1274c5916
splat_kind: spell
splat_key: dnd2024:spells/cure_wounds
splat_name: Cure Wounds
splat_origin:
  key: dnd2024:reference/spell/cure_wounds
splat_sources:
- title: SRD 5.2.1
  locator: spell/cure_wounds
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 1
  school: abjuration
  components:
  - verbal
  - somatic
  concentration: false
  duration: instantaneous
splat_parameters:
  cast_level:
    type: integer
    minimum: 1
    maximum: 9
    required: true
  slot_pool:
    type: actor_resource
    required: true
  spellcasting_modifier:
    type: integer
    minimum: -5
    maximum: 10
    required: true
splat_actions:
  cast:
    name: Cast Cure Wounds
    activation:
      economy: action
      category: magic
      timing: on_turn
      cost_at: commit
    requirements:
    - predicate: dnd2024:spell_casting
      spell: self
    targets:
      chosen:
        kind: creature
        count:
          min: 1
          max: 1
        allow_self: true
        range:
          touch: true
    costs:
    - from: actor
      resource:
        binding: slot_pool
      amount: 1
      at: dnd2024:spell_slot_expenditure
    rolls:
      healing:
        dice:
          count:
            op: multiply
            args:
            - 2
            - binding: cast_level
          sides: 8
          add:
            binding: spellcasting_modifier
        scope: target
    steps:
    - key: resolve
      operation: dnd2024:heal
      target: chosen
      amount:
        roll: healing
---

# Cure Wounds

Mechanics transcribed from SRD 5.2.1. See the source reference for the complete text.
