---
splat_schema: 1
splat_id: b9070a54-028a-52b6-a468-1414db2a2d04
splat_kind: creature
splat_key: dnd2024:creatures/earth_elemental
splat_name: Earth Elemental
splat_origin:
  key: dnd2024:reference/creature/earth_elemental
splat_sources:
- title: SRD 5.2.1
  locator: pp. 282, 283
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: large
  type: elemental
  alignment: Neutral
  movement_text: 30 ft., Burrow 30 ft.
  vulnerabilities_text: Thunder
  immunities_text: Poison; Exhaustion, Paralyzed, Petrified, Poisoned, Unconscious
  senses_text: Darkvision 60 ft., Tremorsense 60 ft.; Passive Perception 10
  languages_text: Primordial (Terran)
  cr_text: 5 (XP 1,800; PB +3)
  hit_points_text: 147 (14d10 + 70)
splat_stats:
  strength:
    value: 20
  save_strength:
    value: 5
  dexterity:
    value: 8
  save_dexterity:
    value: -1
  constitution:
    value: 20
  save_constitution:
    value: 5
  intelligence:
    value: 5
  save_intelligence:
    value: -3
  wisdom:
    value: 10
  save_wisdom:
    value: 0
  charisma:
    value: 5
  save_charisma:
    value: -3
  armor_class:
    value: 17
  max_hp:
    value: 147
  initiative:
    value: -1
  speed:
    value: 30
    unit: ft
  burrow_speed:
    value: 30
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/earth_elemental
  implementation: dnd2024:creature_earth_elemental
  parameters:
    features:
      siege_monster:
        economy: passive
        damage_multiplier: 2
        targets:
        - object
        - structure
      slam:
        economy: action
        op: attack
        attack_modes:
        - melee
        attack_bonus: 8
        targets:
          count: 1
        hit:
        - op: damage
          average: 14
          amount:
            dice:
              count: 2
              sides: 8
            add: 5
          damage_type: bludgeoning
        reach_feet: 10
      rock_launch:
        economy: action
        op: attack
        attack_modes:
        - ranged
        attack_bonus: 8
        targets:
          count: 1
        hit:
        - op: damage
          average: 8
          amount:
            dice:
              count: 1
              sides: 6
            add: 5
          damage_type: bludgeoning
        - op: if
          test:
            target_kind: creature
            target_size_max: large
          then:
          - op: condition
            condition: prone
        range_feet: 60
      multiattack:
        economy: action
        op: repeat_choice
        count: 2
        choices:
        - feature: slam
        - feature: rock_launch
      earth_glide:
        economy: passive
        burrow_through:
          materials:
          - earth
          - stone
          magical: false
          worked: false
        disturbs_material: false
    defenses:
      vulnerabilities:
        damage_types:
        - thunder
      immunities:
        damage_types:
        - poison
        conditions:
        - exhaustion
        - paralyzed
        - petrified
        - poisoned
        - unconscious
    senses:
      passive_perception: 10
      special:
      - sense: darkvision
        range_feet: 60
      - sense: tremorsense
        range_feet: 60
    communication:
      speaks:
      - primordial
      understands:
      - primordial
      dialects:
      - terran
    stat_block_conventions:
      attack_bonus_and_skill_totals: supplied_total_before_external_modifiers
      save_dc: supplied_total
      damage_mode_choice:
      - fixed_average
      - rolled_dice
      condition_source: self
      multiattack_child_economy_cost: 0
      multiattack_child_resource_costs_retained: true
      spellcasting:
        default_level: spell_minimum
        spell_slots_expended: false
        required_materials: available
        casting_time_at_least_one_minute:
          uses_normal_casting_duration: true
          magic_action_each_turn: true
          concentration_required: true
      geometry_assumptions:
        chooser: gm
        supplies:
        - range
        - visibility
        - line_of_sight
        - area_membership
        - reachable_locations
        - unoccupied_locations
        - terrain
        - plane
        - form_appearance
