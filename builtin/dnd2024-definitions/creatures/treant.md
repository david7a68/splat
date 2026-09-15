---
splat_schema: 1
splat_id: 0aa80369-235f-55b4-97f6-a1479ebe1f5b
splat_kind: creature
splat_key: dnd2024:creatures/treant
splat_name: Treant
splat_origin:
  key: dnd2024:reference/creature/treant
splat_sources:
- title: SRD 5.2.1
  locator: pp. 333
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: huge
  type: plant
  alignment: Chaotic Good
  movement_text: 30 ft.
  resistances_text: Bludgeoning, Piercing
  vulnerabilities_text: Fire
  senses_text: Passive Perception 13
  languages_text: Common, Druidic, Elvish, Sylvan
  cr_text: 9 (XP 5,000; PB +4)
  hit_points_text: 138 (12d12 + 60)
splat_stats:
  strength:
    value: 23
  save_strength:
    value: 6
  dexterity:
    value: 8
  save_dexterity:
    value: -1
  constitution:
    value: 21
  save_constitution:
    value: 5
  intelligence:
    value: 12
  save_intelligence:
    value: 1
  wisdom:
    value: 16
  save_wisdom:
    value: 3
  charisma:
    value: 12
  save_charisma:
    value: 1
  armor_class:
    value: 16
  max_hp:
    value: 138
  initiative:
    value: 3
  speed:
    value: 30
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/treant
  implementation: dnd2024:creature_treant
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
        attack_bonus: 10
        targets:
          count: 1
        hit:
        - op: damage
          average: 16
          amount:
            dice:
              count: 3
              sides: 6
            add: 6
          damage_type: bludgeoning
        reach_feet: 5
      hail_of_bark:
        economy: action
        op: attack
        attack_modes:
        - ranged
        attack_bonus: 10
        targets:
          count: 1
        hit:
        - op: damage
          average: 28
          amount:
            dice:
              count: 4
              sides: 10
            add: 6
          damage_type: piercing
        range_feet: 180
      multiattack:
        economy: action
        op: sequence
        steps:
        - op: use_feature
          feature: slam
          count: 2
      animate_trees:
        economy: action
        resource:
          capacity: 1
          recovery: long_rest
          cost: 1
        op: animate_objects
        targets:
          kind: tree
          count_max: 2
          range_feet: 60
          visible_to: self
        stat_block:
          key: dnd2024:creatures/treant
        overrides:
          ability_scores:
            intelligence: 1
            charisma: 1
          can_speak: false
          remove_features:
          - animate_trees
        initiative:
          same_count_as: self
          turn_order: immediately_after_self
        controller: self
        ends_any:
        - days: 1
        - animated_tree_dies: true
        - self_dies: true
        - distance_from_self_exceeds_feet: 120
        on_end:
          op: take_root
          if_possible: true
    defenses:
      vulnerabilities:
        damage_types:
        - fire
      resistances:
        damage_types:
        - bludgeoning
        - piercing
    senses:
      passive_perception: 13
      special: []
    communication:
      speaks:
      - common
      - druidic
      - elvish
      - sylvan
      understands:
      - common
      - druidic
      - elvish
      - sylvan
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
- id: dnd2024:creature_treant
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
  - summoning
  - targeting
splat_conversion:
  status: specified
  category: creature
  source_sha256: 8a1f7e7837ea4bcde1e0d59bbfae609e59941f8ced1a6d5639ce9c1d08b50530
  source_path: builtin/srd-5.2.1/creature/treant.md
  pages:
  - 333
  engine_gaps: &id001
  - dnd2024:creature_treant
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_2.md
  specification_sha256: faa29a79ae7f329623ba74fdf326d0d781dbcf71a67b9cb3a2e06c418e11de3e
splat_clauses:
- key: source_rules
  text: '# Treant Huge Plant, Chaotic Good AC 16 Initiative +3 (13) HP 138 (12d12 + 60) Speed 30 ft. MOD
    SAVE MOD SAVE MOD SAVE Str 23 +6 +6 Dex 8 -1 -1 Con 21 +5 +5 Int 12 +1 +1 Wis 16 +3 +3 Cha 12 +1 +1
    Vulnerabilities Fire Resistances Bludgeoning, Piercing Senses Passive Perception 13 Languages Common,
    Druidic, Elvish, Sylvan CR 9 (XP 5,000; PB +4) ### Traits Siege Monster. The treant deals double damage
    to objects and structures. ### Actions Multiattack. The treant makes two Slam attacks. Slam. Melee
    Attack Roll: +10, reach 5 ft. Hit: 16 (3d6 + 6) Bludgeoning damage. Hail of Bark. Ranged Attack Roll:
    +10, range 180 ft. Hit: 28 (4d10 + 6) Piercing damage. Animate Trees (1/Day). The treant magically
    animates up to two trees it can see within 60 feet of itself. Each tree uses the Treant stat block,
    except it has Intelligence and Charisma scores of 1, it can’t speak, and it lacks this action. The
    tree takes its turn immediately after the treant on the same Initiative count, and it obeys the treant.
    A tree remains animate for 1 day or until it dies, the treant dies, or it is more than 120 feet from
    the treant. The tree then takes root if possible.'
  sha256: 13346f416ca3c5d2152b23b0020996fb548d0d8def783d3b109b01f62f95e422
  status: contract_specified
splat_requires_mechanics: *id001
---

# Treant

<!-- SRD page 333 -->

Huge Plant, Chaotic Good\
AC 16 Initiative +3 (13)\
HP 138 (12d12 + 60)\
Speed 30 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 23 +6 +6 Dex 8 −1 −1 Con 21 +5 +5\
Int 12 +1 +1 Wis 16 +3 +3 Cha 12 +1 +1\
Vulnerabilities Fire\
Resistances Bludgeoning, Piercing\
Senses Passive Perception 13\
Languages Common, Druidic, Elvish, Sylvan\
CR 9 (XP 5,000; PB +4)\

### Traits

Siege Monster. The treant deals double damage to ob-\
jects and structures.\

### Actions

Multiattack. The treant makes two Slam attacks.\
Slam. Melee Attack Roll: +10, reach 5 ft. Hit: 16 (3d6\
+ 6) Bludgeoning damage.\
Hail of Bark. Ranged Attack Roll: +10, range 180 ft.\
Hit: 28 (4d10 + 6) Piercing damage.\
Animate Trees (1/Day). The treant magically animates\
up to two trees it can see within 60 feet of itself. Each\
tree uses the Treant stat block, except it has Intelligence\
and Charisma scores of 1, it can’t speak, and it lacks\
this action. The tree takes its turn immediately after the\
treant on the same Initiative count, and it obeys the tre-\
ant. A tree remains animate for 1 day or until it dies, the\
treant dies, or it is more than 120 feet from the treant.\
The tree then takes root if possible.\

Original layout: [[../../srd-5.2.1/pages/333|page 333]].
