---
splat_schema: 1
splat_id: 5a1e2ab8-e9aa-5c8b-ba09-8d5ccbbfc4da
splat_kind: creature
splat_key: dnd2024:creatures/troll
splat_name: Troll
splat_origin:
  key: dnd2024:reference/creature/troll
splat_sources:
- title: SRD 5.2.1
  locator: pp. 333
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: large
  type: giant
  alignment: Chaotic Evil
  movement_text: 30 ft.
  skills_text: Perception +5
  senses_text: Darkvision 60 ft.; Passive Perception 15
  languages_text: Giant
  cr_text: 5 (XP 1,800; PB +3)
  hit_points_text: 94 (9d10 + 45)
splat_stats:
  strength:
    value: 18
  save_strength:
    value: 4
  dexterity:
    value: 13
  save_dexterity:
    value: 1
  constitution:
    value: 20
  save_constitution:
    value: 5
  intelligence:
    value: 7
  save_intelligence:
    value: -2
  wisdom:
    value: 9
  save_wisdom:
    value: -1
  charisma:
    value: 7
  save_charisma:
    value: -2
  armor_class:
    value: 15
  max_hp:
    value: 94
  initiative:
    value: 1
  speed:
    value: 30
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/troll
  implementation: dnd2024:creature_troll
  parameters:
    features:
      loathsome_limbs:
        economy: trait
        resource:
          kind: daily
          uses: 4
          recovery: long_rest
        operation: passive_rule
        trigger: any.turn_end
        requires:
          self_bloodied: true
          slashing_damage_this_turn_at_least: 15
        sever_limb:
          spawn:
            key: dnd2024:creatures/troll_limb
          location: self.space
          turn: immediately_after_self
        exhaustion_per_missing_limb: 1
        regrow:
          event: self.hp_regained
          all_missing: true
      regeneration:
        economy: trait
        operation: passive_rule
        trigger: self.turn_start
        heal: 15
        suppressed_next_turn_by:
        - acid_damage
        - fire_damage
        death_rule:
          only_if:
            turn_start_hp: 0
            does_not_regenerate: true
      multiattack:
        economy: action
        operation: multiattack
        sequence:
        - operation: attack_sequence
          feature: rend
          count: 3
      rend:
        economy: action
        operation: attack
        attack:
          mode: melee
          targets:
            count: 1
            kind: creature_or_object
          supplied_attack_total: 7
          reach_ft: 10
        hit:
          damage:
          - average: 11
            roll:
              count: 2
              sides: 6
              modifier: 4
            type: slashing
      charge:
        economy: bonus_action
        operation: move
        maximum_ft:
          multiply:
          - stat: speed
          - 0.5
        direction: straight_toward_enemy
        enemy_visible: true
    skill_totals:
      perception: 5
    senses:
      darkvision: 60
      passive_perception: 15
    languages:
      speaks:
      - giant
      understands: []
    resolution_conventions:
      distance_unit: foot
      duration_unit: second
      damage_roll_mode:
        choice:
        - listed_average
        - roll_dice
        chooser: gm
      supplied_totals: before_additional_effect_modifiers
      multiattack_nested_economy_cost: 0
      nested_feature_resource_cost: normal
      spell_level_unless_specified: lowest_possible
      spell_upcast_without_permission: false
      spell_casting_time_at_least_one_minute: normal_casting_procedure
      geometry:
        input: adjudication
        assumptions:
        - range_and_visibility_verified
        - area_membership_verified
        - path_and_destination_legal
      unlisted_success_branch: no_effect
      unlisted_duration: until_removed_by_applicable_rule
splat_contracts:
- id: dnd2024:creature_troll
  inputs:
    actor:
      type: participant
      required: true
    feature:
      required: false
      type: choice
    targets:
      required: false
      type: participant_list
    objects:
      required: false
      type: object_list
    destination:
      required: false
      type: location
    adjudication:
      required: false
      type: resolution
    trigger:
      required: false
      type: event
  outputs:
  - effects
  events:
  - creature.feature_requested
  - turn.started
  - turn.ended
  - rest.completed
  capabilities:
  - creature_features
  - action_economy
  - attacks
  - damage
  - creature_transformation
  - spatial_queries
  - limited_resources
  - hit_points