splat_contracts:
- id: dnd2024:creature_earth_elemental
  inputs:
    self:
      type: participant
      required: true
    targets:
      required: false
      type: participant_list
    feature:
      type: choice
      required: true
    event:
      required: false
      type: event
    choices:
      required: false
      type: choice
    resolution:
      required: false
      type: resolution
    campaign:
      required: false
      type: campaign
  outputs:
  - resolved_feature
  - effects
  - resource_changes
  events: []
  capabilities:
  - action_economy
  - creature_rules
  - dice
  - effects
  - resources
  - targeting
splat_conversion:
  status: specified
  category: creature
  source_sha256: f14176e8e44b6a91fab539816cc9e1d24a01fb08b0324a77c7de2d4478251a78
  source_path: builtin/srd-5.2.1/creature/earth_elemental.md
  pages:
  - 282
  - 283
  engine_gaps: &id001
  - dnd2024:creature_earth_elemental
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_2.md
  specification_sha256: bc2627c84b5d22eef9220f82611d460a659fd3e00e5b06592f4e8b5ee034a808
splat_clauses:
- key: source_rules
  text: '# Earth Elemental Large Elemental, Neutral AC 17 Initiative -1 (9) HP 147 (14d10 + 70) Speed
    30 ft., Burrow 30 ft. MOD SAVE MOD SAVE MOD SAVE Str 20 +5 +5 Dex 8 -1 -1 Con 20 +5 +5 Int 5 -3 -3
    Wis 10 +0 +0 Cha 5 -3 -3 Vulnerabilities Thunder Immunities Poison; Exhaustion, Paralyzed, Petrified,
    Poisoned, Unconscious Senses Darkvision 60 ft., Tremorsense 60 ft.; Passive Perception 10 Languages
    Primordial (Terran) CR 5 (XP 1,800; PB +3) ### Traits Earth Glide. The elemental can burrow through
    nonmagical, unworked earth and stone. While doing so, the elemental doesn’t disturb the material it
    moves through. Siege Monster. The elemental deals double damage to objects and structures. ### Actions
    Multiattack. The elemental makes two attacks, using Slam or Rock Launch in any combination. Slam.
    Melee Attack Roll: +8, reach 10 ft. Hit: 14 (2d8 + 5) Bludgeoning damage. Rock Launch. Ranged Attack
    Roll: +8, range 60 ft. Hit: 8 (1d6 + 5) Bludgeoning damage. If the target is a Large or smaller creature,
    it has the Prone condition.'
  sha256: b91ac0d5408a872723cc2d87f9fb9be3d8a5d2f62e8d3d0616f925f8b34eba8b
  status: contract_specified
splat_requires_mechanics: *id001
---

# Earth Elemental

<!-- SRD page 282 -->

Large Elemental, Neutral\
AC 17 Initiative −1 (9)\
HP 147 (14d10 + 70)\
Speed 30 ft., Burrow 30 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 20 +5 +5 Dex 8 −1 −1 Con 20 +5 +5\
Int 5 −3 −3 Wis 10 +0 +0 Cha 5 −3 −3\
Vulnerabilities Thunder\
Immunities Poison; Exhaustion, Paralyzed, Petrified,\
Poisoned, Unconscious\
Senses Darkvision 60 ft., Tremorsense 60 ft.;\
Passive Perception 10\
Languages Primordial (Terran)\
CR 5 (XP 1,800; PB +3)\

### Traits

Earth Glide. The elemental can burrow through\
nonmagical, unworked earth and stone. While do-\
ing so, the elemental doesn’t disturb the material it\
moves through.\
Siege Monster. The elemental deals double damage to\
objects and structures.\

### Actions

Multiattack. The elemental makes two attacks, using\
Slam or Rock Launch in any combination.\
Slam. Melee Attack Roll: +8, reach 10 ft. Hit: 14 (2d8\
+ 5) Bludgeoning damage.\

<!-- SRD page 283 -->

Rock Launch. Ranged Attack Roll: +8, range 60 ft. Hit:\
8 (1d6 + 5) Bludgeoning damage. If the target is a Large\
or smaller creature, it has the Prone condition.\

Original layout: [[../../srd-5.2.1/pages/282|page 282]], [[../../srd-5.2.1/pages/283|page 283]].
