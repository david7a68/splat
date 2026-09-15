---
splat_schema: 1
splat_id: 3690316d-fb14-5b76-b9f8-5230a0c69e47
splat_kind: creature
splat_key: dnd2024:creatures/knight
splat_name: Knight
splat_origin:
  key: dnd2024:reference/creature/knight
splat_sources:
- title: SRD 5.2.1
  locator: pp. 302
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: medium
  type: humanoid
  alignment: Neutral
  size_options_text: Medium or Small
  movement_text: 30 ft.
  immunities_text: Frightened
  gear_text: Greatsword, Heavy Crossbow, Plate Armor
  senses_text: Passive Perception 10
  languages_text: Common plus one other language
  cr_text: 3 (XP 700; PB +2)
  hit_points_text: 52 (8d8 + 16)
splat_stats:
  strength:
    value: 16
  save_strength:
    value: 3
  dexterity:
    value: 11
  save_dexterity:
    value: 0
  constitution:
    value: 14
  save_constitution:
    value: 4
  intelligence:
    value: 11
  save_intelligence:
    value: 0
  wisdom:
    value: 11
  save_wisdom:
    value: 2
  charisma:
    value: 15
  save_charisma:
    value: 2
  armor_class:
    value: 18
  max_hp:
    value: 52
  initiative:
    value: 0
  speed:
    value: 30
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/knight
  implementation: dnd2024:creature_knight
  parameters:
    features:
      greatsword:
        economy: action
        op: attack
        attack_modes:
        - melee
        attack_bonus: 5
        targets:
          count: 1
        hit:
        - op: damage
          average: 10
          amount:
            dice:
              count: 2
              sides: 6
            add: 3
          damage_type: slashing
        - op: damage
          average: 4
          amount:
            dice:
              count: 1
              sides: 8
            add: 0
          damage_type: radiant
        reach_feet: 5
      heavy_crossbow:
        economy: action
        op: attack
        attack_modes:
        - ranged
        attack_bonus: 2
        targets:
          count: 1
        hit:
        - op: damage
          average: 11
          amount:
            dice:
              count: 2
              sides: 10
            add: 0
          damage_type: piercing
        - op: damage
          average: 4
          amount:
            dice:
              count: 1
              sides: 8
            add: 0
          damage_type: radiant
        range_feet: 100
        long_range_feet: 400
      multiattack:
        economy: action
        op: repeat_choice
        count: 2
        choices:
        - feature: greatsword
        - feature: heavy_crossbow
      parry:
        economy: reaction
        trigger: melee_attack.hits_self
        requires:
          holding_weapon: true
        effects:
        - op: modify_ac
          add: 2
          scope: triggering_attack
          reevaluate_hit: true
    defenses:
      immunities:
        conditions:
        - frightened
    senses:
      passive_perception: 10
      special: []
    gear:
    - item_name: Greatsword
      quantity: 1
    - item_name: Heavy Crossbow
      quantity: 1
    - item_name: Plate Armor
      quantity: 1
    communication:
      speaks:
      - common
      understands:
      - common
      additional_languages:
        count: 1
        chooser: gm
        distinct_from_known: true
        can_speak: true
        can_understand: true
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
- id: dnd2024:creature_knight
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
  - melee_attack.hits_self
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
  source_sha256: 3f2c0dfd8c2d83959f55617c1114fde10137a47b2d952d43f7352f1ba99b77a4
  source_path: builtin/srd-5.2.1/creature/knight.md
  pages:
  - 302
  engine_gaps: &id001
  - dnd2024:creature_knight
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_2.md
  specification_sha256: 845e7fcf4857147976536ec2ac63ee302c5355287932101e5c06b979b0837818
splat_clauses:
- key: source_rules
  text: '# Knight Medium or Small Humanoid, Neutral AC 18 Initiative +0 (10) HP 52 (8d8 + 16) Speed 30
    ft. MOD SAVE MOD SAVE MOD SAVE Str 16 +3 +3 Dex 11 +0 +0 Con 14 +2 +4 Int 11 +0 +0 Wis 11 +0 +2 Cha
    15 +2 +2 Immunities Frightened Gear Greatsword, Heavy Crossbow, Plate Armor Senses Passive Perception
    10 Languages Common plus one other language CR 3 (XP 700; PB +2) ### Actions Multiattack. The knight
    makes two attacks, using Greatsword or Heavy Crossbow in any combination. Greatsword. Melee Attack
    Roll: +5, reach 5 ft. Hit: 10 (2d6 + 3) Slashing damage plus 4 (1d8) Radiant damage. Heavy Crossbow.
    Ranged Attack Roll: +2, range 100/400 ft. Hit: 11 (2d10) Piercing damage plus 4 (1d8) Radiant damage.
    ### Reactions Parry. Trigger: The knight is hit by a melee attack roll while holding a weapon. Response:
    The knight adds 2 to its AC against that attack, possibly causing it to miss.'
  sha256: 8de63fb51e195ce73877b8bf88556c61698ed7fa36a27c5cc5f65ddc16767312
  status: contract_specified
splat_requires_mechanics: *id001
---

# Knight

<!-- SRD page 302 -->

Medium or Small Humanoid, Neutral\
AC 18 Initiative +0 (10)\
HP 52 (8d8 + 16)\
Speed 30 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str  16 +3 +3 Dex 11 +0 +0 Con 14 +2 +4\
Int  11 +0 +0 Wis  11 +0 +2 Cha 15 +2 +2\
Immunities Frightened\
Gear Greatsword, Heavy Crossbow, Plate Armor\
Senses Passive Perception 10\
Languages Common plus one other language\
CR 3 (XP 700; PB +2)\

### Actions

Multiattack. The knight makes two attacks, using\
Greatsword or Heavy Crossbow in any combination.\
Greatsword. Melee Attack Roll: +5, reach 5 ft. Hit: 10\
(2d6 + 3) Slashing damage plus 4 (1d8) Radiant damage.\
Heavy Crossbow. Ranged Attack Roll: +2, range\
100/400 ft. Hit: 11 (2d10) Piercing damage plus 4 (1d8)\
Radiant damage.\

### Reactions

Parry. Trigger: The knight is hit by a melee attack roll\
while holding a weapon. Response: The knight adds 2\
to its AC against that attack, possibly causing it to miss.\

Original layout: [[../../srd-5.2.1/pages/302|page 302]].
