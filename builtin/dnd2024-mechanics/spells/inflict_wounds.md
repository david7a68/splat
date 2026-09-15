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
  locator: spell/inflict_wounds
  url: https://www.dndbeyond.com/srd
splat_spell:
  level: 1
  school: necromancy
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
  save_dc:
    type: integer
    minimum: 1
    maximum: 40
    required: true
splat_actions:
  cast:
    name: Cast Inflict Wounds
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
      damage:
        dice:
          count:
            op: add
            args:
            - 1
            - binding: cast_level
          sides: 10
          add: 0
        scope: use
    steps:
    - key: resolve
      operation: dnd2024:save_damage
      target: chosen
      ability: constitution
      dc:
        binding: save_dc
      damage:
      - type: necrotic
        amount:
          roll: damage
      success:
        damage_scale:
          numerator: 1
          denominator: 2
---

# Inflict Wounds

Mechanics transcribed from SRD 5.2.1. See the source reference for the complete text.
