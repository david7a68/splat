---
splat_schema: 1
splat_id: f0ab06b4-adb6-53ac-b983-62c72d1555cd
splat_kind: creature
splat_key: dnd2024:creatures/hill_giant
splat_name: Hill Giant
splat_origin:
  key: dnd2024:reference/creature/hill_giant
splat_sources:
- title: SRD 5.2.1
  locator: pp. 298
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: huge
  type: giant
  alignment: Chaotic Evil
  movement_text: 40 ft.
  skills_text: Perception +2
  senses_text: Passive Perception 12
  languages_text: Giant
  cr_text: 5 (XP 1,800; PB +3)
  hit_points_text: 105 (10d12 + 40)
splat_stats:
  strength:
    value: 21
  save_strength:
    value: 5
  dexterity:
    value: 8
  save_dexterity:
    value: -1
  constitution:
    value: 19
  save_constitution:
    value: 4
  intelligence:
    value: 5
  save_intelligence:
    value: -3
  wisdom:
    value: 9
  save_wisdom:
    value: -1
  charisma:
    value: 6
  save_charisma:
    value: -2
  armor_class:
    value: 13
  max_hp:
    value: 105
  initiative:
    value: 2
  speed:
    value: 40
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/hill_giant
  implementation: dnd2024:creature_hill_giant
  parameters:
    features:
      tree_club:
        economy: action
        op: attack
        attack_modes:
        - melee
        attack_bonus: 8
        targets:
          count: 1
        hit:
        - op: damage
          average: 18
          amount:
            dice:
              count: 3
              sides: 8
            add: 5
          damage_type: bludgeoning
        - op: if
          test:
            target_kind: creature
            target_size_max: large
          then:
          - op: condition
            condition: prone
        reach_feet: 10
      trash_lob:
        economy: action
        op: attack
        attack_modes:
        - ranged
        attack_bonus: 8
        targets:
          count: 1
        hit:
        - op: damage
          average: 16
          amount:
            dice:
              count: 2
              sides: 10
            add: 5
          damage_type: bludgeoning
        - op: condition
          condition: poisoned
          ends:
            event: turn.end
            participant: target
            occurrence: next
        range_feet: 60
        long_range_feet: 240
      multiattack:
        economy: action
        op: repeat_choice
        count: 2
        choices:
        - feature: tree_club
        - feature: trash_lob
    skill_totals:
      perception: 2
    defenses: {}
    senses:
      passive_perception: 12
      special: []
    communication:
      speaks:
      - giant
      understands:
      - giant
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
- id: dnd2024:creature_hill_giant
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
  events:
  - turn.end
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
  source_sha256: 04e05b911d89e2061d3813b5eaf3a7807c9a4fe3d1fd4179c37f6cbe08cdbae2
  source_path: builtin/srd-5.2.1/creature/hill_giant.md
  pages:
  - 298
  engine_gaps: &id001
  - dnd2024:creature_hill_giant
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_2.md
  specification_sha256: 618156b669bc22817f59e3dc5ad876c97a8c4ecfc45e02bbc04566dc56e15432
splat_clauses:
- key: source_rules
  text: '# Hill Giant Huge Giant, Chaotic Evil AC 13 Initiative +2 (12) HP 105 (10d12 + 40) Speed 40 ft.
    MOD SAVE MOD SAVE MOD SAVE Str 21 +5 +5 Dex 8 -1 -1 Con 19 +4 +4 Int 5 -3 -3 Wis 9 -1 -1 Cha 6 -2
    -2 Skills Perception +2 Senses Passive Perception 12 Languages Giant CR 5 (XP 1,800; PB +3) ### Actions
    Multiattack. The giant makes two attacks, using Tree Club or Trash Lob in any combination. Tree Club.
    Melee Attack Roll: +8, reach 10 ft. Hit: 18 (3d8 + 5) Bludgeoning damage. If the target is a Large
    or smaller creature, it has the Prone condition. Trash Lob. Ranged Attack Roll: +8, range 60/240 ft.
    Hit: 16 (2d10 + 5) Bludgeoning damage, and the target has the Poisoned condition until the end of
    its next turn.'
  sha256: b1ed8c3ee14b0a91cc6eab8075c1e2801b9070d39ac0e8a3f6fdef71494eb68c
  status: contract_specified
splat_requires_mechanics: *id001
---

# Hill Giant

<!-- SRD page 298 -->

Huge Giant, Chaotic Evil\
AC 13 Initiative +2 (12)\
HP 105 (10d12 + 40)\
Speed 40 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 21 +5 +5 Dex 8 −1 −1 Con 19 +4 +4\
Int 5 −3 −3 Wis 9 −1 −1 Cha 6 −2 −2\
Skills Perception +2\
Senses Passive Perception 12\
Languages Giant\
CR 5 (XP 1,800; PB +3)\

### Actions

Multiattack. The giant makes two attacks, using Tree\
Club or Trash Lob in any combination.\
Tree Club. Melee Attack Roll: +8, reach 10 ft. Hit: 18\
(3d8 + 5) Bludgeoning damage. If the target is a Large\
or smaller creature, it has the Prone condition.\
Trash Lob. Ranged Attack Roll: +8, range 60/240\
ft. Hit: 16 (2d10 + 5) Bludgeoning damage, and the\
target has the Poisoned condition until the end of its\
next turn.\

Original layout: [[../../srd-5.2.1/pages/298|page 298]].