splat_conversion:
  status: specified
  category: creature
  source_sha256: 53c9673b2bb5c7f3a79c657a581e86ed0457a012efdb46c6fe911f102a54bfaa
  source_path: builtin/srd-5.2.1/creature/troll.md
  pages:
  - 333
  engine_gaps: &id001
  - dnd2024:creature_troll
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_1.md
  specification_sha256: c1db20fcf4a9f731633d1d4d096fc22eaba9be71243c120cf38e0441da47cab1
splat_clauses:
- key: source_rules
  text: '# Troll Large Giant, Chaotic Evil AC 15 Initiative +1 (11) HP 94 (9d10 + 45) Speed 30 ft. MOD
    SAVE MOD SAVE MOD SAVE Str 18 +4 +4 Dex 13 +1 +1 Con 20 +5 +5 Int 7 -2 -2 Wis 9 -1 -1 Cha 7 -2 -2
    Skills Perception +5 Senses Darkvision 60 ft.; Passive Perception 15 Languages Giant CR 5 (XP 1,800;
    PB +3) ### Traits Loathsome Limbs (4/Day). If the troll ends any turn Bloodied and took 15+ Slashing
    damage during that turn, one of the troll’s limbs is severed, falls into the troll’s space, and becomes
    a Troll Limb. The limb acts immediately after the troll’s turn. The troll has 1 Exhaustion level for
    each missing limb, and it grows replacement limbs the next time it regains Hit Points. Regeneration.
    The troll regains 15 Hit Points at the start of each of its turns. If the troll takes Acid or Fire
    damage, this trait doesn’t function on the troll’s next turn. The troll dies only if it starts its
    turn with 0 Hit Points and doesn’t regenerate. ### Actions Multiattack. The troll makes three Rend
    attacks. Rend. Melee Attack Roll: +7, reach 10 ft. Hit: 11 (2d6 + 4) Slashing damage. ### Bonus Actions
    Charge. The troll moves up to half its Speed straight toward an enemy it can see.'
  sha256: 4b6a628974e857937ee2258b28df50c1e6a3904b41676d8d5a4b952a95f4f833
  status: contract_specified
splat_requires_mechanics: *id001
---

# Troll

<!-- SRD page 333 -->

Large Giant, Chaotic Evil\
AC 15 Initiative +1 (11)\
HP 94 (9d10 + 45)\
Speed 30 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 18 +4 +4 Dex 13 +1 +1 Con 20 +5 +5\
Int 7 −2 −2 Wis 9 −1 −1 Cha 7 −2 −2\
Skills Perception +5\
Senses Darkvision 60 ft.; Passive Perception 15\
Languages Giant\
CR 5 (XP 1,800; PB +3)\

### Traits

Loathsome Limbs (4/Day). If the troll ends any turn\
Bloodied and took 15+ Slashing damage during that\
turn, one of the troll’s limbs is severed, falls into the\
troll’s space, and becomes a Troll Limb. The limb acts\
immediately after the troll’s turn. The troll has 1 Exhaus-\
tion level for each missing limb, and it grows replace-\
ment limbs the next time it regains Hit Points.\
Regeneration. The troll regains 15 Hit Points at the start\
of each of its turns. If the troll takes Acid or Fire dam-\
age, this trait doesn’t function on the troll’s next turn.\
The troll dies only if it starts its turn with 0 Hit Points\
and doesn’t regenerate.\

### Actions

Multiattack. The troll makes three Rend attacks.\
Rend. Melee Attack Roll: +7, reach 10 ft. Hit: 11 (2d6\
+ 4) Slashing damage.\

### Bonus Actions

Charge. The troll moves up to half its Speed straight\
toward an enemy it can see.\

Original layout: [[../../srd-5.2.1/pages/333|page 333]].
