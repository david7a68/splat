---
splat_schema: 1
splat_id: 1e20ad87-337a-5f20-bd89-a170439e60fc
splat_kind: creature
splat_key: dnd2024:creatures/giant_insect
splat_name: Giant Insect
splat_origin:
  key: dnd2024:reference/creature/giant_insect
splat_sources:
- title: SRD 5.2.1
  locator: pp. 136
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: large
  type: beast
  alignment: Unaligned
  movement_text: 40 ft., Climb 40 ft., Fly 40 ft. (Wasp only)
  senses_text: Darkvision 60 ft.; Passive Perception 12
  languages_text: Understands the languages you know
  cr_text: None (XP 0; PB equals your Proficiency Bonus)
  hit_points_text: 30 + 10 for each spell level above 4
splat_stats:
  strength:
    value: 17
  save_strength:
    value: 3
  dexterity:
    value: 13
  save_dexterity:
    value: 1
  constitution:
    value: 15
  save_constitution:
    value: 2
  intelligence:
    value: 4
  save_intelligence:
    value: -3
  wisdom:
    value: 14
  save_wisdom:
    value: 2
  charisma:
    value: 3
  save_charisma:
    value: -4
  armor_class:
    value: 11
  speed:
    value: 40
    unit: ft
  climb_speed:
    value: 40
    unit: ft
  fly_speed:
    value: 40
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/giant_insect
  implementation: dnd2024:creature_giant_insect
  parameters:
    features:
      spider_climb:
        economy: passive
        climb_ability_check_required: false
        surfaces:
        - difficult
        - ceiling
      multiattack:
        economy: action
        op: repeat_choice
        count:
          floor_divide:
          - input: spell_level
          - 2
        choices:
        - feature: poison_jab
        - feature: web_bolt
          requires_form:
          - spider
      poison_jab:
        economy: action
        op: attack
        attack_modes:
        - melee
        attack_bonus:
          input: summoner_spell_attack_modifier
        reach_feet: 10
        targets:
          count: 1
        hit:
        - op: damage
          damage_type: piercing
          amount:
            add:
            - dice:
                count: 1
                sides: 6
              add: 3
            - input: spell_level
        - op: damage
          damage_type: poison
          amount:
            add:
            - dice:
                count: 1
                sides: 4
              add: 0
      web_bolt:
        economy: action
        requires_form:
        - spider
        op: attack
        attack_modes:
        - ranged
        attack_bonus:
          input: summoner_spell_attack_modifier
        range_feet: 60
        targets:
          count: 1
        hit:
        - op: damage
          damage_type: bludgeoning
          amount:
            add:
            - dice:
                count: 1
                sides: 10
              add: 3
            - input: spell_level
        - op: set_speed
          feet: 0
          ends:
            event: turn.start
            participant: self
            occurrence: next
      venomous_spew:
        economy: bonus_action
        requires_form:
        - centipede
        op: save
        ability: constitution
        dc:
          input: summoner_spell_save_dc
        targets:
          kind: creature
          count: 1
          range_feet: 10
          visible_to: self
        failure:
        - op: condition
          condition: poisoned
          ends:
            event: turn.start
            participant: self
            occurrence: next
        success: []
    defenses: {}
    senses:
      passive_perception: 12
      special:
      - sense: darkvision
        range_feet: 60
    communication:
      speaks: []
      understands:
        languages_known_by: summoner
      can_speak: false
    proficiency_bonus:
      input: summoner_proficiency_bonus
    scaling_stats:
      ac:
        add:
        - 11
        - input: spell_level
      hit_points:
        add:
        - 30
        - multiply:
          - 10
          - subtract:
            - input: spell_level
            - 4
    form_choice:
      chooser: summoner
      options:
      - wasp
      - spider
      - centipede
      fly_speed_feet:
        wasp: 40
        otherwise: 0
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
- id: dnd2024:creature_giant_insect
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
    summoner:
      type: participant
      required: true
    spell_level:
      type: integer
      required: true
    summoner_spell_attack_modifier:
      type: integer
      required: true
    summoner_spell_save_dc:
      type: integer
      required: true
    summoner_spellcasting_ability_modifier:
      type: integer
      required: true
    summoner_proficiency_bonus:
      type: integer
      required: true
  outputs:
  - resolved_feature
  - effects
  - resource_changes
  events:
  - turn.start
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
  source_sha256: 44af60f6c2c349b80e27adb47dba51ec9ec1deef6b6a8ae462bd57a3078239ba
  source_path: builtin/srd-5.2.1/creature/giant_insect.md
  pages:
  - 136
  engine_gaps: &id001
  - dnd2024:creature_giant_insect
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_2.md
  specification_sha256: 08ef00b1c7a7f4d9eff2b9a17cb62c080345f76d1b0353336eed9c6c4ec1dbce
