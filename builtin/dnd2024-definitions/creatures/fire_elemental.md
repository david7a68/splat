---
splat_schema: 1
splat_id: 14fca385-727c-5c85-a854-f79578a20735
splat_kind: creature
splat_key: dnd2024:creatures/fire_elemental
splat_name: Fire Elemental
splat_origin:
  key: dnd2024:reference/creature/fire_elemental
splat_sources:
- title: SRD 5.2.1
  locator: pp. 284, 285
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: large
  type: elemental
  alignment: Neutral
  movement_text: 50 ft.
  resistances_text: Bludgeoning, Piercing, Slashing
  immunities_text: Fire, Poison; Exhaustion, Grappled, Paralyzed, Petrified, Poisoned, Prone, Restrained,
    Unconscious
  senses_text: Darkvision 60 ft.; Passive Perception 10
  languages_text: Primordial (Ignan)
  cr_text: 5 (XP 1,800; PB +3)
  hit_points_text: 93 (11d10 + 33)
splat_stats:
  strength:
    value: 10
  save_strength:
    value: 0
  dexterity:
    value: 17
  save_dexterity:
    value: 3
  constitution:
    value: 16
  save_constitution:
    value: 3
  intelligence:
    value: 6
  save_intelligence:
    value: -2
  wisdom:
    value: 10
  save_wisdom:
    value: 0
  charisma:
    value: 7
  save_charisma:
    value: -2
  armor_class:
    value: 13
  max_hp:
    value: 93
  initiative:
    value: 3
  speed:
    value: 50
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/fire_elemental
  implementation: dnd2024:creature_fire_elemental
  parameters:
    damage_resistances:
    - bludgeoning
    - piercing
    - slashing
    damage_immunities:
    - fire
    - poison
    condition_immunities:
    - exhaustion
    - grappled
    - paralyzed
    - petrified
    - poisoned
    - prone
    - restrained
    - unconscious
    senses:
    - sense: darkvision
      range_ft: 60
    passive_perception: 10
    languages:
      speak: true
      known:
      - primordial
      primordial_dialects:
      - ignan
    features:
      fire_form:
        economy: passive
        op: movement.form
        minimum_opening_inches: 1
        extra_movement_cost: 0
        can_enter_and_end_in_creature_space: true
        on_first_entry_per_creature_per_turn:
        - op: damage
          components:
          - type: fire
            amount:
              dice:
                count: 1
                sides: 10
              add: 0
            average: 5
      illumination:
        economy: passive
        op: light.emit
        bright_radius_ft: 30
        dim_additional_ft: 30
      burn:
        economy: action
        op: attack
        modes:
        - melee
        attack_bonus: 6
        target:
          kind: creature_or_object
          count: 1
        reach_ft: 5
        hit:
        - op: damage
          components:
          - type: fire
            amount:
              dice:
                count: 2
                sides: 6
              add: 3
            average: 10
        - op: burning.start
          when_any:
          - target_kind: creature
          - target_kind: object
            flammable: true
      fire_aura:
        economy: passive
        trigger: actor.turn_end
        targets:
          kind: creature
          count: all
          area:
            shape: emanation
            origin: actor
            size_ft: 10
        effects:
        - op: damage
          components:
          - type: fire
            amount:
              dice:
                count: 1
                sides: 10
              add: 0
            average: 5
        additional_targets:
          kind:
          - creature
          - flammable_object
          area:
            shape: emanation
            origin: actor
            size_ft: 10
          effect:
            op: burning.start
      water_susceptibility:
        economy: passive
        triggers:
        - event: actor.moves_in_water
          per_distance_ft: 5
        - event: water.splashed_on_actor
          per_volume_gallons: 1
        effects:
        - op: damage
          components:
          - type: cold
            amount:
              dice:
                count: 1
                sides: 6
              add: 0
            average: 3
      multiattack:
        economy: action
        op: multiattack
        sequence:
        - count: 2
          feature: burn
    damage_resolution:
      choice:
      - printed_average
      - roll_expression
      chooser: controller
      exclusive: true
    spatial_resolution:
      provider: caller
      required_facts:
      - distance
      - area_membership
      - visibility
      - occupancy
      - movement_path
      validate_against_typed_constraints: true
splat_contracts:
- id: dnd2024:creature_fire_elemental
  inputs:
    actor:
      type: participant
      required: true
    targets:
      required: false
      type: participant_list
    objects:
      required: false
      type: object_list
    choices:
      required: false
      type: choice
    scene:
      required: false
      type: resolution
    event:
      required: false
      type: event
  outputs:
  - effects
  - roll_requests
  - state_changes
  events:
  - actor.moves_in_water
  - actor.turn_end
  - creature.feature_requested
  - water.splashed_on_actor
  capabilities:
  - action_economy
  - attack_resolution
  - creature_rules
  - damage
  - effects
  - spatial_constraints
