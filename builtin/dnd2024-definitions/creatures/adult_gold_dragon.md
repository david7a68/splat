---
splat_schema: 1
splat_id: 9705a4e4-6d7c-5510-91bb-86a3686c0c3d
splat_kind: creature
splat_key: dnd2024:creatures/adult_gold_dragon
splat_name: Adult Gold Dragon
splat_origin:
  key: dnd2024:reference/creature/adult_gold_dragon
splat_sources:
- title: SRD 5.2.1
  locator: pp. 291, 292
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: huge
  type: dragon
  alignment: Lawful Good
  tags_text: (Metallic)
  movement_text: 40 ft., Fly 80 ft., Swim 40 ft.
  skills_text: Insight +8, Perception +14, Persuasion +13, Stealth +8
  immunities_text: Fire
  senses_text: Blindsight 60 ft., Darkvision 120 ft.; Passive Perception 24
  languages_text: Common, Draconic
  cr_text: 17 (XP 18,000, or 20,000 in lair; PB +6)
  hit_points_text: 243 (18d12 + 126)
splat_stats:
  strength:
    value: 27
  save_strength:
    value: 8
  dexterity:
    value: 14
  save_dexterity:
    value: 8
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
    value: 8
  charisma:
    value: 24
  save_charisma:
    value: 7
  armor_class:
    value: 19
  max_hp:
    value: 243
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
  mechanic: dnd2024:creatures/adult_gold_dragon
  implementation: dnd2024:creature_adult_gold_dragon
  parameters:
    features:
      amphibious:
        economy: trait
        operation: passive_rule
        breathing:
        - air
        - water
      legendary_resistance:
        economy: trait
        resource:
          kind: daily
          uses: 3
          recovery: long_rest
          lair_uses: 4
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
              feature: weakening_breath
            - operation: cast_spell
              spell:
                key: dnd2024:spells/guiding_bolt
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
          supplied_attack_total: 14
          reach_ft: 10
        hit:
          damage:
          - average: 17
            roll:
              count: 2
              sides: 8
              modifier: 8
            type: slashing
          - average: 4
            roll:
              count: 1
              sides: 8
              modifier: 0
            type: fire
      fire_breath:
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
        ability: dexterity
        dc: 21
        targets:
          kind: creature
          selection: each
          area:
            shape: cone
            size_ft: 60
            origin: self
        failure:
          damage:
          - average: 66
            roll:
              count: 12
              sides: 10
              modifier: 0
            type: fire
        success:
          damage_multiplier: 0.5
          other_effects: false
      spellcasting:
        economy: action
        operation: spellcasting
        ability: charisma
        components_waived:
        - material
        save_dc: 21
        supplied_spell_attack_total: 13
        options:
        - spell:
            key: dnd2024:spells/detect_magic
          usage:
            kind: at_will
        - spell:
            key: dnd2024:spells/guiding_bolt
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
            key: dnd2024:spells/flame_strike
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
      weakening_breath:
        economy: action
        operation: saving_throw
        ability: strength
        dc: 21
        targets:
          kind: creature
          selection: each
          area:
            shape: cone
            size_ft: 60
            origin: self
          exclude_already_affected_by: this_breath
        failure:
          disadvantage_on:
            roll: d20_test
            ability: strength
          damage_roll_modifier:
            subtract:
              count: 1
              sides: 6
              modifier: 0
          repeat_save:
            event: target.turn_end
            success: end_effect
          automatic_success_after_seconds: 60
        success:
          effects: []
      banish:
        economy: legendary_action
        legendary_cost: 1
        cooldown_until: self.next_turn_start
        operation: saving_throw
        ability: charisma
        dc: 21
        targets:
          kind: creature
          count: 1
          range_ft: 120
          visible: true
        failure:
          damage:
          - average: 10
            roll:
              count: 3
              sides: 6
              modifier: 0
            type: force
          conditions:
          - condition: incapacitated
            until: self.next_turn_start
          transport:
            plane: harmless_demiplane
            until: self.next_turn_start
            return:
              unoccupied: true
              within_self_ft: 120
              chooser: self
        success:
          effects: []
      guiding_light:
        economy: legendary_action
        legendary_cost: 1
        operation: cast_spell
        spell:
          key: dnd2024:spells/guiding_bolt
        cast_level: 2
        spellcasting_profile: spellcasting
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
      insight: 8
      perception: 14
      persuasion: 13
      stealth: 8
    damage_immunities:
    - fire
    condition_immunities: []
    senses:
      blindsight: 60
      darkvision: 120
      passive_perception: 24
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
- id: dnd2024:creature_adult_gold_dragon
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
  - spatial_queries
  - limited_resources
  - legendary_actions
  - hit_points
  - defenses
