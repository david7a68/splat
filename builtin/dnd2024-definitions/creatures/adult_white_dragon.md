---
splat_schema: 1
splat_id: cbd73464-e151-5f76-b39b-cb9b2fb367ff
splat_kind: creature
splat_key: dnd2024:creatures/adult_white_dragon
splat_name: Adult White Dragon
splat_origin:
  key: dnd2024:reference/creature/adult_white_dragon
splat_sources:
- title: SRD 5.2.1
  locator: pp. 340
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: huge
  type: dragon
  alignment: Chaotic Evil
  tags_text: (Chromatic)
  movement_text: 40 ft., Burrow 30 ft., Fly 80 ft., Swim 40 ft.
  skills_text: Perception +11, Stealth +5
  immunities_text: Cold
  senses_text: Blindsight 60 ft., Darkvision 120 ft.; Passive Perception 21
  languages_text: Common, Draconic
  cr_text: 13 (XP 10,000, or 11,500 in lair; PB +5)
  hit_points_text: 200 (16d12 + 96)
splat_stats:
  strength:
    value: 22
  save_strength:
    value: 6
  dexterity:
    value: 10
  save_dexterity:
    value: 5
  constitution:
    value: 22
  save_constitution:
    value: 6
  intelligence:
    value: 8
  save_intelligence:
    value: -1
  wisdom:
    value: 12
  save_wisdom:
    value: 6
  charisma:
    value: 12
  save_charisma:
    value: 1
  armor_class:
    value: 18
  max_hp:
    value: 200
  initiative:
    value: 10
  speed:
    value: 40
    unit: ft
  burrow_speed:
    value: 30
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
  mechanic: dnd2024:creatures/adult_white_dragon
  implementation: dnd2024:creature_adult_white_dragon
  parameters:
    features:
      ice_walk:
        economy: passive
        movement_checks_waived:
          surfaces:
          - ice
          modes:
          - walk
          - climb
        difficult_terrain_extra_cost_ignored:
        - ice
        - snow
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
          damage_type: cold
        reach_feet: 10
      cold_breath:
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
        dc: 19
        targets:
          kind: creature
          count: all
          area:
            shape: cone
            size_feet: 60
            origin: self
        failure:
        - op: damage
          average: 54
          amount:
            dice:
              count: 12
              sides: 8
            add: 0
          damage_type: cold
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
      freezing_burst:
        economy: legendary_action
        op: save
        ability: constitution
        dc: 14
        targets:
          kind: creature
          count: all
          area:
            shape: sphere
            size_feet: 30
            origin: chosen_point
            origin_visible_to: self
            origin_range_feet: 120
        failure:
        - op: damage
          average: 7
          amount:
            dice:
              count: 2
              sides: 6
            add: 0
          damage_type: cold
        - op: set_speed
          feet: 0
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
        ability: charisma
        save_dc: 14
        components_waived:
        - material
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
      stealth: 5
    defenses:
      immunities:
        damage_types:
        - cold
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
- id: dnd2024:creature_adult_white_dragon
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
  source_sha256: 8a4a94494efc0bb94f134365b089d79a8da8d124b73ea85e2360a8167520d69f
  source_path: builtin/srd-5.2.1/creature/adult_white_dragon.md
  pages:
  - 340
  engine_gaps: &id001
  - dnd2024:creature_adult_white_dragon
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_2.md
  specification_sha256: ae5cdf7b87b500bb0115136545f78fdb1e59e7f59e7e869336788dc4c41fbc42