splat_clauses:
- key: source_rules
  text: '# Giant Insect Large Beast, Unaligned AC 11 + the spell’s level HP 30 + 10 for each spell level
    above 4 Speed 40 ft., Climb 40 ft., Fly 40 ft. (Wasp only) MOD SAVE MOD SAVE MOD SAVE Str 17 +3 +3
    Dex 13 +1 +1 Con15 +2 +2 Int 4 -3 -3 Wis 14 +2 +2 Cha 3 -4 -4 Senses Darkvision 60 ft.; Passive Perception
    12 Languages Understands the languages you know CR None (XP 0; PB equals your Proficiency Bonus) ###
    Traits Spider Climb. The insect can climb difficult surfaces, including along ceilings, without needing
    to make an ability check. ### Actions Multiattack. The insect makes a number of attacks equal to half
    this spell’s level (round down). Poison Jab. Melee Attack Roll: Bonus equals your spell attack modifier,
    reach 10 ft. Hit: 1d6 + 3 plus the spell’s level Piercing damage plus 1d4 Poison damage. Web Bolt
    (Spider Only). Ranged Attack Roll: Bonus equals your spell attack modifier, range 60 ft. Hit: 1d10
    + 3 plus the spell’s level Bludgeoning damage, and the target’s Speed is reduced to 0 until the start
    of the insect’s next turn. ### Bonus Actions Venomous Spew (Centipede Only). Constitution Saving Throw:
    Your spell save DC, one creature the insect can see within 10 feet. Failure: The target has the Poisoned
    condition until the start of the insect’s next turn.'
  sha256: 5b40e20cb349c1f619e7396496d6219b6dac428ab464b62cc4e5768d4ed54d4d
  status: contract_specified
splat_requires_mechanics: *id001
---

# Giant Insect

<!-- SRD page 136 -->

Large Beast, Unaligned\
AC 11 + the spell’s level\
HP 30 + 10 for each spell level above 4\
Speed 40 ft., Climb 40 ft., Fly 40 ft. (Wasp only)\
MOD SAVE MOD SAVE MOD SAVE\
Str 17 +3 +3 Dex 13 +1 +1 Con15 +2 +2\
Int 4 −3 −3 Wis 14 +2 +2 Cha 3 −4 −4\
Senses Darkvision 60 ft.; Passive Perception 12\
Languages Understands the languages you know\
CR None (XP 0; PB equals your Proficiency Bonus)\

### Traits

Spider Climb. The insect can climb difficult surfaces,\
including along ceilings, without needing to make an\
ability check.\

### Actions

Multiattack. The insect makes a number of attacks\
equal to half this spell’s level (round down).\
Poison Jab. Melee Attack Roll: Bonus equals your spell\
attack modifier, reach 10 ft. Hit: 1d6 + 3 plus the spell’s\
level Piercing damage plus 1d4 Poison damage.\
Web Bolt (Spider Only). Ranged Attack Roll: Bonus\
equals your spell attack modifier, range 60 ft. Hit: 1d10\
+ 3 plus the spell’s level Bludgeoning damage, and the\
target’s Speed is reduced to 0 until the start of the in-\
sect’s next turn.\

### Bonus Actions

Venomous Spew (Centipede Only). Constitution Saving\
Throw: Your spell save DC, one creature the insect can\
see within 10 feet. Failure: The target has the Poisoned\
condition until the start of the insect’s next turn.\

Original layout: [[../../srd-5.2.1/pages/136|page 136]].
