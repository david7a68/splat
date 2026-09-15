---
splat_schema: 1
splat_id: 03807394-3280-5142-bdda-03ebdf020faa
splat_kind: creature
splat_key: dnd2024:creatures/ghost
splat_name: Ghost
splat_origin:
  key: dnd2024:reference/creature/ghost
splat_sources:
- title: SRD 5.2.1
  locator: pp. 287, 288
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: medium
  type: undead
  alignment: Neutral
  movement_text: 5 ft., Fly 40 ft. (hover)
  resistances_text: Acid, Bludgeoning, Cold, Fire, Lightning, Piercing, Slashing, Thunder
  immunities_text: Necrotic, Poison; Charmed, Exhaustion, Frightened, Grappled, Paralyzed, Petrified,
    Poisoned, Prone, Restrained
  senses_text: Darkvision 60 ft.; Passive Perception 11
  languages_text: Common plus one other language
  cr_text: 4 (XP 1,100; PB +2)
  hit_points_text: 45 (10d8)
splat_stats:
  strength:
    value: 7
  save_strength:
    value: -2
  dexterity:
    value: 13
  save_dexterity:
    value: 1
  constitution:
    value: 10
  save_constitution:
    value: 0
  intelligence:
    value: 10
  save_intelligence:
    value: 0
  wisdom:
    value: 12
  save_wisdom:
    value: 1
  charisma:
    value: 17
  save_charisma:
    value: 3
  armor_class:
    value: 11
  max_hp:
    value: 45
  initiative:
    value: 1
  speed:
    value: 5
    unit: ft
  fly_speed:
    value: 40
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/ghost
  implementation: dnd2024:creature_ghost
  parameters:
    damage_resistances:
    - acid
    - bludgeoning
    - cold
    - fire
    - lightning
    - piercing
    - slashing
    - thunder
    damage_immunities:
    - necrotic
    - poison
    condition_immunities:
    - charmed
    - exhaustion
    - frightened
    - grappled
    - paralyzed
    - petrified
    - poisoned
    - prone
    - restrained
    senses:
    - sense: darkvision
      range_ft: 60
    passive_perception: 11
    languages:
      speak: true
      known:
      - common
      additional_choice_count: 1
    features:
      ethereal_sight:
        economy: passive
        op: vision.cross_plane
        from: material
        to: ethereal
        range_ft: 60
      incorporeal_movement:
        economy: passive
        op: movement.pass_through
        entities:
        - creature
        - object
        cost: difficult_terrain
        end_turn_inside_object:
        - op: damage
          components:
          - type: force
            amount:
              dice:
                count: 1
                sides: 10
              add: 0
            average: 5
      withering_touch:
        economy: action
        op: attack
        modes:
        - melee
        attack_bonus: 5
        target:
          kind: creature_or_object
          count: 1
        reach_ft: 5
        hit:
        - op: damage
          components:
          - type: necrotic
            amount:
              dice:
                count: 3
                sides: 10
              add: 3
            average: 19
      etherealness:
        economy: action
        op: spell.cast
        spell:
          key: dnd2024:spells/etherealness
        ability: charisma
        waived_components:
        - verbal
        - somatic
        - material
        overrides:
          visible_across_planes:
          - material
          - border_ethereal
          cross_plane_effects_allowed: false
      horrific_visage:
        economy: action
        op: saving_throw
        ability: wisdom
        dc: 13
        targets:
          kind: creature
          count: all
          area:
            shape: cone
            origin: actor
            size_ft: 60
          must_see_actor: true
          exclude_creature_types:
          - undead
        failure:
        - op: damage
          components:
          - type: psychic
            amount:
              dice:
                count: 2
                sides: 6
              add: 3
            average: 10
        - op: condition.apply
          condition: frightened
          duration:
            until: actor.next_turn_start
        success:
        - op: feature.immunity
          source: actor
          feature: horrific_visage
          duration:
            amount: 24
            unit: hour
      possession:
        economy: action
        op: saving_throw
        ability: charisma
        dc: 13
        targets:
          kind: creature
          count: 1
          range_ft: 5
          visible_to_actor: true
          creature_types:
          - humanoid
        failure:
        - op: possession.start
          actor_disappears: true
          target_conditions:
          - incapacitated
          body_control: actor
          target_retains_awareness: true
          actor_targetable_only_by: effects_specifically_targeting_undead
          actor_stat_overrides_from_target:
          - speed
          - strength_modifier
          - dexterity_modifier
          - constitution_modifier
          end_when_any:
          - body_hp_zero
          - actor.bonus_action_leave
          end_effects:
          - op: actor.appear
            within_target_ft: 5
            unoccupied: true
          - op: feature.immunity
            source: actor
            feature: possession
            duration:
              amount: 24
              unit: hour
        success:
        - op: feature.immunity
          source: actor
          feature: possession
          duration:
            amount: 24
            unit: hour
        leave_economy: bonus_action
        recharge:
          die_sides: 6
          success_min: 6
          success_max: 6
          check_event: actor.turn_start
          maximum_uses: 1
          initial_uses: 1
          also_recovers_after:
          - short_rest
          - long_rest
      multiattack:
        economy: action
        op: multiattack
        sequence:
        - count: 2
          feature: withering_touch
    damage_resolution:
      choice:
      - printed_average
      - roll_expression
      chooser: controller
      exclusive: true
    spellcasting_defaults:
      cast_level: lowest_possible_unless_overridden
      upcast_allowed: only_explicit_cast_level_overrides
      required_material_components_available: true
      casting_time_at_least_1_minute:
        uses_normal_time: true
        magic_action_each_turn: true
        concentration_required: true
      other_components: spell_definition_unless_waived
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
- id: dnd2024:creature_ghost
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
  - actor.turn_start
  - creature.feature_requested
  capabilities:
  - action_economy
  - attack_resolution
  - creature_lifecycle
  - creature_rules
  - damage
  - effects
  - resources
  - rest_recovery
  - saving_throws
  - spatial_constraints
  - spellcasting
