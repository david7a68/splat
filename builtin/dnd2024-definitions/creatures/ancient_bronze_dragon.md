---
splat_schema: 1
splat_id: d587891f-84a1-59f8-aefb-70ae0130f5bc
splat_kind: creature
splat_key: dnd2024:creatures/ancient_bronze_dragon
splat_name: Ancient Bronze Dragon
splat_origin:
  key: dnd2024:reference/creature/ancient_bronze_dragon
splat_sources:
- title: SRD 5.2.1
  locator: pp. 271
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: gargantuan
  type: dragon
  alignment: Lawful Good
  tags_text: (Metallic)
  movement_text: 40 ft., Fly 80 ft., Swim 40 ft.
  skills_text: Insight +10, Perception +17, Stealth +7
  immunities_text: Lightning
  senses_text: Blindsight 60 ft., Darkvision 120 ft.; Passive Perception 27
  languages_text: Common, Draconic
  cr_text: 22 (XP 41,000, or 50,000 in lair; PB +7)
  hit_points_text: 444 (24d20 + 192)
splat_stats:
  strength:
    value: 29
  save_strength:
    value: 9
  dexterity:
    value: 10
  save_dexterity:
    value: 7
  constitution:
    value: 27
  save_constitution:
    value: 8
  intelligence:
    value: 18
  save_intelligence:
    value: 4
  wisdom:
    value: 17
  save_wisdom:
    value: 10
  charisma:
    value: 25
  save_charisma:
    value: 7
  armor_class:
    value: 22
  max_hp:
    value: 444
  initiative:
    value: 14
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
  mechanic: dnd2024:creatures/ancient_bronze_dragon
  implementation: dnd2024:creature_ancient_bronze_dragon
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
        attack_bonus: 16
        targets:
          count: 1
        hit:
        - op: damage
          average: 18
          amount:
            dice:
              count: 2
              sides: 8
            add: 9
          damage_type: slashing
        - op: damage
          average: 9
          amount:
            dice:
              count: 2
              sides: 8
            add: 0
          damage_type: lightning
        reach_feet: 15
      lightning_breath:
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
        dc: 23
        targets:
          kind: creature
          count: all
          area:
            shape: line
            size_feet: 120
            origin: self
            width_feet: 10
        failure:
        - op: damage
          average: 82
          amount:
            dice:
              count: 15
              sides: 10
            add: 0
          damage_type: lightning
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
          - feature: repulsion_breath
          - feature: spellcasting
            spell:
              key: dnd2024:spells/guiding_bolt
            cast_level: 2
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
            key: dnd2024:spells/guiding_bolt
          frequency: at_will
          cast_level: 2
        - spell:
            key: dnd2024:spells/shapechange
          frequency: at_will
          overrides:
            form_types:
            - beast
            - humanoid
            temporary_hit_points_gained: 0
            concentration_required: false
            temporary_hit_points_to_maintain_required: false
          cast_level: spell_minimum
        - spell:
            key: dnd2024:spells/speak_with_animals
          frequency: at_will
          cast_level: spell_minimum
        - spell:
            key: dnd2024:spells/thaumaturgy
          frequency: at_will
          cast_level: spell_minimum
        - spell:
            key: dnd2024:spells/detect_thoughts
          frequency: daily
          uses: 1
          resource_scope: per_spell
          recovery: long_rest
          cast_level: spell_minimum
        - spell:
            key: dnd2024:spells/control_water
          frequency: daily
          uses: 1
          resource_scope: per_spell
          recovery: long_rest
          cast_level: spell_minimum
        - spell:
            key: dnd2024:spells/scrying
          frequency: daily
          uses: 1
          resource_scope: per_spell
          recovery: long_rest
          cast_level: spell_minimum
        - spell:
            key: dnd2024:spells/water_breathing
          frequency: daily
          uses: 1
          resource_scope: per_spell
          recovery: long_rest
          cast_level: spell_minimum
        components_waived:
        - material
        save_dc: 22
        spell_attack_bonus: 14
      thunderclap:
        economy: legendary_action
        op: save
        ability: constitution
        dc: 22
        targets:
          kind: creature
          count: all
          area:
            shape: sphere
            size_feet: 20
            origin: chosen_point
            origin_visible_to: self
            origin_range_feet: 120
        failure:
        - op: damage
          average: 13
          amount:
            dice:
              count: 3
              sides: 8
            add: 0
          damage_type: thunder
        - op: condition
          condition: deafened
          ends:
            event: turn.end
            participant: target
            occurrence: next
        success: []
      repulsion_breath:
        economy: action
        op: save
        ability: strength
        dc: 23
        targets:
          kind: creature
          count: all
          area:
            shape: cone
            size_feet: 30
            origin: self
        failure:
        - op: push
          distance_max_feet: 60
          direction: straight_away_from_self
        - op: condition
          condition: prone
        success: []
      guiding_light:
        economy: legendary_action
        op: cast_spell
        spell:
          key: dnd2024:spells/guiding_bolt
        via: spellcasting
        cast_level: 2
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
      insight: 10
      perception: 17
      stealth: 7
    defenses:
      immunities:
        damage_types:
        - lightning
    senses:
      passive_perception: 27
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
- id: dnd2024:creature_ancient_bronze_dragon
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
  source_sha256: 702d08850e2d4e12fa5bb27fc8d4be56f61f39b2690f2d5348b5d4103a7fda0a
  source_path: builtin/srd-5.2.1/creature/ancient_bronze_dragon.md
  pages:
  - 271
  engine_gaps: &id001
  - dnd2024:creature_ancient_bronze_dragon
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_2.md
  specification_sha256: 7f08b46979265ae73e601cac82fa360dd771bb0ea0087def29ae04e441a07772
