---
splat_schema: 1
splat_id: b4fa59d2-2ff7-5217-95b9-00d9abaa06a6
splat_kind: creature
splat_key: dnd2024:creatures/animated_object
splat_name: Animated Object
splat_origin:
  key: dnd2024:reference/creature/animated_object
splat_sources:
- title: SRD 5.2.1
  locator: pp. 109
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: huge
  type: construct
  alignment: Unaligned
  size_options_text: Huge or Smaller
  movement_text: 30 ft.
  immunities_text: Poison, Psychic; Charmed, Exhaustion, Frightened, Paralyzed, Poisoned
  senses_text: Blindsight 30 ft.; Passive Perception 6
  languages_text: Understands the languages you know
  cr_text: None (XP 0; PB equals your Proficiency Bonus)
  hit_points_text: 10 (Medium or smaller), 20 (Large), 40 (Huge)
splat_stats:
  strength:
    value: 16
  save_strength:
    value: 3
  dexterity:
    value: 10
  save_dexterity:
    value: 0
  constitution:
    value: 10
  save_constitution:
    value: 0
  intelligence:
    value: 3
  save_intelligence:
    value: -4
  wisdom:
    value: 3
  save_wisdom:
    value: -4
  charisma:
    value: 1
  save_charisma:
    value: -5
  armor_class:
    value: 15
  speed:
    value: 30
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/animated_object
  implementation: dnd2024:creature_animated_object
  parameters:
    features:
      slam:
        economy: action
        op: attack
        attack_modes:
        - melee
        attack_bonus:
          input: summoner_spell_attack_modifier
        reach_feet: 5
        targets:
          count: 1
        hit:
        - op: switch
          value:
            stat: size
          cases:
          - values:
            - tiny
            - small
            - medium
            effects:
            - op: damage
              damage_type: force
              amount:
                add:
                - dice:
                    count: 1
                    sides: 4
                  add: 3
          - values:
            - large
            effects:
            - op: damage
              damage_type: force
              amount:
                add:
                - dice:
                    count: 2
                    sides: 6
                  add: 3
                - input: summoner_spellcasting_ability_modifier
          - values:
            - huge
            effects:
            - op: damage
              damage_type: force
              amount:
                add:
                - dice:
                    count: 2
                    sides: 12
                  add: 3
                - input: summoner_spellcasting_ability_modifier
    defenses:
      immunities:
        damage_types:
        - poison
        - psychic
        conditions:
        - charmed
        - exhaustion
        - frightened
        - paralyzed
        - poisoned
    senses:
      passive_perception: 6
      special:
      - sense: blindsight
        range_feet: 30
    communication:
      speaks: []
      understands:
        languages_known_by: summoner
      can_speak: false
    proficiency_bonus:
      input: summoner_proficiency_bonus
    variant_stats:
      size_choice:
      - tiny
      - small
      - medium
      - large
      - huge
      hit_points_by_size:
        tiny: 10
        small: 10
        medium: 10
        large: 20
        huge: 40
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
- id: dnd2024:creature_animated_object
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
  source_sha256: 5838502de906c1a93f8a6aecc5ef025eb58de7c377eb20da6a345bd77004ffc4
  source_path: builtin/srd-5.2.1/creature/animated_object.md
  pages:
  - 109
  engine_gaps: &id001
  - dnd2024:creature_animated_object
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_2.md
  specification_sha256: 852e06dd216661c43d344bcf4d9269ed7d869e756bf43d60c34e7a14cef9a4e0
splat_clauses:
- key: source_rules
  text: '# Animated Object Huge or Smaller Construct, Unaligned AC 15 HP 10 (Medium or smaller), 20 (Large),
    40 (Huge) Speed 30 ft. MOD SAVE MOD SAVE MOD SAVE Str 16 +3 +3 Dex 10 +0 +0 Con10 +0 +0 Int 3 -4 -4
    Wis 3 -4 -4 Cha 1 -5 -5 Immunities Poison, Psychic; Charmed, Exhaustion, Frightened, Paralyzed, Poisoned
    Senses Blindsight 30 ft.; Passive Perception 6 Languages Understands the languages you know CR None
    (XP 0; PB equals your Proficiency Bonus) ### Actions Slam. Melee Attack Roll: Bonus equals your spell
    attack modifier, reach 5 ft. Hit: Force damage equal to 1d4 + 3 (Medium or smaller), 2d6 + 3 + your
    spellcasting ability modifier (Large), or 2d12 + 3 + your spellcasting ability modifier (Huge).'
  sha256: 1a499114861faca89cd8122d54b59d604e67e22513dc78143b5d884337cc2318
  status: contract_specified
splat_requires_mechanics: *id001
---

# Animated Object

<!-- SRD page 109 -->

Huge or Smaller Construct, Unaligned\
AC 15\
HP 10 (Medium or smaller), 20 (Large), 40 (Huge)\
Speed 30 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 16 +3 +3 Dex 10 +0 +0 Con10 +0 +0\
Int 3 −4 −4 Wis 3 −4 −4 Cha 1 −5 −5\
Immunities Poison, Psychic; Charmed, Exhaustion,\
Frightened, Paralyzed, Poisoned\
Senses Blindsight 30 ft.; Passive Perception 6\
Languages Understands the languages you know\
CR None (XP 0; PB equals your Proficiency Bonus)\

### Actions

Slam. Melee Attack Roll: Bonus equals your spell attack\
modifier, reach 5 ft. Hit: Force damage equal to 1d4\
+ 3 (Medium or smaller), 2d6 + 3 + your spellcasting\
ability modifier (Large), or 2d12 + 3 + your spellcasting\
ability modifier (Huge).\

Original layout: [[../../srd-5.2.1/pages/109|page 109]].
