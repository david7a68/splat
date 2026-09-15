---
splat_schema: 1
splat_id: 40badd7c-76b8-5532-850e-cab17fcded3d
splat_kind: creature
splat_key: dnd2024:creatures/adult_green_dragon
splat_name: Adult Green Dragon
splat_origin:
  key: dnd2024:reference/creature/adult_green_dragon
splat_sources:
- title: SRD 5.2.1
  locator: pp. 294
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: huge
  type: dragon
  alignment: Lawful Evil
  tags_text: (Chromatic)
  movement_text: 40 ft., Fly 80 ft., Swim 40 ft.
  skills_text: Deception +9, Perception +12, Persuasion +9, Stealth +6
  immunities_text: Poison; Poisoned
  senses_text: Blindsight 60 ft., Darkvision 120 ft.; Passive Perception 22
  languages_text: Common, Draconic
  cr_text: 15 (XP 13,000, or 15,000 in lair; PB +5)
  hit_points_text: 207 (18d12 + 90)
splat_stats:
  strength:
    value: 23
  save_strength:
    value: 6
  dexterity:
    value: 12
  save_dexterity:
    value: 6
  constitution:
    value: 21
  save_constitution:
    value: 5
  intelligence:
    value: 18
  save_intelligence:
    value: 4
  wisdom:
    value: 15
  save_wisdom:
    value: 7
  charisma:
    value: 18
  save_charisma:
    value: 4
  armor_class:
    value: 19
  max_hp:
    value: 207
  initiative:
    value: 11
  speed:
    value: 40
    unit: ft
  fly_speed:
    value: 80
    unit: ft
  swim_speed:
    value: 40
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/adult_green_dragon
  implementation: dnd2024:creature_adult_green_dragon
  parameters:
    features:
      amphibious:
        economy: passive
        breathing_media:
        - air
        - water
      legendary_resistance:
        economy: passive
        resource:
          capacity: 3
          capacity_in_lair: 4
          recovery: long_rest
          cost: 1
        trigger: saving_throw.failed
        optional: true
        op: replace_save_outcome
        outcome: success
      rend:
        economy: action
        op: attack
        attack_modes:
        - melee
        attack_bonus: 11
        targets:
          count: 1
        hit:
        - op: damage
          average: 15
          amount:
            dice:
              count: 2
              sides: 8
            add: 6
          damage_type: slashing
        - op: damage
          average: 7
          amount:
            dice:
              count: 2
              sides: 6
            add: 0
          damage_type: poison
        reach_feet: 10
      poison_breath:
        economy: action
        resource:
          capacity: 1
          initial: 1
          recharge:
            event: turn.start
            participant: self
            roll:
              dice:
                count: 1
                sides: 6
              add: 0
            succeeds_on:
            - 5
            - 6
          cost: 1
          recovery_also:
          - short_rest
          - long_rest
        op: save
        ability: constitution
        dc: 18
        targets:
          kind: creature
          count: all
          area:
            shape: cone
            size_feet: 60
            origin: self
        failure:
        - op: damage
          average: 56
          amount:
            dice:
              count: 16
              sides: 6
            add: 0
          damage_type: poison
        success:
        - op: damage_fraction
          numerator: 1
          denominator: 2
          round: down
      multiattack:
        economy: action
        op: sequence
        steps:
        - op: use_feature
          feature: rend
          count: 3
        replacement:
          max_attacks: 1
          choices:
          - feature: spellcasting
            spell:
              key: dnd2024:spells/mind_spike
            cast_level: 3
      spellcasting:
        economy: action
        op: cast_spell_choice
        ability: charisma
        spells:
        - spell:
            key: dnd2024:spells/detect_magic
          frequency: at_will
          cast_level: spell_minimum
        - spell:
            key: dnd2024:spells/mind_spike
          frequency: at_will
          cast_level: 3
        - spell:
            key: dnd2024:spells/geas
          frequency: daily
          uses: 1
          resource_scope: per_spell
          recovery: long_rest
          cast_level: spell_minimum
        components_waived:
        - material
        save_dc: 17
      noxious_miasma:
        economy: legendary_action
        op: save
        ability: constitution
        dc: 17
        targets:
          kind: creature
          count: all
          area:
            shape: sphere
            size_feet: 20
            origin: chosen_point
            origin_visible_to: self
            origin_range_feet: 90
        failure:
        - op: damage
          average: 7
          amount:
            dice:
              count: 2
              sides: 6
            add: 0
          damage_type: poison
        - op: modify_ac
          add: -2
          ends:
            event: turn.end
            participant: target
            occurrence: next
        success: []
        cooldown_ends:
          event: turn.start
          participant: self
          occurrence: next
      mind_invasion:
        economy: legendary_action
        op: cast_spell
        spell:
          key: dnd2024:spells/mind_spike
        via: spellcasting
        cast_level: 3
      pounce:
        economy: legendary_action
        op: sequence
        steps:
        - op: move
          distance_max:
            multiply:
            - stat: speed
            - numerator: 1
              denominator: 2
        - op: use_feature
          feature: rend
          count: 1
    skill_totals:
      deception: 9
      perception: 12
      persuasion: 9
      stealth: 6
    defenses:
      immunities:
        damage_types:
        - poison
        conditions:
        - poisoned
    senses:
      passive_perception: 22
      special:
      - sense: blindsight
        range_feet: 60
      - sense: darkvision
        range_feet: 120
    communication:
      speaks:
      - common
      - draconic
      understands:
      - common
      - draconic
    legendary_actions:
      uses: 3
      uses_in_lair: 4
      cost_per_action: 1
      trigger: another_creature.turn.end
      recovery:
        event: turn.start
        participant: self
        amount: all
      maximum_per_trigger: 1
      requires:
        self_incapacitated: false
        self_can_take_actions: true
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
- id: dnd2024:creature_adult_green_dragon
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
  - another_creature.turn.end
  - saving_throw.failed
  - turn.end
  - turn.start
  capabilities:
  - action_economy
  - creature_rules
  - dice
  - effects
  - legendary_actions
  - resources
  - spellcasting
  - targeting
