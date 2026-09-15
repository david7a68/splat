---
splat_schema: 1
splat_id: ae140097-94e9-5b72-8a9b-0f19ce7ae08a
splat_kind: creature
splat_key: dnd2024:creatures/blink_dog
splat_name: Blink Dog
splat_origin:
  key: dnd2024:reference/creature/blink_dog
splat_sources:
- title: SRD 5.2.1
  locator: pp. 266
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: medium
  type: fey
  alignment: Lawful Good
  movement_text: 40 ft.
  skills_text: Perception +5, Stealth +5
  senses_text: Darkvision 60 ft.; Passive Perception 15
  languages_text: Blink Dog; understands Elvish and Sylvan but can’t speak them
  cr_text: 1/4 (XP 50; PB +2)
  hit_points_text: 22 (4d8 + 4)
splat_stats:
  strength:
    value: 12
  save_strength:
    value: 1
  dexterity:
    value: 17
  save_dexterity:
    value: 3
  constitution:
    value: 12
  save_constitution:
    value: 1
  intelligence:
    value: 10
  save_intelligence:
    value: 0
  wisdom:
    value: 13
  save_wisdom:
    value: 1
  charisma:
    value: 11
  save_charisma:
    value: 0
  armor_class:
    value: 13
  max_hp:
    value: 22
  initiative:
    value: 3
  speed:
    value: 40
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/blink_dog
  implementation: dnd2024:creature_blink_dog
  parameters:
    features:
      bite:
        economy: action
        op: attack
        attack_modes:
        - melee
        attack_bonus: 5
        targets:
          count: 1
        hit:
        - op: damage
          average: 5
          amount:
            dice:
              count: 1
              sides: 4
            add: 3
          damage_type: piercing
        reach_feet: 5
      teleport:
        economy: bonus_action
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
            - 4
            - 5
            - 6
          cost: 1
          recovery_also:
          - short_rest
          - long_rest
        op: teleport
        targets: self
        range_feet: 40
        destination:
          unoccupied: true
          visible_to: self
    skill_totals:
      perception: 5
      stealth: 5
    defenses: {}
    senses:
      passive_perception: 15
      special:
      - sense: darkvision
        range_feet: 60
    communication:
      speaks:
      - blink_dog
      understands:
      - blink_dog
      - elvish
      - sylvan
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
- id: dnd2024:creature_blink_dog
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
  - turn.start
  capabilities:
  - action_economy
  - creature_rules
  - dice
  - effects
  - movement
  - resources
  - targeting
splat_conversion:
  status: specified
  category: creature
  source_sha256: b5315f48bab0d2bebfae8e7db9d1e1f7f7c270d5ff04ab73887916368c37ea41
  source_path: builtin/srd-5.2.1/creature/blink_dog.md
  pages:
  - 266
  engine_gaps: &id001
  - dnd2024:creature_blink_dog
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_2.md
  specification_sha256: 8dd9199fc468c6cbf9baec95c54af72088bb49853682345ab3ea9f10054a513f
splat_clauses:
- key: source_rules
  text: '# Blink Dog Medium Fey, Lawful Good AC 13 Initiative +3 (13) HP 22 (4d8 + 4) Speed 40 ft. MOD
    SAVE MOD SAVE MOD SAVE Str 12 +1 +1 Dex 17 +3 +3 Con 12 +1 +1 Int 10 +0 +0 Wis 13 +1 +1 Cha 11 +0
    +0 Skills Perception +5, Stealth +5 Senses Darkvision 60 ft.; Passive Perception 15 Languages Blink
    Dog; understands Elvish and Sylvan but can’t speak them CR 1/4 (XP 50; PB +2) ### Actions Bite. Melee
    Attack Roll: +5, reach 5 ft. Hit: 5 (1d4 + 3) Piercing damage. ### Bonus Actions Teleport (Recharge
    4–6). The dog teleports up to 40 feet to an unoccupied space it can see.'
  sha256: 7689e771c5e42c99e81ad3a67ce3f89a2250ea548c4d8d1c0ad0c530d7812efc
  status: contract_specified
splat_requires_mechanics: *id001
---

# Blink Dog

<!-- SRD page 266 -->

Medium Fey, Lawful Good\
AC 13 Initiative +3 (13)\
HP 22 (4d8 + 4)\
Speed 40 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 12 +1 +1 Dex 17 +3 +3 Con 12 +1 +1\
Int 10 +0 +0 Wis 13 +1 +1 Cha 11 +0 +0\
Skills Perception +5, Stealth +5\
Senses Darkvision 60 ft.; Passive Perception 15\
Languages Blink Dog; understands Elvish and Sylvan but\
can’t speak them\
CR 1/4 (XP 50; PB +2)\

### Actions

Bite. Melee Attack Roll: +5, reach 5 ft. Hit: 5 (1d4 + 3)\
Piercing damage.\

### Bonus Actions

Teleport (Recharge 4–6). The dog teleports up to 40\
feet to an unoccupied space it can see.\

Original layout: [[../../srd-5.2.1/pages/266|page 266]].