splat_conversion:
  status: specified
  category: creature
  source_sha256: c9a20acc0e211c82308ece00e28fe4dd53c4437ec9360718ddbc81a66b73790e
  source_path: builtin/srd-5.2.1/creature/ghost.md
  pages:
  - 287
  - 288
  engine_gaps: &id001
  - dnd2024:creature_ghost
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_3.md
  specification_sha256: 413142b850fecf99d264f912784079fd6af2a92af7c52f4055f6d96d35a104e8
splat_clauses:
- key: source_rules
  text: '# Ghost Medium Undead, Neutral AC 11 Initiative +1 (11) HP 45 (10d8) Speed 5 ft., Fly 40 ft.
    (hover) MOD SAVE MOD SAVE MOD SAVE Str 7 -2 -2 Dex 13 +1 +1 Con 10 +0 +0 Int 10 +0 +0 Wis 12 +1 +1
    Cha 17 +3 +3 Resistances Acid, Bludgeoning, Cold, Fire, Lightning, Piercing, Slashing, Thunder Immunities
    Necrotic, Poison; Charmed, Exhaustion, Frightened, Grappled, Paralyzed, Petrified, Poisoned, Prone,
    Restrained Senses Darkvision 60 ft.; Passive Perception 11 Languages Common plus one other language
    CR 4 (XP 1,100; PB +2) ### Traits Ethereal Sight. The ghost can see 60 feet into the Ethereal Plane
    when it is on the Material Plane. Incorporeal Movement. The ghost can move through other creatures
    and objects as if they were Difficult Terrain. It takes 5 (1d10) Force damage if it ends its turn
    inside an object. ### Actions Multiattack. The ghost makes two Withering Touch attacks. Withering
    Touch. Melee Attack Roll: +5, reach 5 ft. Hit: 19 (3d10 + 3) Necrotic damage. Etherealness. The ghost
    casts the Etherealness spell, requiring no spell components and using Charisma as the spellcasting
    ability. The ghost is visible on the Material Plane while on the Border Ethereal and vice versa, but
    it can’t affect or be affected by anything on the other plane. Horrific Visage. Wisdom Saving Throw:
    DC 13, each creature in a 60-foot Cone that can see the ghost and isn’t an Undead. Failure: 10 (2d6
    + 3) Psychic damage, and the target has the Frightened condition until the start of the ghost’s next
    turn. Success: The target is immune to this ghost’s Horrific Visage for 24 hours. Possession (Recharge
    6). Charisma Saving Throw: DC 13, one Humanoid the ghost can see within 5 feet. Failure: The target
    is possessed by the ghost; the ghost disappears, and the target has the Incapacitated condition and
    loses control of its body. The ghost now controls the body, but the target retains awareness. The
    ghost can’t be targeted by any attack, spell, or other effect, except ones that specifically target
    Undead. The ghost’s game statistics are the same, except it uses the possessed target’s Speed, as
    well as the target’s Strength, Dexterity, and Constitution modifiers. The possession lasts until the
    body drops to 0 Hit Points or the ghost leaves as a Bonus Action. When the possession ends, the ghost
    appears in an unoccupied space within 5 feet of the target, and the target is immune to this ghost’s
    Possession for 24 hours. Success: The target is immune to this ghost’s Possession for 24 hours.'
  sha256: 786e0503e53e1add3b87aad43b328d382192eb17cda4da47244b1722e5a7af9a
  status: contract_specified
