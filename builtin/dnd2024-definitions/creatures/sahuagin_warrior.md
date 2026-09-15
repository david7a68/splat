---
splat_schema: 1
splat_id: 752e6d53-c0bb-5ee5-8cdb-c522a0e75279
splat_kind: creature
splat_key: dnd2024:creatures/sahuagin_warrior
splat_name: Sahuagin Warrior
splat_origin:
  key: dnd2024:reference/creature/sahuagin_warrior
splat_sources:
- title: SRD 5.2.1
  locator: pp. 321
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: medium
  type: fiend
  alignment: Lawful Evil
  movement_text: 30 ft., Swim 40 ft.
  skills_text: Perception +5
  resistances_text: Acid, Cold
  senses_text: Darkvision 120 ft.; Passive Perception 15
  languages_text: Sahuagin
  cr_text: 1/2 (XP 100; PB +2)
  hit_points_text: 22 (4d8 + 4)
splat_stats:
  strength:
    value: 13
  save_strength:
    value: 1
  dexterity:
    value: 11
  save_dexterity:
    value: 0
  constitution:
    value: 12
  save_constitution:
    value: 1
  intelligence:
    value: 12
  save_intelligence:
    value: 1
  wisdom:
    value: 13
  save_wisdom:
    value: 1
  charisma:
    value: 9
  save_charisma:
    value: -1
  armor_class:
    value: 12
  max_hp:
    value: 22
  initiative:
    value: 0
  speed:
    value: 30
    unit: ft
  swim_speed:
    value: 40
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/sahuagin_warrior
  implementation: dnd2024:creature_sahuagin_warrior
  parameters:
    features:
      claw:
        economy: action
        op: attack
        attack_modes:
        - melee
        attack_bonus: 3
        targets:
          count: 1
        hit:
        - op: damage
          average: 4
          amount:
            dice:
              count: 1
              sides: 6
            add: 1
          damage_type: slashing
        reach_feet: 5
      multiattack:
        economy: action
        op: sequence
        steps:
        - op: use_feature
          feature: claw
          count: 2
      blood_frenzy:
        economy: passive
        advantage:
          roll: attack
          target_kind: creature
          when:
            target_hp_less_than_maximum: true
      limited_amphibiousness:
        economy: passive
        breathing_media:
        - air
        - water
        submersion_required_every:
          hours: 4
        failure:
          outside_water: suffocating
      shark_telepathy:
        economy: passive
        magical: true
        op: control_creatures
        targets:
          kind: shark
          range_feet: 120
          count: all
        communication: telepathy
      aquatic_charge:
        economy: bonus_action
        op: move
        mode: swim
        distance_max:
          stat: swim_speed
        direction: straight_toward_visible_enemy
    skill_totals:
      perception: 5
    defenses:
      resistances:
        damage_types:
        - acid
        - cold
    senses:
      passive_perception: 15
      special:
      - sense: darkvision
        range_feet: 120
    communication:
      speaks:
      - sahuagin
      understands:
      - sahuagin
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
- id: dnd2024:creature_sahuagin_warrior
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
  source_sha256: eb38cc123f00a292e7a19fc1c0044af0ae31eaa0f79f62f91a07d68e9dbeb873
  source_path: builtin/srd-5.2.1/creature/sahuagin_warrior.md
  pages:
  - 321
  engine_gaps: &id001
  - dnd2024:creature_sahuagin_warrior
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_2.md
  specification_sha256: a6bba3b175527197338720c6a75bd891daa061e20cac5a34720bac02e5b47e87
splat_clauses:
- key: source_rules
  text: '# Sahuagin Warrior Medium Fiend, Lawful Evil AC 12 Initiative +0 (10) HP 22 (4d8 + 4) Speed 30
    ft., Swim 40 ft. MOD SAVE MOD SAVE MOD SAVE Str 13 +1 +1 Dex 11 +0 +0 Con 12 +1 +1 Int 12 +1 +1 Wis
    13 +1 +1 Cha 9 -1 -1 Skills Perception +5 Resistances Acid, Cold Senses Darkvision 120 ft.; Passive
    Perception 15 Languages Sahuagin CR 1/2 (XP 100; PB +2) ### Traits Blood Frenzy. The sahuagin has
    Advantage on attack rolls against any creature that doesn’t have all its Hit Points. Limited Amphibiousness.
    The sahuagin can breathe air and water, but it must be submerged at least once every 4 hours to avoid
    suffocating outside water. Shark Telepathy. The sahuagin can magically control sharks within 120 feet
    of itself, using a special telepathy. ### Actions Multiattack. The sahuagin makes two Claw attacks.
    Claw. Melee Attack Roll: +3, reach 5 ft. Hit: 4 (1d6 + 1) Slashing damage. ### Bonus Actions Aquatic
    Charge. The sahuagin swims up to its Swim Speed straight toward an enemy it can see.'
  sha256: f6a7803548e2c1a2cfaf69b6cdcd0c5b3b4cc88c1763d2c466c5f398c880d961
  status: contract_specified
splat_requires_mechanics: *id001
---

# Sahuagin Warrior

<!-- SRD page 321 -->

Medium Fiend, Lawful Evil\
AC 12 Initiative +0 (10)\
HP 22 (4d8 + 4)\
Speed 30 ft., Swim 40 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 13 +1 +1 Dex 11 +0 +0 Con 12 +1 +1\
Int 12 +1 +1 Wis 13 +1 +1 Cha 9 −1 −1\
Skills Perception +5\
Resistances Acid, Cold\
Senses Darkvision 120 ft.; Passive Perception 15\
Languages Sahuagin\
CR 1/2 (XP 100; PB +2)\

### Traits

Blood Frenzy. The sahuagin has Advantage on at-\
tack rolls against any creature that doesn’t have all its\
Hit Points.\
Limited Amphibiousness. The sahuagin can breathe air\
and water, but it must be submerged at least once every\
4 hours to avoid suffocating outside water.\
Shark Telepathy. The sahuagin can magically control\
sharks within 120 feet of itself, using a special telepathy.\

### Actions

Multiattack. The sahuagin makes two Claw attacks.\
Claw. Melee Attack Roll: +3, reach 5 ft. Hit: 4 (1d6 + 1)\
Slashing damage.\

### Bonus Actions

Aquatic Charge. The sahuagin swims up to its Swim\
Speed straight toward an enemy it can see.\

Original layout: [[../../srd-5.2.1/pages/321|page 321]].
