---
splat_schema: 1
splat_id: e528d14d-5b02-5a4a-abbc-d72f5aaa14cf
splat_kind: creature
splat_key: dnd2024:creatures/ancient_silver_dragon
splat_name: Ancient Silver Dragon
splat_origin:
  key: dnd2024:reference/creature/ancient_silver_dragon
splat_sources:
- title: SRD 5.2.1
  locator: pp. 325
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: gargantuan
  type: dragon
  alignment: Lawful Good
  tags_text: (Metallic)
  movement_text: 40 ft., Fly 80 ft.
  skills_text: History +11, Perception +16, Stealth +7
  immunities_text: Cold
  senses_text: Blindsight 60 ft., Darkvision 120 ft.; Passive Perception 26
  languages_text: Common, Draconic
  cr_text: 23 (XP 50,000, or 62,000 in lair; PB +7)
  hit_points_text: 468 (24d20 + 216)
splat_stats:
  strength:
    value: 30
  save_strength:
    value: 10
  dexterity:
    value: 10
  save_dexterity:
    value: 7
  constitution:
    value: 29
  save_constitution:
    value: 9
  intelligence:
    value: 18
  save_intelligence:
    value: 4
  wisdom:
    value: 15
  save_wisdom:
    value: 9
  charisma:
    value: 26
  save_charisma:
    value: 8
  armor_class:
    value: 22
  max_hp:
    value: 468
  initiative:
    value: 14
  speed:
    value: 40
    unit: ft
  fly_speed:
    value: 80
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/ancient_silver_dragon
  implementation: dnd2024:creature_ancient_silver_dragon
  parameters:
    features:
      legendary_resistance:
        economy: trait
        resource:
          kind: daily
          uses: 4
          recovery: long_rest
          lair_uses: 5
        operation: replace_save_result
        trigger: self.save_failed
        optional: true
        result: success
      multiattack:
        economy: action
        operation: multiattack
        sequence:
        - operation: attack_sequence
          feature: rend
          count: 3
        replacements:
        - replace_count: 1
          maximum_uses: 1
          operation:
            operation: choice
            options:
            - operation: use_feature
              feature: paralyzing_breath
            - operation: cast_spell
              spell:
                key: dnd2024:spells/ice_knife
              cast_level: 2
              spellcasting_profile: spellcasting
      rend:
        economy: action
        operation: attack
        attack:
          mode: melee
          targets:
            count: 1
            kind: creature_or_object
          supplied_attack_total: 17
          reach_ft: 15
        hit:
          damage:
          - average: 19
            roll:
              count: 2
              sides: 8
              modifier: 10
            type: slashing
          - average: 9
            roll:
              count: 2
              sides: 8
              modifier: 0
            type: cold
      cold_breath:
        economy: action
        resource:
          kind: recharge
          die: 6
          success_faces:
          - 5
          - 6
          check: self.turn_start
          rest_recovery:
          - short
          - long
          capacity: 1
          initial_uses: 1
        operation: saving_throw
        ability: constitution
        dc: 24
        targets:
          kind: creature
          selection: each
          area:
            shape: cone
            size_ft: 90
            origin: self
        failure:
          damage:
          - average: 67
            roll:
              count: 15
              sides: 8
              modifier: 0
            type: cold
        success:
          damage_multiplier: 0.5
          other_effects: false
      paralyzing_breath:
        economy: action
        operation: saving_throw
        ability: constitution
        dc: 24
        targets:
          kind: creature
          selection: each
          area:
            shape: cone
            size_ft: 90
            origin: self
        failure:
          damage: []
          conditions:
          - condition: incapacitated
            until: target.next_turn_end
          repeat_save:
            event: target.next_turn_end
            failure:
              conditions:
              - condition: paralyzed
              repeat_save:
                event: target.turn_end
                success: end_effect
              automatic_success_after_seconds: 60
              automatic_success_clock_origin: initial_failed_save
            success: end_effect
        success:
          effects: []
      spellcasting:
        economy: action
        operation: spellcasting
        ability: charisma
        components_waived:
        - material
        save_dc: 23
        supplied_spell_attack_total: 15
        options:
        - spell:
            key: dnd2024:spells/detect_magic
          usage:
            kind: at_will
        - spell:
            key: dnd2024:spells/hold_monster
          usage:
            kind: at_will
        - spell:
            key: dnd2024:spells/ice_knife
          usage:
            kind: at_will
          cast_level: 2
        - spell:
            key: dnd2024:spells/shapechange
          usage:
            kind: at_will
          overrides:
            allowed_types:
            - beast
            - humanoid
            gain_temporary_hp: false
            maintain_requires_concentration: false
            maintain_requires_temporary_hp: false
        - spell:
            key: dnd2024:spells/control_weather
          usage:
            kind: daily
            uses: 1
            pool: per_spell
            recovery: long_rest
        - spell:
            key: dnd2024:spells/ice_storm
          usage:
            kind: daily
            uses: 1
            pool: per_spell
            recovery: long_rest
          cast_level: 7
        - spell:
            key: dnd2024:spells/teleport
          usage:
            kind: daily
            uses: 1
            pool: per_spell
            recovery: long_rest
        - spell:
            key: dnd2024:spells/zone_of_truth
          usage:
            kind: daily
            uses: 1
            pool: per_spell
            recovery: long_rest
        material_components_available: true
      chill:
        economy: legendary_action
        legendary_cost: 1
        cooldown_until: self.next_turn_start
        operation: cast_spell
        spell:
          key: dnd2024:spells/hold_monster
        spellcasting_profile: spellcasting
      cold_gale:
        economy: legendary_action
        legendary_cost: 1
        cooldown_until: self.next_turn_start
        operation: saving_throw
        ability: dexterity
        dc: 23
        targets:
          kind: creature
          selection: each
          area:
            shape: line
            size_ft: 60
            origin: self
            width_ft: 10
        failure:
          damage:
          - average: 14
            roll:
              count: 4
              sides: 6
              modifier: 0
            type: cold
          push:
            maximum_ft: 30
            direction: straight_away_from_self
        success:
          damage_multiplier: 0.5
          other_effects: false
      pounce:
        economy: legendary_action
        legendary_cost: 1
        operation: sequence
        effects:
        - operation: move
          maximum_ft:
            multiply:
            - stat: speed
            - 0.5
        - operation: attack_sequence
          feature: rend
          count: 1
    skill_totals:
      history: 11
      perception: 16
      stealth: 7
    damage_immunities:
    - cold
    condition_immunities: []
    senses:
      blindsight: 60
      darkvision: 120
      passive_perception: 26
    languages:
      speaks:
      - common
      - draconic
      understands: []
    legendary_actions:
      uses: 3
      recovery: self.turn_start
      trigger: other_creature.turn_end
      cost_per_action: 1
      lair_uses: 4
      requires:
        not_incapacitated: true
      maximum_actions_per_trigger: 1
    resolution_conventions:
      distance_unit: foot
      duration_unit: second
      damage_roll_mode:
        choice:
        - listed_average
        - roll_dice
        chooser: gm
      supplied_totals: before_additional_effect_modifiers
      multiattack_nested_economy_cost: 0
      nested_feature_resource_cost: normal
      spell_level_unless_specified: lowest_possible
      spell_upcast_without_permission: false
      spell_casting_time_at_least_one_minute: normal_casting_procedure
      geometry:
        input: adjudication
        assumptions:
        - range_and_visibility_verified
        - area_membership_verified
        - path_and_destination_legal
      unlisted_success_branch: no_effect
      unlisted_duration: until_removed_by_applicable_rule