splat_clauses:
- key: source_rules
  text: '# Adult White Dragon Huge Dragon (Chromatic), Chaotic Evil AC 18 Initiative +10 (20) HP 200 (16d12
    + 96) Speed 40 ft., Burrow 30 ft., Fly 80 ft., Swim 40 ft. MOD SAVE MOD SAVE MOD SAVE Str 22 +6 +6
    Dex 10 +0 +5 Con 22 +6 +6 Int 8 -1 -1 Wis 12 +1 +6 Cha 12 +1 +1 Skills Perception +11, Stealth +5
    Immunities Cold Senses Blindsight 60 ft., Darkvision 120 ft.; Passive Perception 21 Languages Common,
    Draconic CR 13 (XP 10,000, or 11,500 in lair; PB +5) ### Traits Ice Walk. The dragon can move across
    and climb icy surfaces without needing to make an ability check. Additionally, Difficult Terrain composed
    of ice or snow doesn’t cost it extra movement. Legendary Resistance (3/Day, or 4/Day in Lair). If
    the dragon fails a saving throw, it can choose to succeed instead. ### Actions Multiattack. The dragon
    makes three Rend attacks. Rend. Melee Attack Roll: +11, reach 10 ft. Hit: 13 (2d6 + 6) Slashing damage
    plus 4 (1d8) Cold damage. Cold Breath (Recharge 5–6). Constitution Saving Throw: DC 19, each creature
    in a 60-foot Cone. Failure: 54 (12d8) Cold damage. Success: Half damage. ### Legendary Actions Legendary
    Action Uses: 3 (4 in Lair). Immediately after another creature’s turn, the dragon can expend a use
    to take one of the following actions. The dragon regains all expended uses at the start of each of
    its turns. Freezing Burst. Constitution Saving Throw: DC 14, each creature in a 30-foot-radius Sphere
    centered on a point the dragon can see within 120 feet. Failure: 7 (2d6) Cold damage, and the target’s
    Speed is 0 until the end of the target’s next turn. Failure or Success: The dragon can’t take this
    action again until the start of its next turn. Frightful Presence. The dragon casts Fear, requiring
    no Material components and using Charisma as the spellcasting ability (spell save DC 14). The dragon
    can’t take this action again until the start of its next turn. Pounce. The dragon moves up to half
    its Speed, and it makes one Rend attack.'
  sha256: 5b1865011d750d6751b31ec4ded41c5d010307986b8c3e51f691c5b764a8dd9c
  status: contract_specified
splat_requires_mechanics: *id001
---

# Adult White Dragon

<!-- SRD page 340 -->

Huge Dragon (Chromatic), Chaotic Evil\
AC 18 Initiative +10 (20)\
HP 200 (16d12 + 96)\
Speed 40 ft., Burrow 30 ft., Fly 80 ft., Swim 40 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 22 +6 +6 Dex 10 +0 +5 Con 22 +6 +6\
Int 8 −1 −1 Wis 12 +1 +6 Cha 12 +1 +1\
Skills Perception +11, Stealth +5\
Immunities Cold\
Senses Blindsight 60 ft., Darkvision 120 ft.;\
Passive Perception 21\
Languages Common, Draconic\
CR 13 (XP 10,000, or 11,500 in lair; PB +5)\

### Traits

Ice Walk. The dragon can move across and climb icy\
surfaces without needing to make an ability check.\
Additionally, Difficult Terrain composed of ice or snow\
doesn’t cost it extra movement.\
Legendary Resistance (3/Day, or 4/Day in Lair). If\
the dragon fails a saving throw, it can choose to suc-\
ceed instead.\

### Actions

Multiattack. The dragon makes three Rend attacks.\
Rend. Melee Attack Roll: +11, reach 10 ft. Hit: 13 (2d6\
+ 6) Slashing damage plus 4 (1d8) Cold damage.\
Cold Breath (Recharge 5–6). Constitution Saving\
Throw: DC 19, each creature in a 60-foot Cone. Failure:\
54 (12d8) Cold damage. Success: Half damage.\

### Legendary Actions

Legendary Action Uses: 3 (4 in Lair). Immediately after\
another creature’s turn, the dragon can expend a use to\
take one of the following actions. The dragon regains all\
expended uses at the start of each of its turns.\
Freezing Burst. Constitution Saving Throw: DC 14,\
each creature in a 30-foot-radius Sphere centered on\
a point the dragon can see within 120 feet. Failure: 7\
(2d6) Cold damage, and the target’s Speed is 0 until\
the end of the target’s next turn. Failure or Success: The\
dragon can’t take this action again until the start of its\
next turn.\
Frightful Presence. The dragon casts Fear, requiring no\
Material components and using Charisma as the spell-\
casting ability (spell save DC 14). The dragon can’t take\
this action again until the start of its next turn.\
Pounce. The dragon moves up to half its Speed, and it\
makes one Rend attack.\

Original layout: [[../../srd-5.2.1/pages/340|page 340]].
