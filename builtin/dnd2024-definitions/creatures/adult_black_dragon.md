---
splat_schema: 1
splat_id: 88e6df83-4160-5611-b731-883f728ff052
splat_kind: creature
splat_key: dnd2024:creatures/adult_black_dragon
splat_name: Adult Black Dragon
splat_origin:
  key: dnd2024:reference/creature/adult_black_dragon
splat_sources:
- title: SRD 5.2.1
  locator: pp. 264
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: huge
  type: dragon
  alignment: Chaotic Evil
  tags_text: (Chromatic)
  movement_text: 40 ft., Fly 80 ft., Swim 40 ft.
  skills_text: Perception +11, Stealth +7
  immunities_text: Acid
  senses_text: Blindsight 60 ft., Darkvision 120 ft.; Passive Perception 21
  languages_text: Common, Draconic
  cr_text: 14 (XP 11,500, or 13,000 in lair; PB +5)
  hit_points_text: 195 (17d12 + 85)
splat_stats:
  strength:
    value: 23
  save_strength:
    value: 6
  dexterity:
    value: 14
  save_dexterity:
    value: 7
  constitution:
    value: 21
  save_constitution:
    value: 5
  intelligence:
    value: 14
  save_intelligence:
    value: 2
  wisdom:
    value: 13
  save_wisdom:
    value: 6
  charisma:
    value: 19
  save_charisma:
    value: 4
  armor_class:
    value: 19
  max_hp:
    value: 195
  initiative:
    value: 12
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
  mechanic: dnd2024:creatures/adult_black_dragon
  implementation: dnd2024:creature_adult_black_dragon
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
          average: 13
          amount:
            dice:
              count: 2
              sides: 6
            add: 6
          damage_type: slashing
        - op: damage
          average: 4
          amount:
            dice:
              count: 1
              sides: 8
            add: 0
          damage_type: acid
        reach_feet: 10
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
        dc: 18
        targets:
          kind: creature
          count: all
          area:
            shape: line
            size_feet: 60
            origin: self
            width_feet: 5
        failure:
        - op: damage
          average: 54
          amount:
            dice:
              count: 12
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
            cast_level: 3
      spellcasting:
        economy: action
        op: cast_spell_choice
        ability: charisma
        spells:
        - spell:
            key: dnd2024:spells/acid_arrow
          frequency: at_will
          cast_level: 3
        - spell:
            key: dnd2024:spells/detect_magic
          frequency: at_will
          cast_level: spell_minimum
        - spell:
            key: dnd2024:spells/fear
          frequency: at_will
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
          recovery: long_rest
          cast_level: spell_minimum
        components_waived:
        - material
        save_dc: 17
        spell_attack_bonus: 9
      cloud_of_insects:
        economy: legendary_action
        op: save
        ability: dexterity
        dc: 17
        targets:
          kind: creature
          count: 1
          range_feet: 120
          visible_to: self
        failure:
        - op: damage
          average: 22
          amount:
            dice:
              count: 4
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
      perception: 11
      stealth: 7
    defenses:
      immunities:
        damage_types:
        - acid
    senses:
      passive_perception: 21
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
- id: dnd2024:creature_adult_black_dragon
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
  source_sha256: 1ca7bb9ad49fa6af8f6fce815866ec702df88705b147a6fccc0a869ae73f9ef3
  source_path: builtin/srd-5.2.1/creature/adult_black_dragon.md
  pages:
  - 264
  engine_gaps: &id001
  - dnd2024:creature_adult_black_dragon
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_2.md
  specification_sha256: fafe717c48fa802a5b41da9cc740f0b56307a4ed898bdb0aef90f762804c7afa