splat_clauses:
- key: source_rules
  text: '# Ancient Bronze Dragon Gargantuan Dragon (Metallic), Lawful Good AC 22 Initiative +14 (24) HP
    444 (24d20 + 192) Speed 40 ft., Fly 80 ft., Swim 40 ft. MOD SAVE MOD SAVE MOD SAVE Str 29 +9 +9 Dex
    10 +0 +7 Con 27 +8 +8 Int 18 +4 +4 Wis 17 +3 +10 Cha 25 +7 +7 Skills Insight +10, Perception +17,
    Stealth +7 Immunities Lightning Senses Blindsight 60 ft., Darkvision 120 ft.; Passive Perception 27
    Languages Common, Draconic CR 22 (XP 41,000, or 50,000 in lair; PB +7) ### Traits Amphibious. The
    dragon can breathe air and water. Legendary Resistance (4/Day, or 5/Day in Lair). If the dragon fails
    a saving throw, it can choose to succeed instead. ### Actions Multiattack. The dragon makes three
    Rend attacks. It can replace one attack with a use of (A) Repulsion Breath or (B) Spellcasting to
    cast Guiding Bolt (level 2 version). Rend. Melee Attack Roll: +16, reach 15 ft. Hit: 18 (2d8 + 9)
    Slashing damage plus 9 (2d8) Lightning damage. Lightning Breath (Recharge 5–6). Dexterity Saving Throw:
    DC 23, each creature in a 120-foot-long, 10-foot-wide Line. Failure: 82 (15d10) Lightning damage.
    Success: Half damage. Repulsion Breath. Strength Saving Throw: DC 23, each creature in a 30-foot Cone.
    Failure: The target is pushed up to 60 feet straight away from the dragon and has the Prone condition.
    Spellcasting. The dragon casts one of the following spells, requiring no Material components and using
    Charisma as the spellcasting ability (spell save DC 22, +14 to hit with spell attacks): At Will: Detect
    Magic, Guiding Bolt (level 2 version), Shapechange (Beast or Humanoid form only, no Temporary Hit
    Points gained from the spell, and no Concentration or Temporary Hit Points required to maintain the
    spell), Speak with Animals, Thaumaturgy 1/Day Each: Detect Thoughts, Control Water, Scrying, Water
    Breathing ### Legendary Actions Legendary Action Uses: 3 (4 in Lair). Immediately after another creature’s
    turn, the dragon can expend a use to take one of the following actions. The dragon regains all expended
    uses at the start of each of its turns. Guiding Light. The dragon uses Spellcasting to cast Guiding
    Bolt (level 2 version). Pounce. The dragon moves up to half its Speed, and it makes one Rend attack.
    Thunderclap. Constitution Saving Throw: DC 22, each creature in a 20-foot-radius Sphere centered on
    a point the dragon can see within 120 feet. Failure: 13 (3d8) Thunder damage, and the target has the
    Deafened condition until the end of its next turn.'
  sha256: bb3e1331cd82584ccf919367f41151931cee5506bc95d30783f69abe40bc7ffa
  status: contract_specified
