---
splat_schema: 1
splat_id: d72a9024-caed-5593-870f-acc73edd73e4
splat_kind: creature
splat_key: dnd2024:creatures/ancient_green_dragon
splat_name: Ancient Green Dragon
splat_origin:
  key: dnd2024:reference/creature/ancient_green_dragon
splat_sources:
- title: SRD 5.2.1
  locator: pp. 294, 295
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: gargantuan
  type: dragon
  alignment: Lawful Evil
  tags_text: (Chromatic)
  movement_text: 40 ft., Fly 80 ft., Swim 40 ft.
  skills_text: Deception +13, Perception +17, Persuasion +13, Stealth +8
  immunities_text: Poison; Poisoned
  senses_text: Blindsight 60 ft., Darkvision 120 ft.; Passive Perception 27
  languages_text: Common, Draconic
  cr_text: 22 (XP 41,000, or 50,000 in lair; PB +7)
  hit_points_text: 402 (23d20 + 161)
splat_stats:
  strength:
    value: 27
  save_strength:
    value: 8
  dexterity:
    value: 12
  save_dexterity:
    value: 8
  constitution:
    value: 25
  save_constitution:
    value: 7
  intelligence:
    value: 20
  save_intelligence:
    value: 5
  wisdom:
    value: 17
  save_wisdom:
    value: 10
  charisma:
    value: 22
  save_charisma:
    value: 6
  armor_class:
    value: 21
  max_hp:
    value: 402
  initiative:
    value: 15
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
  mechanic: dnd2024:creatures/ancient_green_dragon
  implementation: dnd2024:creature_ancient_green_dragon
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
          average: 10
          amount:
            dice:
              count: 3
              sides: 6
            add: 0
          damage_type: poison
        reach_feet: 15
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
        dc: 22
        targets:
          kind: creature
          count: all
          area:
            shape: cone
            size_feet: 90
            origin: self
        failure:
        - op: damage
          average: 77
          amount:
            dice:
              count: 22
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
            cast_level: 5
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
          cast_level: 5
        - spell:
            key: dnd2024:spells/geas
          frequency: daily
          uses: 1
          resource_scope: per_spell
          recovery: long_rest
          cast_level: spell_minimum
        - spell:
            key: dnd2024:spells/modify_memory
          frequency: daily
          uses: 1
          resource_scope: per_spell
          recovery: long_rest
          cast_level: spell_minimum
        components_waived:
        - material
        save_dc: 21
      noxious_miasma:
        economy: legendary_action
        op: save
        ability: constitution
        dc: 21
        targets:
          kind: creature
          count: all
          area:
            shape: sphere
            size_feet: 30
            origin: chosen_point
            origin_visible_to: self
            origin_range_feet: 90
        failure:
        - op: damage
          average: 17
          amount:
            dice:
              count: 5
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
        cast_level: 5
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
      deception: 13
      perception: 17
      persuasion: 13
      stealth: 8
    defenses:
      immunities:
        damage_types:
        - poison
        conditions:
        - poisoned
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
- id: dnd2024:creature_ancient_green_dragon
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
  source_sha256: d8024b63c63fc2a2400b52dd8135f6c6ef50f96b6d5b0662d74674a1e9f14294
  source_path: builtin/srd-5.2.1/creature/ancient_green_dragon.md
  pages:
  - 294
  - 295
  engine_gaps: &id001
  - dnd2024:creature_ancient_green_dragon
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_2.md
  specification_sha256: 4af788e4cccfd35bb105334db5178fe0cbfb39e6a27e7bc184a0baf63c8c9d75
