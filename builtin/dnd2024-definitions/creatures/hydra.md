---
splat_schema: 1
splat_id: b5a5c587-b6b9-5e9e-9dd9-ef58d6d3321d
splat_kind: creature
splat_key: dnd2024:creatures/hydra
splat_name: Hydra
splat_origin:
  key: dnd2024:reference/creature/hydra
splat_sources:
- title: SRD 5.2.1
  locator: pp. 300
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: huge
  type: monstrosity
  alignment: Unaligned
  movement_text: 40 ft., Swim 40 ft.
  skills_text: Perception +6
  immunities_text: Blinded, Charmed, Deafened, Frightened, Stunned, Unconscious
  senses_text: Darkvision 60 ft.; Passive Perception 16
  languages_text: None
  cr_text: 8 (XP 3,900; PB +3)
  hit_points_text: 184 (16d12 + 80)
splat_stats:
  strength:
    value: 20
  save_strength:
    value: 5
  dexterity:
    value: 12
  save_dexterity:
    value: 1
  constitution:
    value: 20
  save_constitution:
    value: 5
  intelligence:
    value: 2
  save_intelligence:
    value: -4
  wisdom:
    value: 10
  save_wisdom:
    value: 0
  charisma:
    value: 7
  save_charisma:
    value: -2
  armor_class:
    value: 15
  max_hp:
    value: 184
  initiative:
    value: 4
  speed:
    value: 40
    unit: ft
  swim_speed:
    value: 40
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/hydra
  implementation: dnd2024:creature_hydra
  parameters:
    features:
      hold_breath:
        economy: trait
        operation: passive_rule
        hold_breath_seconds: 3600
      multiple_heads:
        economy: trait
        operation: passive_rule
        initial_heads: 5
        head_loss:
          trigger: damage_on_single_turn
          threshold: 25
          heads_lost: 1
        dies_if_heads: 0
        regrow:
          event: self.turn_end
          requires_living_head: true
          unless_fire_damage_since_last_turn: true
          heads_per_head_lost_since_last_turn: 2
          heal_per_regrowth_event: 20
      reactive_heads:
        economy: trait
        operation: passive_rule
        extra_reaction_capacity:
          max:
          - 0
          - subtract:
            - stat: heads
            - 1
        refresh_event: self.turn_start
        recompute_only_at_refresh: true
        allowed_uses:
        - opportunity_attack
      multiattack:
        economy: action
        operation: multiattack
        sequence:
        - operation: attack_sequence
          feature: bite
          count:
            stat: heads
      bite:
        economy: action
        operation: attack
        attack:
          mode: melee
          targets:
            count: 1
            kind: creature_or_object
          supplied_attack_total: 8
          reach_ft: 10
        hit:
          damage:
          - average: 10
            roll:
              count: 1
              sides: 10
              modifier: 5
            type: piercing
    skill_totals:
      perception: 6
    damage_immunities: []
    condition_immunities:
    - blinded
    - charmed
    - deafened
    - frightened
    - stunned
    - unconscious
    senses:
      darkvision: 60
      passive_perception: 16
    languages:
      speaks: []
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
- id: dnd2024:creature_hydra
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
  capabilities:
  - creature_features
  - action_economy
  - attacks
  - conditions
  - damage
  - creature_transformation
  - spatial_queries
  - limited_resources
  - defenses
splat_conversion:
  status: specified
  category: creature
  source_sha256: 96505c3a52429bac9d95003be408df5a3deb0dd3d4abdf1745eb6afeda326843
  source_path: builtin/srd-5.2.1/creature/hydra.md
  pages:
  - 300
  engine_gaps: &id001
  - dnd2024:creature_hydra
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_1.md
  specification_sha256: 8dd9f43d3f2d86a0c69dfb118b1682966ad637371f2d294a53afd82dcd8ae639
splat_clauses:
- key: source_rules
  text: '# Hydra Huge Monstrosity, Unaligned AC 15 Initiative +4 (14) HP 184 (16d12 + 80) Speed 40 ft.,
    Swim 40 ft. MOD SAVE MOD SAVE MOD SAVE Str 20 +5 +5 Dex 12 +1 +1 Con 20 +5 +5 Int 2 -4 -4 Wis 10 +0
    +0 Cha 7 -2 -2 Skills Perception +6 Immunities Blinded, Charmed, Deafened, Frightened, Stunned, Unconscious
    Senses Darkvision 60 ft.; Passive Perception 16 Languages None CR 8 (XP 3,900; PB +3) ### Traits Hold
    Breath. The hydra can hold its breath for 1 hour. Multiple Heads. The hydra has five heads. Whenever
    the hydra takes 25 damage or more on a single turn, one of its heads dies. The hydra dies if all its
    heads are dead. At the end of each of its turns when it has at least one living head, the hydra grows
    two heads for each of its heads that died since its last turn, unless it has taken Fire damage since
    its last turn. The hydra regains 20 Hit Points when it grows new heads. Reactive Heads. For each head
    the hydra has beyond one, it gets an extra Reaction that can be used only for Opportunity Attacks.
    ### Actions Multiattack. The hydra makes as many Bite attacks as it has heads. Bite. Melee Attack
    Roll: +8, reach 10 ft. Hit: 10 (1d10 + 5) Piercing damage.'
  sha256: faa03386c63f4afa2c6a7a294353f6eb7f70756e81af2a0b15829078a63ddc91
  status: contract_specified
splat_requires_mechanics: *id001
---

# Hydra

<!-- SRD page 300 -->

Huge Monstrosity, Unaligned\
AC 15 Initiative +4 (14)\
HP 184 (16d12 + 80)\
Speed 40 ft., Swim 40 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 20 +5 +5 Dex 12 +1 +1 Con 20 +5 +5\
Int 2 −4 −4 Wis 10 +0 +0 Cha 7 −2 −2\
Skills Perception +6\
Immunities Blinded, Charmed, Deafened, Frightened,\
Stunned, Unconscious\
Senses Darkvision 60 ft.; Passive Perception 16\
Languages None\
CR 8 (XP 3,900; PB +3)\

### Traits

Hold Breath. The hydra can hold its breath for 1 hour.\
Multiple Heads. The hydra has five heads. Whenever\
the hydra takes 25 damage or more on a single turn,\
one of its heads dies. The hydra dies if all its heads are\
dead. At the end of each of its turns when it has at least\
one living head, the hydra grows two heads for each of\
its heads that died since its last turn, unless it has taken\
Fire damage since its last turn. The hydra regains 20 Hit\
Points when it grows new heads.\
Reactive Heads. For each head the hydra has beyond\
one, it gets an extra Reaction that can be used only for\
Opportunity Attacks.\

### Actions

Multiattack. The hydra makes as many Bite attacks as it\
has heads.\
Bite. Melee Attack Roll: +8, reach 10 ft. Hit: 10 (1d10\
+ 5) Piercing damage.\

Original layout: [[../../srd-5.2.1/pages/300|page 300]].