splat_contracts:
- id: dnd2024:creature_ancient_silver_dragon
  inputs:
    actor:
      type: participant
      required: true
    feature:
      required: false
      type: choice
    targets:
      required: false
      type: participant_list
    objects:
      required: false
      type: object_list
    destination:
      required: false
      type: location
    adjudication:
      required: false
      type: resolution
    trigger:
      required: false
      type: event
  outputs:
  - effects
  events:
  - creature.feature_requested
  - turn.started
  - turn.ended
  - rest.completed
  - saving_throw.resolved
  capabilities:
  - creature_features
  - action_economy
  - attacks
  - saving_throws
  - conditions
  - damage
  - spellcasting
  - teleportation
  - spatial_queries
  - limited_resources
  - legendary_actions
  - hit_points
  - defenses
splat_conversion:
  status: specified
  category: creature
  source_sha256: efde91130bb36a8aacca608b7c68cf8e921afeb7a94a2fd5d9dfcbecd93ea199
  source_path: builtin/srd-5.2.1/creature/ancient_silver_dragon.md
  pages:
  - 325
  engine_gaps: &id001
  - dnd2024:creature_ancient_silver_dragon
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_1.md
  specification_sha256: 0a30bf312739540bbd62ae54a7e9f161972cef7ee5a46258030f02d8318885d2