splat_conversion:
  status: specified
  category: creature
  source_sha256: e56ec69e5bb881da184bba8a4a9cb1369bb7e42a76507ef40a55f01884b6cb27
  source_path: builtin/srd-5.2.1/creature/fire_elemental.md
  pages:
  - 284
  - 285
  engine_gaps: &id001
  - dnd2024:creature_fire_elemental
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_3.md
  specification_sha256: 47e3c1336e204f7fe12073af63a0a62298239c8f96d4fc46eceb08a3f2bbf843
splat_clauses:
- key: source_rules
  text: '# Fire Elemental Large Elemental, Neutral AC 13 Initiative +3 (13) HP 93 (11d10 + 33) Speed 50
    ft. MOD SAVE MOD SAVE MOD SAVE Str 10 +0 +0 Dex 17 +3 +3 Con 16 +3 +3 Int 6 -2 -2 Wis 10 +0 +0 Cha
    7 -2 -2 Resistances Bludgeoning, Piercing, Slashing Immunities Fire, Poison; Exhaustion, Grappled,
    Paralyzed, Petrified, Poisoned, Prone, Restrained, Unconscious Senses Darkvision 60 ft.; Passive Perception
    10 Languages Primordial (Ignan) CR 5 (XP 1,800; PB +3) ### Traits Fire Aura. At the end of each of
    the elemental’s turns, each creature in a 10-foot Emanation originating from the elemental takes 5
    (1d10) Fire damage. Creatures and flammable objects in the Emanation start burning. Fire Form. The
    elemental can move through a space as narrow as 1 inch without expending extra movement to do so,
    and it can enter a creature’s space and stop there. The first time it enters a creature’s space on
    a turn, that creature takes 5 (1d10) Fire damage. Illumination. The elemental sheds Bright Light in
    a 30- foot radius and Dim Light for an additional 30 feet. Water Susceptibility. The elemental takes
    3 (1d6) Cold damage for every 5 feet the elemental moves in water or for every gallon of water splashed
    on it. ### Actions Multiattack. The elemental makes two Burn attacks. Burn. Melee Attack Roll: +6,
    reach 5 ft. Hit: 10 (2d6 + 3) Fire damage. If the target is a creature or a flammable object, it starts
    burning.'
  sha256: 0d99465468b4c3ce5e01c50a4c5edc06b773f7605154088fb2842f31eeee0158
  status: contract_specified
splat_requires_mechanics: *id001
---

# Fire Elemental

<!-- SRD page 284 -->

Large Elemental, Neutral\
AC 13 Initiative +3 (13)\
HP 93 (11d10 + 33)\
Speed 50 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 10 +0 +0 Dex 17 +3 +3 Con 16 +3 +3\
Int 6 −2 −2 Wis 10 +0 +0 Cha 7 −2 −2\
Resistances Bludgeoning, Piercing, Slashing\
Immunities Fire, Poison; Exhaustion, Grappled, Paralyzed,\
Petrified, Poisoned, Prone, Restrained, Unconscious\
Senses Darkvision 60 ft.; Passive Perception 10\
Languages Primordial (Ignan)\
CR 5 (XP 1,800; PB +3)\

### Traits

Fire Aura. At the end of each of the elemental’s turns,\
each creature in a 10-foot Emanation originating from\
the elemental takes 5 (1d10) Fire damage. Creatures and\
flammable objects in the Emanation start burning.\
Fire Form. The elemental can move through a space as\
narrow as 1 inch without expending extra movement\
to do so, and it can enter a creature’s space and stop\
there. The first time it enters a creature’s space on a\
turn, that creature takes 5 (1d10) Fire damage.\
Illumination. The elemental sheds Bright Light in a 30-\
foot radius and Dim Light for an additional 30 feet.\

<!-- SRD page 285 -->

Water Susceptibility. The elemental takes 3 (1d6) Cold\
damage for every 5 feet the elemental moves in water\
or for every gallon of water splashed on it.\

### Actions

Multiattack. The elemental makes two Burn attacks.\
Burn. Melee Attack Roll: +6, reach 5 ft. Hit: 10 (2d6 +\
3) Fire damage. If the target is a creature or a flammable\
object, it starts burning.\

Original layout: [[../../srd-5.2.1/pages/284|page 284]], [[../../srd-5.2.1/pages/285|page 285]].
