---
splat_schema: 1
splat_id: 501f429d-aec6-542a-98c8-ffdbd2772985
splat_kind: creature
splat_key: dnd2024:creatures/ancient_black_dragon
splat_name: Ancient Black Dragon
splat_origin:
  key: dnd2024:reference/creature/ancient_black_dragon
splat_sources:
- title: SRD 5.2.1
  locator: pp. 265
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: gargantuan
  type: dragon
  alignment: Chaotic Evil
  tags_text: (Chromatic)
  movement_text: 40 ft., Fly 80 ft., Swim 40 ft.
  skills_text: Perception +16, Stealth +9
  immunities_text: Acid
  senses_text: Blindsight 60 ft., Darkvision 120 ft.; Passive Perception 26
  languages_text: Common, Draconic
  cr_text: 21 (XP 33,000, or 41,000 in lair; PB +7)
  hit_points_text: 367 (21d20 + 147)
splat_stats:
  strength:
    value: 27
  save_strength:
    value: 8
  dexterity:
    value: 14
  save_dexterity:
    value: 9
  constitution:
    value: 25
  save_constitution:
    value: 7
  intelligence:
    value: 16
  save_intelligence:
    value: 3
  wisdom:
    value: 15
  save_wisdom:
    value: 9
  charisma:
    value: 22
  save_charisma:
    value: 6
  armor_class:
    value: 22
  max_hp:
    value: 367
  initiative:
    value: 16
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
  mechanic: dnd2024:creatures/ancient_black_dragon
  implementation: dnd2024:creature_ancient_black_dragon
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
          capacity: 4
          capacity_in_lair: 5
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
        attack_bonus: 15
        targets:
          count: 1
        hit:
        - op: damage
          average: 17
          amount:
            dice:
              count: 2
              sides: 8
            add: 8
          damage_type: slashing
        - op: damage
          average: 9
          amount:
            dice:
              count: 2
              sides: 8
            add: 0
          damage_type: acid
        reach_feet: 15
      acid_breath:
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
        ability: dexterity
        dc: 22
        targets:
          kind: creature
          count: all
          area:
            shape: line
            size_feet: 90
            origin: self
            width_feet: 10
        failure:
        - op: damage
          average: 67
          amount:
            dice:
              count: 15
              sides: 8
            add: 0
          damage_type: acid
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
              key: dnd2024:spells/acid_arrow
            cast_level: 4
      spellcasting:
        economy: action
        op: cast_spell_choice
        ability: charisma
        spells:
        - spell:
            key: dnd2024:spells/acid_arrow
          frequency: at_will
          cast_level: 4
        - spell:
            key: dnd2024:spells/detect_magic
          frequency: at_will
          cast_level: spell_minimum
        - spell:
            key: dnd2024:spells/fear
          frequency: at_will
          cast_level: spell_minimum
        - spell:
            key: dnd2024:spells/create_undead
          frequency: daily
          uses: 1
          resource_scope: per_spell
          recovery: long_rest
          cast_level: spell_minimum
        - spell:
            key: dnd2024:spells/speak_with_dead
          frequency: daily
          uses: 1
          resource_scope: per_spell
          recovery: long_rest
          cast_level: spell_minimum
        - spell:
            key: dnd2024:spells/vitriolic_sphere
          frequency: daily
          uses: 1
          resource_scope: per_spell
          cast_level: 5
          recovery: long_rest
        components_waived:
        - material
        save_dc: 21
        spell_attack_bonus: 13
      cloud_of_insects:
        economy: legendary_action
        op: save
        ability: dexterity
        dc: 21
        targets:
          kind: creature
          count: 1
          range_feet: 120
          visible_to: self
        failure:
        - op: damage
          average: 33
          amount:
            dice:
              count: 6
              sides: 10
            add: 0
          damage_type: poison
        - op: disadvantage
          roll: concentration_save
          ends:
            event: turn.end
            participant: target
            occurrence: next
        success: []
        cooldown_ends:
          event: turn.start
          participant: self
          occurrence: next
      frightful_presence:
        economy: legendary_action
        op: cast_spell
        spell:
          key: dnd2024:spells/fear
        via: spellcasting
        cooldown_ends:
          event: turn.start
          participant: self
          occurrence: next
        cast_level: spell_minimum
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
      perception: 16
      stealth: 9
    defenses:
      immunities:
        damage_types:
        - acid
    senses:
      passive_perception: 26
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
- id: dnd2024:creature_ancient_black_dragon
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
  source_sha256: 0cb683b5bae14f01b0db36b2fbb43a720dae11043e9406b56adf192ecbf85585
  source_path: builtin/srd-5.2.1/creature/ancient_black_dragon.md
  pages:
  - 265
  engine_gaps: &id001
  - dnd2024:creature_ancient_black_dragon
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_2.md
  specification_sha256: 57a92f3e871b2cfbba3bc150f80163cdd5c2978fe2adfe276c0bbe8edbf0f3a7
