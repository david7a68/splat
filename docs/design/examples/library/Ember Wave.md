---
splat_schema: 1
splat_id: 0375c0dd-6a44-4459-88ad-f23d9f156e01
splat_kind: spell
splat_key: "my_homebrew:spells/ember_wave"
splat_name: Ember Wave
splat_spell:
  level: 1
  school: evocation
  components: [verbal, somatic]
  concentration: false
  duration: instantaneous
splat_parameters:
  slot_pool: {type: actor_resource, required: true}
  cast_level: {type: integer, minimum: 1, maximum: 1}
  save_dc: {type: integer, minimum: 1}
splat_actions:
  cast:
    name: Cast Ember Wave
    activation: {economy: action, category: magic, timing: on_turn, cost_at: commit}
    requirements:
      - {predicate: "dnd2024:spell_casting", spell: self}
    targets:
      chosen:
        kind: creature
        count: {min: 1, max: 3}
        allow_self: false
        range: {feet: 30}
        visibility: seen
    costs:
      - from: actor
        resource: {binding: slot_pool}
        amount: 1
        at: "dnd2024:spell_slot_expenditure"
    rolls:
      damage: {expression: "2d6", scope: use}
    steps:
      - key: burst
        operation: "dnd2024:save_damage"
        target: chosen
        ability: dexterity
        dc: {binding: save_dc}
        damage:
          - {type: fire, amount: {roll: damage}}
        success: {damage_scale: {numerator: 1, denominator: 2}}
---

# Ember Wave

A homebrew spell for a shared damage roll and separate saving throws. Choose
up to three creatures you can see within 30 feet. Each makes a Dexterity save,
taking the rolled Fire damage on failure or half on success.

This fixture has no higher-level benefit and permits only its level-one
binding. A future upcasting variant must declare its level-dependent dice.

`splat:use cast` · `splat:roll cast.damage`