splat_requires_mechanics: *id001
---

# Ghost

<!-- SRD page 287 -->

Medium Undead, Neutral\
AC 11 Initiative +1 (11)\
HP 45 (10d8)\
Speed 5 ft., Fly 40 ft. (hover)\
MOD SAVE MOD SAVE MOD SAVE\
Str 7 −2 −2 Dex 13 +1 +1 Con 10 +0 +0\
Int 10 +0 +0 Wis 12 +1 +1 Cha 17 +3 +3\
Resistances Acid, Bludgeoning, Cold, Fire, Lightning,\
Piercing, Slashing, Thunder\
Immunities Necrotic, Poison; Charmed, Exhaustion,\
Frightened, Grappled, Paralyzed, Petrified, Poisoned,\
Prone, Restrained\
Senses Darkvision 60 ft.; Passive Perception 11\
Languages Common plus one other language\
CR 4 (XP 1,100; PB +2)\

### Traits

Ethereal Sight. The ghost can see 60 feet into the Ethe-\
real Plane when it is on the Material Plane.\
Incorporeal Movement. The ghost can move through\
other creatures and objects as if they were Difficult\
Terrain. It takes 5 (1d10) Force damage if it ends its turn\
inside an object.\

<!-- SRD page 288 -->


### Actions

Multiattack. The ghost makes two Withering\
Touch attacks.\
Withering Touch. Melee Attack Roll: +5, reach 5 ft.\
Hit: 19 (3d10 + 3) Necrotic damage.\
Etherealness. The ghost casts the Etherealness spell,\
requiring no spell components and using Charisma as\
the spellcasting ability. The ghost is visible on the Mate-\
rial Plane while on the Border Ethereal and vice versa,\
but it can’t affect or be affected by anything on the\
other plane.\
Horrific Visage. Wisdom Saving Throw: DC 13, each\
creature in a 60-foot Cone that can see the ghost and\
isn’t an Undead. Failure: 10 (2d6 + 3) Psychic damage,\
and the target has the Frightened condition until the\
start of the ghost’s next turn. Success: The target is im-\
mune to this ghost’s Horrific Visage for 24 hours.\
Possession (Recharge 6). Charisma Saving Throw: DC\
13, one Humanoid the ghost can see within 5 feet. Fail-\
ure: The target is possessed by the ghost; the ghost dis-\
appears, and the target has the Incapacitated condition\
and loses control of its body. The ghost now controls\
the body, but the target retains awareness. The ghost\
can’t be targeted by any attack, spell, or other effect,\
except ones that specifically target Undead. The ghost’s\
game statistics are the same, except it uses the pos-\
sessed target’s Speed, as well as the target’s Strength,\
Dexterity, and Constitution modifiers.\
The possession lasts until the body drops to 0 Hit\
Points or the ghost leaves as a Bonus Action. When the\
possession ends, the ghost appears in an unoccupied\
space within 5 feet of the target, and the target is im-\
mune to this ghost’s Possession for 24 hours.\
Success: The target is immune to this ghost’s Posses-\
sion for 24 hours.\

Original layout: [[../../srd-5.2.1/pages/287|page 287]], [[../../srd-5.2.1/pages/288|page 288]].
