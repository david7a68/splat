---
splat_schema: 1
splat_id: 89e2602e-5fd7-5f0c-82c6-d5bb78e0fcf5
splat_kind: creature
splat_key: dnd2024:creatures/wererat
splat_name: Wererat
splat_origin:
  key: dnd2024:reference/creature/wererat
splat_sources:
- title: SRD 5.2.1
  locator: pp. 338
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: medium
  type: monstrosity
  alignment: Lawful Evil
  size_options_text: Medium or Small
  tags_text: (Lycanthrope)
  movement_text: 30 ft., Climb 30 ft.
  skills_text: Perception +4, Stealth +5
  gear_text: Hand Crossbow
  senses_text: Darkvision 60 ft.; Passive Perception 14
  languages_text: Common (can’t speak in rat form)
  cr_text: 2 (XP 450; PB +2)
  hit_points_text: 60 (11d8 + 11)
splat_stats:
  strength:
    value: 10
  save_strength:
    value: 0
  dexterity:
    value: 16
  save_dexterity:
    value: 3
  constitution:
    value: 12
  save_constitution:
    value: 1
  intelligence:
    value: 11
  save_intelligence:
    value: 0
  wisdom:
    value: 10
  save_wisdom:
    value: 0
  charisma:
    value: 8
  save_charisma:
    value: -1
  armor_class:
    value: 13
  max_hp:
    value: 60
  initiative:
    value: 3
  speed:
    value: 30
    unit: ft
  climb_speed:
    value: 30
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/wererat
  implementation: dnd2024:creature_wererat
  parameters:
    features:
      bite:
        economy: action
        op: attack
        attack_modes:
        - melee
        attack_bonus: 5
        targets:
          count: 1
        hit:
        - op: damage
          average: 8
          amount:
            dice:
              count: 2
              sides: 4
            add: 3
          damage_type: piercing
        - op: if
          test:
            target_type: humanoid
          then:
          - op: save
            ability: constitution
            dc: 11
            targets:
              participant: target
            failure:
            - op: curse
              trigger: target.hit_points_would_become_zero
              replacement:
                op: replace_stat_block
                definition:
                  key: dnd2024:creatures/wererat
                controller: gm
                hit_points: 10
            success:
            - op: feature_immunity
              source: self
              feature: bite_curse
              duration:
                hours: 24
        reach_feet: 5
        requires_form:
        - rat
        - hybrid
      scratch:
        economy: action
        op: attack
        attack_modes:
        - melee
        attack_bonus: 5
        targets:
          count: 1
        hit:
        - op: damage
          average: 6
          amount:
            dice:
              count: 1
              sides: 6
            add: 3
          damage_type: slashing
        reach_feet: 5
        range_feet: 30
        long_range_feet: 120
      multiattack:
        economy: action
        op: repeat_choice
        count: 2
        choices:
        - feature: scratch
        - feature: hand_crossbow
        replacement:
          max_attacks: 1
          choices:
          - feature: bite
      shape_shift:
        economy: bonus_action
        op: shape_shift
        forms:
        - form: hybrid
          size: medium
        - form: rat
          size: small
        - form: true_humanoid
        retain_statistics_except:
        - size
        equipment_transformed: false
      hand_crossbow:
        economy: action
        op: attack
        attack_modes:
        - ranged
        attack_bonus: 5
        range_feet: 30
        long_range_feet: 120
        targets:
          count: 1
        hit:
        - op: damage
          average: 6
          amount:
            dice:
              count: 1
              sides: 6
            add: 3
          damage_type: piercing
        requires_form:
        - humanoid
        - hybrid
    skill_totals:
      perception: 4
      stealth: 5
    defenses: {}
    senses:
      passive_perception: 14
      special:
      - sense: darkvision
        range_feet: 60
    gear:
    - item_name: Hand Crossbow
      quantity: 1
    communication:
      speaks:
      - common
      understands:
      - common
      cannot_speak_in_forms:
      - rat
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
- id: dnd2024:creature_wererat
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
  - target.hit_points_would_become_zero
  capabilities:
  - action_economy
  - creature_rules
  - curses
  - dice
  - effects
  - resources
  - shapechanging
  - stat_block_replacement
  - targeting