splat_clauses:
- key: source_rules
  text: '# Ancient Green Dragon Gargantuan Dragon (Chromatic), Lawful Evil AC 21 Initiative +15 (25) HP
    402 (23d20 + 161) Speed 40 ft., Fly 80 ft., Swim 40 ft. MOD SAVE MOD SAVE MOD SAVE Str 27 +8 +8 Dex
    12 +1 +8 Con 25 +7 +7 Int 20 +5 +5 Wis 17 +3 +10 Cha 22 +6 +6 Skills Deception +13, Perception +17,
    Persuasion +13, Stealth +8 Immunities Poison; Poisoned Senses Blindsight 60 ft., Darkvision 120 ft.;
    Passive Perception 27 Languages Common, Draconic CR 22 (XP 41,000, or 50,000 in lair; PB +7) ### Traits
    Amphibious. The dragon can breathe air and water. Legendary Resistance (4/Day, or 5/Day in Lair).
    If the dragon fails a saving throw, it can choose to succeed instead. ### Actions Multiattack. The
    dragon makes three Rend attacks. It can replace one attack with a use of Spellcasting to cast Mind
    Spike (level 5 version). Rend. Melee Attack Roll: +15, reach 15 ft. Hit: 17 (2d8 + 8) Slashing damage
    plus 10 (3d6) Poison damage. Poison Breath (Recharge 5–6). Constitution Saving Throw: DC 22, each
    creature in a 90-foot Cone. Failure: 77 (22d6) Poison damage. Success: Half damage. Spellcasting.
    The dragon casts one of the following spells, requiring no Material components and using Charisma
    as the spellcasting ability (spell save DC 21): At Will: Detect Magic, Mind Spike (level 5 version)
    1/Day Each: Geas, Modify Memory ### Legendary Actions Legendary Action Uses: 3 (4 in Lair). Immediately
    after another creature’s turn, the dragon can expend a use to take one of the following actions. The
    dragon regains all expended uses at the start of each of its turns. Mind Invasion. The dragon uses
    Spellcasting to cast Mind Spike (level 5 version). Noxious Miasma. Constitution Saving Throw: DC 21,
    each creature in a 30-foot-radius Sphere centered on a point the dragon can see within 90 feet. Failure:
    17 (5d6) Poison damage, and the target takes a -2 penalty to AC until the end of its next turn. Failure
    or Success: The dragon can’t take this action again until the start of its next turn. Pounce. The
    dragon moves up to half its Speed, and it makes one Rend attack.'
  sha256: 48ade20e03d9458374b09e2b932d29a4685fc831e831663cf40c48cef6840b97
  status: contract_specified
splat_requires_mechanics: *id001
---

# Ancient Green Dragon

<!-- SRD page 294 -->

Gargantuan Dragon (Chromatic), Lawful Evil\
AC 21 Initiative +15 (25)\
HP 402 (23d20 + 161)\
Speed 40 ft., Fly 80 ft., Swim 40 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 27 +8 +8 Dex 12 +1 +8 Con 25 +7 +7\
Int 20 +5 +5 Wis 17 +3 +10 Cha 22 +6 +6\
Skills Deception +13, Perception +17, Persuasion +13,\
Stealth +8\
Immunities Poison; Poisoned\
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

Multiattack. The dragon makes three Rend attacks. It\
can replace one attack with a use of Spellcasting to cast\
Mind Spike (level 5 version).\
Rend. Melee Attack Roll: +15, reach 15 ft. Hit: 17 (2d8\
+ 8) Slashing damage plus 10 (3d6) Poison damage.\
Poison Breath (Recharge 5–6). Constitution Saving\
Throw: DC 22, each creature in a 90-foot Cone. Fail-\
ure: 77 (22d6) Poison damage. Success: Half damage.\
Spellcasting. The dragon casts one of the following\
spells, requiring no Material components and using\
Charisma as the spellcasting ability (spell save DC 21):\
At Will: Detect Magic, Mind Spike (level 5 version)\
1/Day Each: Geas, Modify Memory\

<!-- SRD page 295 -->


### Legendary Actions

Legendary Action Uses: 3 (4 in Lair). Immediately after\
another creature’s turn, the dragon can expend a use to\
take one of the following actions. The dragon regains all\
expended uses at the start of each of its turns.\
Mind Invasion. The dragon uses Spellcasting to cast\
Mind Spike (level 5 version).\
Noxious Miasma. Constitution Saving Throw: DC 21,\
each creature in a 30-foot-radius Sphere centered on\
a point the dragon can see within 90 feet. Failure: 17\
(5d6) Poison damage, and the target takes a −2 penalty\
to AC until the end of its next turn. Failure or Success:\
The dragon can’t take this action again until the start of\
its next turn.\
Pounce. The dragon moves up to half its Speed, and it\
makes one Rend attack.\

Original layout: [[../../srd-5.2.1/pages/294|page 294]], [[../../srd-5.2.1/pages/295|page 295]].
