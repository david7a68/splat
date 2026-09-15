---
splat_schema: 1
splat_id: 2268e2e6-8129-53ff-8c67-b9ce03805d92
splat_kind: creature
splat_key: dnd2024:creatures/black_pudding
splat_name: Black Pudding
splat_origin:
  key: dnd2024:reference/creature/black_pudding
splat_sources:
- title: SRD 5.2.1
  locator: pp. 265, 266
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: large
  type: ooze
  alignment: Unaligned
  movement_text: 20 ft., Climb 20 ft.
  immunities_text: Acid, Cold, Lightning, Slashing; Charmed, Deafened, Exhaustion, Frightened, Grappled,
    Prone, Restrained
  senses_text: Blindsight 60 ft.; Passive Perception 8
  languages_text: None
  cr_text: 4 (XP 1,100; PB +2)
  hit_points_text: 68 (8d10 + 24)
splat_stats:
  strength:
    value: 16
  save_strength:
    value: 3
  dexterity:
    value: 5
  save_dexterity:
    value: -3
  constitution:
    value: 16
  save_constitution:
    value: 3
  intelligence:
    value: 1
  save_intelligence:
    value: -5
  wisdom:
    value: 6
  save_wisdom:
    value: -2
  charisma:
    value: 1
  save_charisma:
    value: -5
  armor_class:
    value: 7
  max_hp:
    value: 68
  initiative:
    value: -3
  speed:
    value: 20
    unit: ft
  climb_speed:
    value: 20
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/black_pudding
  implementation: dnd2024:creature_black_pudding
  parameters:
    features:
      amorphous:
        economy: trait
        operation: passive_rule
        minimum_gap_inches: 1
        extra_movement_cost: 0
      corrosive_form:
        economy: trait
        operation: passive_rule
        on_melee_attack_hit:
          attacker_damage:
          - roll:
              count: 1
              sides: 8
              modifier: 0
            type: acid
        on_ammunition_damage:
          requires_nonmagical: true
          destroy_ammunition: true
        on_weapon_damage_contact:
          requires_nonmagical: true
          cumulative_attack_penalty: -1
          destroy_at_penalty: -5
          remove_penalty_spell:
            key: dnd2024:spells/mending
        consume_material:
          materials:
          - nonmagical_wood
          - nonmagical_metal
          depth_ft: 2
          seconds: 60
      spider_climb:
        economy: trait
        operation: passive_rule
        climb_difficult_surfaces: true
        climb_ceilings: true
        ability_check_required: false
      dissolving_pseudopod:
        economy: action
        operation: attack
        attack:
          mode: melee
          targets:
            count: 1
            kind: creature_or_object
          supplied_attack_total: 5
          reach_ft: 10
        hit:
          damage:
          - average: 17
            roll:
              count: 4
              sides: 6
              modifier: 3
            type: acid
          armor_corrosion:
            worn: true
            nonmagical: true
            cumulative_ac_penalty: -1
            destroy_at_ac: 10
            remove_penalty_spell:
              key: dnd2024:spells/mending
      split:
        economy: reaction
        operation: split_creature
        trigger:
          any_of:
          - self.becomes_bloodied
          - self.subjected_to_lightning_damage
          - self.subjected_to_slashing_damage
        requires:
          size:
          - medium
          - large
          hp_minimum: 10
        remove_original: true
        offspring:
          definition:
            key: dnd2024:creatures/black_pudding
          count: 2
          size_steps: -1
          hp_each:
            floor:
              divide:
              - stat: self.current_hp
              - 2
          initiative: original
    damage_immunities:
    - acid
    - cold
    - lightning
    - slashing
    condition_immunities:
    - charmed
    - deafened
    - exhaustion
    - frightened
    - grappled
    - prone
    - restrained
    senses:
      blindsight: 60
      passive_perception: 8
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
- id: dnd2024:creature_black_pudding
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
  capabilities:
  - creature_features
  - action_economy
  - attacks
  - conditions
  - damage
  - grappling
  - spatial_queries
  - limited_resources
  - hit_points
  - defenses
splat_conversion:
  status: specified
  category: creature
  source_sha256: 6db8059ab8fd13249a1d6d64bd329bda23ef6589d557a93c39130744e16b35c2
  source_path: builtin/srd-5.2.1/creature/black_pudding.md
  pages:
  - 265
  - 266
  engine_gaps: &id001
  - dnd2024:creature_black_pudding
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_1.md
  specification_sha256: 34e65b614d79d650fd765f6bbb692372156a092a1c787ec1e98c232ad99382f3