splat_conversion:
  status: specified
  category: creature
  source_sha256: f3541c2114dba8ed0811255e7e29122327f1f2a62b0e8e9a0f73c3ef9d010cd6
  source_path: builtin/srd-5.2.1/creature/adult_green_dragon.md
  pages:
  - 294
  engine_gaps: &id001
  - dnd2024:creature_adult_green_dragon
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_2.md
  specification_sha256: 1a00a707e29ab645a55813d1b188d7f84bf5ee6887ec0cd4af1856929289816b
splat_clauses:
- key: source_rules
  text: '# Adult Green Dragon Huge Dragon (Chromatic), Lawful Evil AC 19 Initiative +11 (21) HP 207 (18d12
    + 90) Speed 40 ft., Fly 80 ft., Swim 40 ft. MOD SAVE MOD SAVE MOD SAVE Str 23 +6 +6 Dex 12 +1 +6 Con
    21 +5 +5 Int 18 +4 +4 Wis 15 +2 +7 Cha 18 +4 +4 Skills Deception +9, Perception +12, Persuasion +9,
    Stealth +6 Immunities Poison; Poisoned Senses Blindsight 60 ft., Darkvision 120 ft.; Passive Perception
    22 Languages Common, Draconic CR 15 (XP 13,000, or 15,000 in lair; PB +5) ### Traits Amphibious. The
    dragon can breathe air and water. Legendary Resistance (3/Day, or 4/Day in Lair). If the dragon fails
    a saving throw, it can choose to succeed instead. ### Actions Multiattack. The dragon makes three
    Rend attacks. It can replace one attack with a use of Spellcasting to cast Mind Spike (level 3 version).
    Rend. Melee Attack Roll: +11, reach 10 ft. Hit: 15 (2d8 + 6) Slashing damage plus 7 (2d6) Poison damage.
    Poison Breath (Recharge 5–6). Constitution Saving Throw: DC 18, each creature in a 60-foot Cone. Failure:
    56 (16d6) Poison damage. Success: Half damage. Spellcasting. The dragon casts one of the following
    spells, requiring no Material components and using Charisma as the spellcasting ability (spell save
    DC 17): At Will: Detect Magic, Mind Spike (level 3 version) 1/Day: Geas ### Legendary Actions Legendary
    Action Uses: 3 (4 in Lair). Immediately after another creature’s turn, the dragon can expend a use
    to take one of the following actions. The dragon regains all expended uses at the start of each of
    its turns. Mind Invasion. The dragon uses Spellcasting to cast Mind Spike (level 3 version). Noxious
    Miasma. Constitution Saving Throw: DC 17, each creature in a 20-foot-radius Sphere centered on a point
    the dragon can see within 90 feet. Failure: 7 (2d6) Poison damage, and the target takes a -2 penalty
    to AC until the end of its next turn. Failure or Success: The dragon can’t take this action again
    until the start of its next turn. Pounce. The dragon moves up to half its Speed, and it makes one
    Rend attack.'
  sha256: d96f8d73b2c55a08a072b317857962b2f07a016989d8c01902738ae14f008f50
  status: contract_specified