splat_clauses:
- key: source_rules
  text: '# Ancient Silver Dragon Gargantuan Dragon (Metallic), Lawful Good AC 22 Initiative +14 (24) HP
    468 (24d20 + 216) Speed 40 ft., Fly 80 ft. MOD SAVE MOD SAVE MOD SAVE Str 30 +10 +10 Dex 10 +0 +7
    Con 29 +9 +9 Int 18 +4 +4 Wis 15 +2 +9 Cha 26 +8 +8 Skills History +11, Perception +16, Stealth +7
    Immunities Cold Senses Blindsight 60 ft., Darkvision 120 ft.; Passive Perception 26 Languages Common,
    Draconic CR 23 (XP 50,000, or 62,000 in lair; PB +7) ### Traits Legendary Resistance (4/Day, or 5/Day
    in Lair). If the dragon fails a saving throw, it can choose to succeed instead. ### Actions Multiattack.
    The dragon makes three Rend attacks. It can replace one attack with a use of (A) Paralyzing Breath
    or (B) Spellcasting to cast Ice Knife (level 2 version). Rend. Melee Attack Roll: +17, reach 15 ft.
    Hit: 19 (2d8 + 10) Slashing damage plus 9 (2d8) Cold damage. Cold Breath (Recharge 5–6). Constitution
    Saving Throw: DC 24, each creature in a 90-foot Cone. Failure: 67 (15d8) Cold damage. Success: Half
    damage. Paralyzing Breath. Constitution Saving Throw: DC 24, each creature in a 90-foot Cone. First
    Failure: The target has the Incapacitated condition until the end of its next turn, when it repeats
    the save. Second Failure: The target has the Paralyzed condition, and it repeats the save at the end
    of each of its turns, ending the effect on itself on a success. After 1 minute, it succeeds automatically.
    Spellcasting. The dragon casts one of the following spells, requiring no Material components and using
    Charisma as the spellcasting ability (spell save DC 23, +15 to hit with spell attacks): At Will: Detect
    Magic, Hold Monster, Ice Knife (level 2 version), Shapechange (Beast or Humanoid form only, no Temporary
    Hit Points gained from the spell, and no Concentration or Temporary Hit Points required to maintain
    the spell) 1/Day Each: Control Weather, Ice Storm (level 7 version), Teleport, Zone of Truth ### Legendary
    Actions Legendary Action Uses: 3 (4 in Lair). Immediately after another creature’s turn, the dragon
    can expend a use to take one of the following actions. The dragon regains all expended uses at the
    start of each of its turns. Chill. The dragon uses Spellcasting to cast Hold Monster. The dragon can’t
    take this action again until the start of its next turn. Cold Gale. Dexterity Saving Throw: DC 23,
    each creature in a 60-foot-long, 10-foot-wide Line. Failure: 14 (4d6) Cold damage, and the target
    is pushed up to 30 feet straight away from the dragon. Success: Half damage only. Failure or Success:
    The dragon can’t take this action again until the start of its next turn. Pounce. The dragon moves
    up to half its Speed, and it makes one Rend attack.'
  sha256: a3e3724622668b03431afc53f08b8f30e605bda04c4fec5158ef66e76db18011
  status: contract_specified