splat_conversion:
  status: specified
  category: creature
  source_sha256: fb5e4fc9ef8591bb2a9906ade41f535f23f6e1fb00d05868a40d918a464ce581
  source_path: builtin/srd-5.2.1/creature/wererat.md
  pages:
  - 338
  engine_gaps: &id001
  - dnd2024:creature_wererat
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_2.md
  specification_sha256: 4149cde2906de29d669f39effeb0a993d8f659721ea5128749b44f575e375b98
splat_clauses:
- key: source_rules
  text: '# Wererat Medium or Small Monstrosity (Lycanthrope), Lawful Evil AC 13 Initiative +3 (13) HP
    60 (11d8 + 11) Speed 30 ft., Climb 30 ft. MOD SAVE MOD SAVE MOD SAVE Str 10 +0 +0 Dex 16 +3 +3 Con
    12 +1 +1 Int 11 +0 +0 Wis 10 +0 +0 Cha 8 -1 -1 Skills Perception +4, Stealth +5 Gear Hand Crossbow
    Senses Darkvision 60 ft.; Passive Perception 14 Languages Common (can’t speak in rat form) CR 2 (XP
    450; PB +2) ### Actions Multiattack. The wererat makes two attacks, using Scratch or Hand Crossbow
    in any combination. It can replace one attack with a Bite attack. Bite (Rat or Hybrid Form Only).
    Melee Attack Roll: +5, reach 5 ft. Hit: 8 (2d4 + 3) Piercing damage. If the target is a Humanoid,
    it is subjected to the following effect. Constitution Saving Throw: DC 11. Failure: The target is
    cursed. If the cursed target drops to 0 Hit Points, it instead becomes a Wererat under the GM’s control
    and has 10 Hit Points. Success: The target is immune to this wererat’s curse for 24 hours. Scratch.
    Melee Attack Roll: +5, reach 5 ft. Hit: 6 (1d6 + 3) Slashing damage. Hand Crossbow (Humanoid or Hybrid
    Form Only). Ranged Attack Roll: +5, range 30/120 ft. Hit: 6 (1d6 + 3) Piercing damage. ### Bonus Actions
    Shape-Shift. The wererat shape-shifts into a Medium rat-humanoid hybrid or a Small rat, or it returns
    to its true humanoid form. Its game statistics, other than its size, are the same in each form. Any
    equipment it is wearing or carrying isn’t transformed.'
  sha256: 067930c5ab10eb3a97ddaf1ebc1168faa8228e181bc5eccd6d98ea05577e8be4
  status: contract_specified
splat_requires_mechanics: *id001
---

# Wererat

<!-- SRD page 338 -->

Medium or Small Monstrosity (Lycanthrope), Lawful Evil\
AC 13 Initiative +3 (13)\
HP 60 (11d8 + 11)\
Speed 30 ft., Climb 30 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 10 +0 +0 Dex 16 +3 +3 Con 12 +1 +1\
Int 11 +0 +0 Wis 10 +0 +0 Cha 8 −1 −1\
Skills Perception +4, Stealth +5\
Gear Hand Crossbow\
Senses Darkvision 60 ft.; Passive Perception 14\
Languages Common (can’t speak in rat form)\
CR 2 (XP 450; PB +2)\

### Actions

Multiattack. The wererat makes two attacks, using\
Scratch or Hand Crossbow in any combination. It can\
replace one attack with a Bite attack.\
Bite (Rat or Hybrid Form Only). Melee Attack Roll:\
+5, reach 5 ft. Hit: 8 (2d4 + 3) Piercing damage. If the\
target is a Humanoid, it is subjected to the following\
effect. Constitution Saving Throw: DC 11. Failure: The\
target is cursed. If the cursed target drops to 0 Hit\
Points, it instead becomes a Wererat under the GM’s\
control and has 10 Hit Points. Success: The target is im-\
mune to this wererat’s curse for 24 hours.\
Scratch. Melee Attack Roll: +5, reach 5 ft. Hit: 6 (1d6\
+ 3) Slashing damage.\
Hand Crossbow (Humanoid or Hybrid Form Only).\
Ranged Attack Roll: +5, range 30/120 ft. Hit: 6 (1d6\
+ 3) Piercing damage.\

### Bonus Actions

Shape-Shift. The wererat shape-shifts into a Medium\
rat-humanoid hybrid or a Small rat, or it returns to its\
true humanoid form. Its game statistics, other than its\
size, are the same in each form. Any equipment it is\
wearing or carrying isn’t transformed.\

Original layout: [[../../srd-5.2.1/pages/338|page 338]].