splat_clauses:
- key: source_rules
  text: '# Black Pudding Large Ooze, Unaligned AC 7 Initiative -3 (7) HP 68 (8d10 + 24) Speed 20 ft.,
    Climb 20 ft. MOD SAVE MOD SAVE MOD SAVE Str 16 +3 +3 Dex 5 -3 -3 Con 16 +3 +3 Int 1 -5 -5 Wis 6 -2
    -2 Cha 1 -5 -5 Immunities Acid, Cold, Lightning, Slashing; Charmed, Deafened, Exhaustion, Frightened,
    Grappled, Prone, Restrained Senses Blindsight 60 ft.; Passive Perception 8 Languages None CR 4 (XP
    1,100; PB +2) ### Traits Amorphous. The pudding can move through a space as narrow as 1 inch without
    expending extra movement to do so. Corrosive Form. A creature that hits the pudding with a melee attack
    roll takes 4 (1d8) Acid damage. Nonmagical ammunition is destroyed immediately after hitting the pudding
    and dealing any damage. Any nonmagical weapon takes a cumulative -1 penalty to attack rolls immediately
    after dealing damage to the pudding and coming into contact with it. The weapon is destroyed if the
    penalty reaches -5. The penalty can be removed by casting the Mending spell on the weapon. In 1 minute,
    the pudding can eat through 2 feet of nonmagical wood or metal. Spider Climb. The pudding can climb
    difficult surfaces, including along ceilings, without needing to make an ability check. ### Actions
    Dissolving Pseudopod. Melee Attack Roll: +5, reach 10 ft. Hit: 17 (4d6 + 3) Acid damage. Nonmagical
    armor worn by the target takes a -1 penalty to the AC it offers. The armor is destroyed if the penalty
    reduces its AC to 10. The penalty can be removed by casting the Mending spell on the armor. ### Reactions
    Split. Trigger: While the pudding is Large or Medium and has 10+ Hit Points, it becomes Bloodied or
    is subjected to Lightning or Slashing damage. Response: The pudding splits into two new Black Puddings.
    Each new pudding is one size smaller than the original pudding and acts on its Initiative. The original
    pudding’s Hit Points are divided evenly between the new puddings (round down).'
  sha256: ab471b9f196e0e27854e6936c47869ecdd02cd30894620cc5fb3e242f3d53339
  status: contract_specified
splat_requires_mechanics: *id001
---

# Black Pudding

<!-- SRD page 265 -->

Large Ooze, Unaligned\
AC 7 Initiative −3 (7)\
HP 68 (8d10 + 24)\
Speed 20 ft., Climb 20 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 16 +3 +3 Dex 5 −3 −3 Con 16 +3 +3\
Int 1 −5 −5 Wis 6 −2 −2 Cha 1 −5 −5\
Immunities Acid, Cold, Lightning, Slashing; Charmed,\
Deafened, Exhaustion, Frightened, Grappled, Prone,\
Restrained\
Senses Blindsight 60 ft.; Passive Perception 8\
Languages None\
CR 4 (XP 1,100; PB +2)\

### Traits

Amorphous. The pudding can move through a space\
as narrow as 1 inch without expending extra move-\
ment to do so.\
Corrosive Form. A creature that hits the pudding with a\
melee attack roll takes 4 (1d8) Acid damage. Nonmag-\
ical ammunition is destroyed immediately after hitting\
the pudding and dealing any damage. Any nonmagical\
weapon takes a cumulative −1 penalty to attack rolls\
immediately after dealing damage to the pudding and\
coming into contact with it. The weapon is destroyed if\
the penalty reaches −5. The penalty can be removed by\
casting the Mending spell on the weapon.\
In 1 minute, the pudding can eat through 2 feet of\
nonmagical wood or metal.\
Spider Climb. The pudding can climb difficult surfaces,\
including along ceilings, without needing to make an\
ability check.\

### Actions

Dissolving Pseudopod. Melee Attack Roll: +5, reach\
10 ft. Hit: 17 (4d6 + 3) Acid damage. Nonmagical ar-\
mor worn by the target takes a −1 penalty to the AC it\
offers. The armor is destroyed if the penalty reduces its\
AC to 10. The penalty can be removed by casting the\
Mending spell on the armor.\

### Reactions

Split. Trigger: While the pudding is Large or Medium\
and has 10+ Hit Points, it becomes Bloodied or is sub-\
jected to Lightning or Slashing damage. Response: The\
pudding splits into two new Black Puddings. Each new\
pudding is one size smaller than the original pudding\
and acts on its Initiative. The original pudding’s Hit\

<!-- SRD page 266 -->

Points are divided evenly between the new puddings\
(round down).\

Original layout: [[../../srd-5.2.1/pages/265|page 265]], [[../../srd-5.2.1/pages/266|page 266]].