splat_clauses:
- key: source_rules
  text: '# Ancient Black Dragon Gargantuan Dragon (Chromatic), Chaotic Evil AC 22 Initiative +16 (26)
    HP 367 (21d20 + 147) Speed 40 ft., Fly 80 ft., Swim 40 ft. MOD SAVE MOD SAVE MOD SAVE Str 27 +8 +8
    Dex 14 +2 +9 Con 25 +7 +7 Int 16 +3 +3 Wis 15 +2 +9 Cha 22 +6 +6 Skills Perception +16, Stealth +9
    Immunities Acid Senses Blindsight 60 ft., Darkvision 120 ft.; Passive Perception 26 Languages Common,
    Draconic CR 21 (XP 33,000, or 41,000 in lair; PB +7) ### Traits Amphibious. The dragon can breathe
    air and water. Legendary Resistance (4/Day, or 5/Day in Lair). If the dragon fails a saving throw,
    it can choose to succeed instead. ### Actions Multiattack. The dragon makes three Rend attacks. It
    can replace one attack with a use of Spellcasting to cast Acid Arrow (level 4 version). Rend. Melee
    Attack Roll: +15, reach 15 ft. Hit: 17 (2d8 + 8) Slashing damage plus 9 (2d8) Acid damage. Acid Breath
    (Recharge 5–6). Dexterity Saving Throw: DC 22, each creature in a 90-foot-long, 10-footwide Line.
    Failure: 67 (15d8) Acid damage. Success: Half damage. Spellcasting. The dragon casts one of the following
    spells, requiring no Material components and using Charisma as the spellcasting ability (spell save
    DC 21, +13 to hit with spell attacks): At Will: Acid Arrow (level 4 version), Detect Magic, Fear 1/Day
    Each: Create Undead, Speak with Dead, Vitriolic Sphere (level 5 version) ### Legendary Actions Legendary
    Action Uses: 3 (4 in Lair). Immediately after another creature’s turn, the dragon can expend a use
    to take one of the following actions. The dragon regains all expended uses at the start of each of
    its turns. Cloud of Insects. Dexterity Saving Throw: DC 21, one creature the dragon can see within
    120 feet. Failure: 33 (6d10) Poison damage, and the target has Disadvantage on saving throws to maintain
    Concentration until the end of its next turn. Failure or Success: The dragon can’t take this action
    again until the start of its next turn. Frightful Presence. The dragon uses Spellcasting to cast Fear.
    The dragon can’t take this action again until the start of its next turn. Pounce. The dragon moves
    up to half its Speed, and it makes one Rend attack.'
  sha256: 1da6aad4decdac613b47c2cf62a7cd31976d42106318a547a4be423f24bef87e
  status: contract_specified
splat_requires_mechanics: *id001
---

# Ancient Black Dragon

<!-- SRD page 265 -->

Gargantuan Dragon (Chromatic), Chaotic Evil\
AC 22  Initiative +16 (26)\
HP 367 (21d20 + 147)\
Speed 40 ft., Fly 80 ft., Swim 40 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 27 +8 +8 Dex 14 +2 +9 Con 25 +7 +7\
Int 16 +3 +3 Wis 15 +2 +9 Cha 22 +6 +6\
Skills Perception +16, Stealth +9\
Immunities Acid\
Senses Blindsight 60 ft., Darkvision 120 ft.;\
Passive Perception 26\
Languages Common, Draconic\
CR 21 (XP 33,000, or 41,000 in lair; PB +7)\

### Traits

Amphibious. The dragon can breathe air and water.\
Legendary Resistance (4/Day, or 5/Day in Lair). If\
the dragon fails a saving throw, it can choose to suc-\
ceed instead.\

### Actions

Multiattack. The dragon makes three Rend attacks. It\
can replace one attack with a use of Spellcasting to cast\
Acid Arrow (level 4 version).\
Rend. Melee Attack Roll: +15, reach 15 ft. Hit: 17 (2d8\
+ 8) Slashing damage plus 9 (2d8) Acid damage.\
Acid Breath (Recharge 5–6). Dexterity Saving Throw:\
DC 22, each creature in a 90-foot-long, 10-foot-\
wide Line. Failure: 67 (15d8) Acid damage. Success:\
Half damage.\
Spellcasting. The dragon casts one of the following\
spells, requiring no Material components and using\
Charisma as the spellcasting ability (spell save DC 21,\
+13 to hit with spell attacks):\
At Will: Acid Arrow (level 4 version), Detect\
Magic, Fear\
1/Day Each: Create Undead, Speak with Dead, Vitriolic\
Sphere (level 5 version)\

### Legendary Actions

Legendary Action Uses: 3 (4 in Lair). Immediately after\
another creature’s turn, the dragon can expend a use to\
take one of the following actions. The dragon regains all\
expended uses at the start of each of its turns.\
Cloud of Insects. Dexterity Saving Throw: DC 21, one\
creature the dragon can see within 120 feet. Failure: 33\
(6d10) Poison damage, and the target has Disadvantage\
on saving throws to maintain Concentration until the\
end of its next turn. Failure or Success: The dragon can’t\
take this action again until the start of its next turn.\
Frightful Presence. The dragon uses Spellcasting to cast\
Fear. The dragon can’t take this action again until the\
start of its next turn.\
Pounce. The dragon moves up to half its Speed, and it\
makes one Rend attack.\

Original layout: [[../../srd-5.2.1/pages/265|page 265]].