splat_clauses:
- key: source_rules
  text: '# Adult Black Dragon Huge Dragon (Chromatic), Chaotic Evil AC 19 Initiative +12 (22) HP 195 (17d12
    + 85) Speed 40 ft., Fly 80 ft., Swim 40 ft. MOD SAVE MOD SAVE MOD SAVE Str 23 +6 +6 Dex 14 +2 +7 Con
    21 +5 +5 Int 14 +2 +2 Wis 13 +1 +6 Cha 19 +4 +4 Skills Perception +11, Stealth +7 Immunities Acid
    Senses Blindsight 60 ft., Darkvision 120 ft.; Passive Perception 21 Languages Common, Draconic CR
    14 (XP 11,500, or 13,000 in lair; PB +5) ### Traits Amphibious. The dragon can breathe air and water.
    Legendary Resistance (3/Day, or 4/Day in Lair). If the dragon fails a saving throw, it can choose
    to succeed instead. ### Actions Multiattack. The dragon makes three Rend attacks. It can replace one
    attack with a use of Spellcasting to cast Acid Arrow (level 3 version). Rend. Melee Attack Roll: +11,
    reach 10 ft. Hit: 13 (2d6 + 6) Slashing damage plus 4 (1d8) Acid damage. Acid Breath (Recharge 5–6).
    Dexterity Saving Throw: DC 18, each creature in a 60-foot-long, 5-footwide Line. Failure: 54 (12d8)
    Acid damage. Success: Half damage. Spellcasting. The dragon casts one of the following spells, requiring
    no Material components and using Charisma as the spellcasting ability (spell save DC 17, +9 to hit
    with spell attacks): At Will: Acid Arrow (level 3 version), Detect Magic, Fear 1/Day Each: Speak with
    Dead, Vitriolic Sphere ### Legendary Actions Legendary Action Uses: 3 (4 in Lair). Immediately after
    another creature’s turn, the dragon can expend a use to take one of the following actions. The dragon
    regains all expended uses at the start of each of its turns. Cloud of Insects. Dexterity Saving Throw:
    DC 17, one creature the dragon can see within 120 feet. Failure: 22 (4d10) Poison damage, and the
    target has Disadvantage on saving throws to maintain Concentration until the end of its next turn.
    Failure or Success: The dragon can’t take this action again until the start of its next turn. Frightful
    Presence. The dragon uses Spellcasting to cast Fear. The dragon can’t take this action again until
    the start of its next turn. Pounce. The dragon moves up to half its Speed, and it makes one Rend attack.'
  sha256: ede20f1bbf4a6b542eb64b25a0931c0a9adafe8b4457eb9b77904c3cf1a1ce79
  status: contract_specified
splat_requires_mechanics: *id001
---

# Adult Black Dragon

<!-- SRD page 264 -->

Huge Dragon (Chromatic), Chaotic Evil\
AC 19 Initiative +12 (22)\
HP 195 (17d12 + 85)\
Speed 40 ft., Fly 80 ft., Swim 40 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 23 +6 +6 Dex 14 +2 +7 Con 21 +5 +5\
Int 14 +2 +2 Wis 13 +1 +6 Cha 19 +4 +4\
Skills Perception +11, Stealth +7\
Immunities Acid\
Senses Blindsight 60 ft., Darkvision 120 ft.;\
Passive Perception 21\
Languages Common, Draconic\
CR 14 (XP 11,500, or 13,000 in lair; PB +5)\

### Traits

Amphibious. The dragon can breathe air and water.\
Legendary Resistance (3/Day, or 4/Day in Lair). If\
the dragon fails a saving throw, it can choose to suc-\
ceed instead.\

### Actions

Multiattack. The dragon makes three Rend attacks. It\
can replace one attack with a use of Spellcasting to cast\
Acid Arrow (level 3 version).\
Rend. Melee Attack Roll: +11, reach 10 ft. Hit: 13 (2d6\
+ 6) Slashing damage plus 4 (1d8) Acid damage.\
Acid Breath (Recharge 5–6). Dexterity Saving Throw:\
DC 18, each creature in a 60-foot-long, 5-foot-\
wide Line. Failure: 54 (12d8) Acid damage. Success:\
Half damage.\
Spellcasting. The dragon casts one of the following\
spells, requiring no Material components and using\
Charisma as the spellcasting ability (spell save DC 17,\
+9 to hit with spell attacks):\
At Will: Acid Arrow (level 3 version), Detect Magic, Fear\
1/Day Each: Speak with Dead, Vitriolic Sphere\

### Legendary Actions

Legendary Action Uses: 3 (4 in Lair). Immediately after\
another creature’s turn, the dragon can expend a use to\
take one of the following actions. The dragon regains all\
expended uses at the start of each of its turns.\
Cloud of Insects. Dexterity Saving Throw: DC 17, one\
creature the dragon can see within 120 feet. Failure: 22\
(4d10) Poison damage, and the target has Disadvantage\
on saving throws to maintain Concentration until the\
end of its next turn. Failure or Success: The dragon can’t\
take this action again until the start of its next turn.\
Frightful Presence. The dragon uses Spellcasting to cast\
Fear. The dragon can’t take this action again until the\
start of its next turn.\
Pounce. The dragon moves up to half its Speed, and it\
makes one Rend attack.\

Original layout: [[../../srd-5.2.1/pages/264|page 264]].