splat_requires_mechanics: *id001
---

# Adult Green Dragon

<!-- SRD page 294 -->

Huge Dragon (Chromatic), Lawful Evil\
AC 19 Initiative +11 (21)\
HP 207 (18d12 + 90)\
Speed 40 ft., Fly 80 ft., Swim 40 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 23 +6 +6 Dex 12 +1 +6 Con 21 +5 +5\
Int 18 +4 +4 Wis 15 +2 +7 Cha 18 +4 +4\
Skills Deception +9, Perception +12, Persuasion +9,\
Stealth +6\
Immunities Poison; Poisoned\
Senses Blindsight 60 ft., Darkvision 120 ft.;\
Passive Perception 22\
Languages Common, Draconic\
CR 15 (XP 13,000, or 15,000 in lair; PB +5)\

### Traits

Amphibious. The dragon can breathe air and water.\
Legendary Resistance (3/Day, or 4/Day in Lair). If\
the dragon fails a saving throw, it can choose to suc-\
ceed instead.\

### Actions

Multiattack. The dragon makes three Rend attacks. It\
can replace one attack with a use of Spellcasting to cast\
Mind Spike (level 3 version).\
Rend. Melee Attack Roll: +11, reach 10 ft. Hit: 15 (2d8\
+ 6) Slashing damage plus 7 (2d6) Poison damage.\
Poison Breath (Recharge 5–6). Constitution Saving\
Throw: DC 18, each creature in a 60-foot Cone. Fail-\
ure: 56 (16d6) Poison damage. Success: Half damage.\
Spellcasting. The dragon casts one of the following\
spells, requiring no Material components and using\
Charisma as the spellcasting ability (spell save DC 17):\
At Will: Detect Magic, Mind Spike (level 3 version)\
1/Day: Geas\

### Legendary Actions

Legendary Action Uses: 3 (4 in Lair). Immediately after\
another creature’s turn, the dragon can expend a use to\
take one of the following actions. The dragon regains all\
expended uses at the start of each of its turns.\
Mind Invasion. The dragon uses Spellcasting to cast\
Mind Spike (level 3 version).\
Noxious Miasma. Constitution Saving Throw: DC 17,\
each creature in a 20-foot-radius Sphere centered on a\
point the dragon can see within 90 feet. Failure: 7 (2d6)\
Poison damage, and the target takes a −2 penalty to AC\
until the end of its next turn. Failure or Success: The\
dragon can’t take this action again until the start of its\
next turn.\
Pounce. The dragon moves up to half its Speed, and it\
makes one Rend attack.\

Original layout: [[../../srd-5.2.1/pages/294|page 294]].
