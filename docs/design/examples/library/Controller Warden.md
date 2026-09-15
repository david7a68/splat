---
tags:
- homebrew
- splat-design
splat_schema: 1
splat_id: 0375c0dd-6a44-4459-88ad-f23d9f156e0a
splat_kind: creature
splat_key: my_homebrew:creatures/controller_warden
splat_name: Controller Warden
splat_creature:
  size: medium
  type: humanoid
splat_stats:
  strength:
    value: 16
  dexterity:
    value: 14
  constitution:
    value: 14
  intelligence:
    value: 12
  wisdom:
    value: 12
  charisma:
    value: 10
  proficiency_bonus:
    value: 2
  armor_class:
    value: 16
    includes:
    - inventory.armor.base
    - ability.dexterity
    rebuild: dnd2024:armor_class
  max_hp:
    value: 30
  speed:
    value: 30
    unit: ft
  initiative:
    derive:
      op: ability_modifier
      args:
      - stat:
          of: actor
          key: dexterity
splat_proficiencies:
  saves:
  - strength
  - constitution
  skills:
    athletics: proficient
    perception: proficient
  armor:
  - light
  - medium
  - shield
  weapons:
  - simple
splat_hit_dice:
  count: 4
  die: d8
splat_senses:
  passive_perception: 13
splat_languages:
- common
splat_resources:
  healing:
    name: Healing Uses
    max: 3
    recovery:
    - key: long_rest
      mechanic: dnd2024:long_rest
      restore: all
  spell_slots_1:
    name: Level 1 Spell Slots
    kind: spell_slot
    level: 1
    max: 2
    recovery:
    - key: long_rest
      mechanic: dnd2024:long_rest
      restore: all
splat_actions:
  strike:
    name: Strike
    activation:
      economy: action
      category: attack
      timing: on_turn
      cost_at: commit
    requirements:
    - predicate: equipment_state
      row: spear
      state: wielded
    targets:
      chosen:
        kind: creature
        count:
          min: 1
          max: 1
        range:
          feet: 5
    rolls:
      attack:
        expression: 1d20
        modifier: 5
        scope: attack
      damage:
        expression: 1d8 + 3
        scope: attack
    steps:
    - key: attack
      operation: dnd2024:attack
      target: chosen
      kind: melee_weapon
      weapon_row: spear
      roll: attack
      on_hit:
        damage:
        - type: piercing
          amount:
            roll: damage
  mend:
    name: Mend
    activation:
      economy: action
      category: feature
      timing: on_turn
      cost_at: commit
    targets:
      chosen:
        kind: creature
        count:
          min: 1
          max: 1
        allow_self: true
        willing: true
        range:
          touch: true
    costs:
    - from: actor
      resource: healing
      amount: 1
      at: commit
    rolls:
      healing:
        expression: 1d8 + 2
        scope: target
    steps:
    - key: healing
      operation: dnd2024:heal
      target: chosen
      amount:
        roll: healing
  protect:
    name: Protect Ally
    activation:
      economy: reaction
      category: feature
      cost_at: commit
      trigger:
        event: dnd2024:attack.hit_pending
        predicates:
        - predicate: relation
          subject: event.target
          to: actor
          value: ally
        - predicate: different_actor
          left: event.target
          right: actor
        - predicate: within_reach
          from: actor
          to: event.target
          feet: 5
    effects:
      ward:
        stacking:
          key: my_homebrew:protect_ally
          policy: dnd2024:same_effect
        duration:
          until: resolution_end
          of: event.resolution
        modifiers:
        - stat: armor_class
          operation: add
          value: 2
          scope: event.resolution
    steps:
    - key: ward
      operation: splat:apply_effect
      target:
        context: event.target
      effect: ward
  ember_wave:
    name: Ember Wave
    use:
      entry:
        key: my_homebrew:spells/ember_wave
      action: cast
      bindings:
        slot_pool: spell_slots_1
        cast_level: 1
        save_dc: 13
  frighten:
    name: Frighten
    activation:
      economy: action
      category: feature
      timing: on_turn
      cost_at: commit
    targets:
      chosen:
        kind: creature
        count:
          min: 1
          max: 2
        allow_self: false
        range:
          feet: 30
        visibility: seen
    steps:
    - key: fear
      operation: splat:apply_effect
      target: chosen
      effect:
        local: fearful
      bindings:
        save_dc: 13
splat_inventory_details:
  wand_a:
    resources:
      fire: 4
      healing: 2
  wand_b:
    resources:
      fire: 1
      healing: 1
splat_origin:
  id: 6b02d7d9-19a7-4a18-a5ee-8bb9f00a04bd
splat_effects:
  fearful:
    name: Frightened of the Warden
    parameters:
      save_dc: {type: integer, minimum: 1, required: true}
    stacking:
      key: my_homebrew:fearful
      policy: dnd2024:same_effect
      compare:
        potency: 1
        equal: newest
      scope: application_per_target
    duration:
      any:
      - elapsed:
          seconds: 60
      - event: dnd2024:concentration.ended
        subject: application.source
    conditions:
    - mechanic: dnd2024:conditions/frightened
      source: application.source
    concentration:
      owner: application.source
      required: true
    triggers:
      resist:
        event: dnd2024:turn.end
        subject: application.target
        steps:
        - key: save
          operation: splat:test
          kind: save
          subject: application.target
          ability: wisdom
          against:
            binding: save_dc
          rule: dnd2024:saving_throw
          on_success:
          - key: end
            operation: splat:end_effect
            target_effect:
              context: application.target_effect
            reason: successful_save
---

# Controller Warden

A homebrew test creature. Its package definition never stores damage, spent
charges, or other state from a run.

**Strike.** `splat:use strike` · `splat:roll strike.attack`

**Mend.** `splat:use mend` · `splat:roll mend.healing`

**Protect Ally.** `splat:use protect` is enabled only at a valid reaction opportunity.

**Ember Wave.** `splat:use ember_wave`

<!-- splat:inventory -->

| Key | Item | Quantity | Equipment | Attuned |
| --- | --- | --- | --- | --- |
| armor | `my_homebrew:items/scale_mail` | 1 | worn | false |
| spear | `my_homebrew:items/spear` | 1 | wielded | false |
| shield | `my_homebrew:items/shield` | 1 | carried | false |
| potions | `my_homebrew:items/healing_draught` | 3 | carried | false |
| wand_a | `my_homebrew:items/dual_wand` | 1 | carried | false |
| wand_b | `my_homebrew:items/dual_wand` | 1 | carried | false |
| pack | `my_homebrew:containers/supply_pack` | 1 | worn | false |

The supply pack supplies its own contents. There is no local `container=pack`
table to merge with it. Item controls are rendered with the selected inventory
instance as their source; a carried wand must be wielded before use.

**Frighten.** `splat:use frighten`

This homebrew action applies fear to up to two chosen creatures without an
initial save, then permits the specified end-of-turn saves. One use owns the
shared Concentration and separate target records. It is not an official spell.