splat_conversion:
  status: specified
  category: creature
  source_sha256: 0764b0e63690e940b17950d61d0770dfd12e86c166a9f5bae694c0fe28fa08f0
  source_path: builtin/srd-5.2.1/creature/adult_gold_dragon.md
  pages:
  - 291
  - 292
  engine_gaps: &id001
  - dnd2024:creature_adult_gold_dragon
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_1.md
  specification_sha256: 538be546164f73fa194517766b185529950d50c35b0abf10784b488eff5f7f56
splat_clauses:
- key: source_rules
  text: '# Adult Gold Dragon Huge Dragon (Metallic), Lawful Good AC 19 Initiative +14 (24) HP 243 (18d12
    + 126) Speed 40 ft., Fly 80 ft., Swim 40 ft. MOD SAVE MOD SAVE MOD SAVE Str 27 +8 +8 Dex 14 +2 +8
    Con 25 +7 +7 Int 16 +3 +3 Wis 15 +2 +8 Cha 24 +7 +7 Skills Insight +8, Perception +14, Persuasion
    +13, Stealth +8 Immunities Fire Senses Blindsight 60 ft., Darkvision 120 ft.; Passive Perception 24
    Languages Common, Draconic CR 17 (XP 18,000, or 20,000 in lair; PB +6) ### Traits Amphibious. The
    dragon can breathe air and water. Legendary Resistance (3/Day, or 4/Day in Lair). If the dragon fails
    a saving throw, it can choose to succeed instead. ### Actions Multiattack. The dragon makes three
    Rend attacks. It can replace one attack with a use of (A) Spellcasting to cast Guiding Bolt (level
    2 version) or (B) Weakening Breath. Rend. Melee Attack Roll: +14, reach 10 ft. Hit: 17 (2d8 + 8) Slashing
    damage plus 4 (1d8) Fire damage. Fire Breath (Recharge 5–6). Dexterity Saving Throw: DC 21, each creature
    in a 60-foot Cone. Failure: 66 (12d10) Fire damage. Success: Half damage. Spellcasting. The dragon
    casts one of the following spells, requiring no Material components and using Charisma as the spellcasting
    ability (spell save DC 21, +13 to hit with spell attacks): At Will: Detect Magic, Guiding Bolt (level
    2 version), Shapechange (Beast or Humanoid form only, no Temporary Hit Points gained from the spell,
    and no Concentration or Temporary Hit Points required to maintain the spell) 1/Day Each: Flame Strike,
    Zone of Truth Weakening Breath. Strength Saving Throw: DC 21, each creature that isn’t currently affected
    by this breath in a 60-foot Cone. Failure: The target has Disadvantage on Strength-based D20 Tests
    and subtracts 3 (1d6) from its damage rolls. It repeats the save at the end of each of its turns,
    ending the effect on itself on a success. After 1 minute, it succeeds automatically. ### Legendary
    Actions Legendary Action Uses: 3 (4 in Lair). Immediately after another creature’s turn, the dragon
    can expend a use to take one of the following actions. The dragon regains all expended uses at the
    start of each of its turns. Banish. Charisma Saving Throw: DC 21, one creature the dragon can see
    within 120 feet. Failure: 10 (3d6) Force damage, and the target has the Incapacitated condition and
    is transported to a harmless demiplane until the start of the dragon’s next turn, at which point it
    re appears in an unoccupied space of the dragon’s choice within 120 feet of the dragon. Failure or
    Success: The dragon can’t take this action again until the start of its next turn. Guiding Light.
    The dragon uses Spellcasting to cast Guiding Bolt (level 2 version). Pounce. The dragon moves up to
    half its Speed, and it makes one Rend attack.'
  sha256: 4420165a25001552d6103637fe671461bec49103f378f516ac2e86bc5270cec2
  status: contract_specified