splat_requires_mechanics: *id001
---

# Ancient Silver Dragon

<!-- SRD page 325 -->

Gargantuan Dragon (Metallic), Lawful Good\
AC 22  Initiative +14 (24)\
HP 468 (24d20 + 216)\
Speed 40 ft., Fly 80 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 30 +10 +10 Dex 10 +0 +7 Con 29 +9 +9\
Int 18 +4 +4 Wis 15 +2 +9 Cha 26 +8 +8\
Skills History +11, Perception +16, Stealth +7\
Immunities Cold\
Senses Blindsight 60 ft., Darkvision 120 ft.;\
Passive Perception 26\
Languages Common, Draconic\
CR 23 (XP 50,000, or 62,000 in lair; PB +7)\

### Traits

Legendary Resistance (4/Day, or 5/Day in Lair). If\
the dragon fails a saving throw, it can choose to suc-\
ceed instead.\

### Actions

Multiattack. The dragon makes three Rend attacks.\
It can replace one attack with a use of (A) Paralyz-\
ing Breath or (B) Spellcasting to cast Ice Knife (level\
2 version).\
Rend. Melee Attack Roll: +17, reach 15 ft. Hit: 19 (2d8\
+ 10) Slashing damage plus 9 (2d8) Cold damage.\
Cold Breath (Recharge 5–6). Constitution Saving\
Throw: DC 24, each creature in a 90-foot Cone. Fail-\
ure: 67 (15d8) Cold damage. Success: Half damage.\
Paralyzing Breath. Constitution Saving Throw: DC 24,\
each creature in a 90-foot Cone. First Failure: The target\
has the Incapacitated condition until the end of its next\
turn, when it repeats the save. Second Failure: The tar-\
get has the Paralyzed condition, and it repeats the save\
at the end of each of its turns, ending the effect on itself\
on a success. After 1 minute, it succeeds automatically.\
Spellcasting. The dragon casts one of the following\
spells, requiring no Material components and using\
Charisma as the spellcasting ability (spell save DC 23,\
+15 to hit with spell attacks):\
At Will: Detect Magic, Hold Monster, Ice Knife (level 2\
version), Shapechange (Beast or Humanoid form only,\
no Temporary Hit Points gained from the spell, and\
no Concentration or Temporary Hit Points required to\
maintain the spell)\
1/Day Each: Control Weather, Ice Storm (level 7 ver-\
sion), Teleport, Zone of Truth\

### Legendary Actions

Legendary Action Uses: 3 (4 in Lair). Immediately after\
another creature’s turn, the dragon can expend a use to\
take one of the following actions. The dragon regains all\
expended uses at the start of each of its turns.\
Chill. The dragon uses Spellcasting to cast Hold Mon-\
ster. The dragon can’t take this action again until the\
start of its next turn.\
Cold Gale. Dexterity Saving Throw: DC 23, each crea-\
ture in a 60-foot-long, 10-foot-wide Line. Failure: 14\
(4d6) Cold damage, and the target is pushed up to 30\
feet straight away from the dragon. Success: Half dam-\
age only. Failure or Success: The dragon can’t take this\
action again until the start of its next turn.\
Pounce. The dragon moves up to half its Speed, and it\
makes one Rend attack.\

Original layout: [[../../srd-5.2.1/pages/325|page 325]].