splat_requires_mechanics: *id001
---

# Ancient Bronze Dragon

<!-- SRD page 271 -->

Gargantuan Dragon (Metallic), Lawful Good\
AC 22 Initiative +14 (24)\
HP 444 (24d20 + 192)\
Speed 40 ft., Fly 80 ft., Swim 40 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 29 +9 +9 Dex 10 +0 +7 Con 27 +8 +8\
Int 18 +4 +4 Wis 17 +3 +10 Cha 25 +7 +7\
Skills Insight +10, Perception +17, Stealth +7\
Immunities Lightning\
Senses Blindsight 60 ft., Darkvision 120 ft.;\
Passive Perception 27\
Languages Common, Draconic\
CR 22 (XP 41,000, or 50,000 in lair; PB +7)\

### Traits

Amphibious. The dragon can breathe air and water.\
Legendary Resistance (4/Day, or 5/Day in Lair). If\
the dragon fails a saving throw, it can choose to suc-\
ceed instead.\

### Actions

Multiattack. The dragon makes three Rend attacks.\
It can replace one attack with a use of (A) Repulsion\
Breath or (B) Spellcasting to cast Guiding Bolt (level\
2 version).\
Rend. Melee Attack Roll: +16, reach 15 ft. Hit: 18 (2d8\
+ 9) Slashing damage plus 9 (2d8) Lightning damage.\
Lightning Breath (Recharge 5–6). Dexterity Saving\
Throw: DC 23, each creature in a 120-foot-long,\
10-foot-wide Line. Failure: 82 (15d10) Lightning dam-\
age. Success: Half damage.\
Repulsion Breath. Strength Saving Throw: DC 23, each\
creature in a 30-foot Cone. Failure: The target is pushed\
up to 60 feet straight away from the dragon and has the\
Prone condition.\
Spellcasting. The dragon casts one of the following\
spells, requiring no Material components and using\
Charisma as the spellcasting ability (spell save DC 22,\
+14 to hit with spell attacks):\
At Will: Detect Magic, Guiding Bolt (level 2 version),\
Shapechange (Beast or Humanoid form only, no\
Temporary Hit Points gained from the spell, and no\
Concentration or Temporary Hit Points required to\
maintain the spell), Speak with Animals, Thaumaturgy\
1/Day Each: Detect Thoughts, Control Water, Scrying,\
Water Breathing\

### Legendary Actions

Legendary Action Uses: 3 (4 in Lair). Immediately after\
another creature’s turn, the dragon can expend a use to\
take one of the following actions. The dragon regains all\
expended uses at the start of each of its turns.\
Guiding Light. The dragon uses Spellcasting to cast\
Guiding Bolt (level 2 version).\
Pounce. The dragon moves up to half its Speed, and it\
makes one Rend attack.\
Thunderclap. Constitution Saving Throw: DC 22, each\
creature in a 20-foot-radius Sphere centered on a point\
the dragon can see within 120 feet. Failure: 13 (3d8)\
Thunder damage, and the target has the Deafened con-\
dition until the end of its next turn.\

Original layout: [[../../srd-5.2.1/pages/271|page 271]].