splat_requires_mechanics: *id001
---

# Adult Gold Dragon

<!-- SRD page 291 -->

Huge Dragon (Metallic), Lawful Good\
AC 19 Initiative +14 (24)\
HP 243 (18d12 + 126)\
Speed 40 ft., Fly 80 ft., Swim 40 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 27 +8 +8 Dex 14 +2 +8 Con 25 +7 +7\
Int 16 +3 +3 Wis 15 +2 +8 Cha 24 +7 +7\
Skills Insight +8, Perception +14, Persuasion +13, Stealth\
+8\
Immunities Fire\
Senses Blindsight 60 ft., Darkvision 120 ft.;\
Passive Perception 24\
Languages Common, Draconic\
CR 17 (XP 18,000, or 20,000 in lair; PB +6)\

### Traits

Amphibious. The dragon can breathe air and water.\
Legendary Resistance (3/Day, or 4/Day in Lair). If\
the dragon fails a saving throw, it can choose to suc-\
ceed instead.\

### Actions

Multiattack. The dragon makes three Rend attacks. It\
can replace one attack with a use of (A) Spellcasting\
to cast Guiding Bolt (level 2 version) or (B) Weaken-\
ing Breath.\
Rend. Melee Attack Roll: +14, reach 10 ft. Hit: 17 (2d8\
+ 8) Slashing damage plus 4 (1d8) Fire damage.\
Fire Breath (Recharge 5–6). Dexterity Saving Throw:\
DC 21, each creature in a 60-foot Cone. Failure: 66\
(12d10) Fire damage. Success: Half damage.\
Spellcasting. The dragon casts one of the following\
spells, requiring no Material components and using\
Charisma as the spellcasting ability (spell save DC 21,\
+13 to hit with spell attacks):\
At Will: Detect Magic, Guiding Bolt (level 2 version),\
Shapechange (Beast or Humanoid form only, no\
Temporary Hit Points gained from the spell, and no\
Concentration or Temporary Hit Points required to\
maintain the spell)\
1/Day Each: Flame Strike, Zone of Truth\
Weakening Breath. Strength Saving Throw: DC 21,\
each creature that isn’t currently affected by this breath\
in a 60-foot Cone. Failure: The target has Disadvantage\
on Strength-based D20 Tests and subtracts 3 (1d6) from\
its damage rolls. It repeats the save at the end of each\
of its turns, ending the effect on itself on a success. Af-\
ter 1 minute, it succeeds automatically.\

<!-- SRD page 292 -->


### Legendary Actions

Legendary Action Uses: 3 (4 in Lair). Immediately after\
another creature’s turn, the dragon can expend a use to\
take one of the following actions. The dragon regains all\
expended uses at the start of each of its turns.\
Banish. Charisma Saving Throw: DC 21, one creature\
the dragon can see within 120 feet. Failure: 10 (3d6)\
Force damage, and the target has the Incapacitated\
condition and is transported to a harmless demiplane\
until the start of the dragon’s next turn, at which point\
it re appears in an unoccupied space of the dragon’s\
choice within 120 feet of the dragon. Failure or Success:\
The dragon can’t take this action again until the start of\
its next turn.\
Guiding Light. The dragon uses Spellcasting to cast\
Guiding Bolt (level 2 version).\
Pounce. The dragon moves up to half its Speed, and it\
makes one Rend attack.\

Original layout: [[../../srd-5.2.1/pages/291|page 291]], [[../../srd-5.2.1/pages/292|page 292]].
