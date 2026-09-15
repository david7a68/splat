---
splat_schema: 1
splat_id: 4eb6a57f-e362-59ad-8474-2d2e0563379a
splat_kind: creature
splat_key: dnd2024:creatures/lamia
splat_name: Lamia
splat_origin:
  key: dnd2024:reference/creature/lamia
splat_sources:
- title: SRD 5.2.1
  locator: pp. 303, 304
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: large
  type: fiend
  alignment: Chaotic Evil
  movement_text: 40 ft.
  skills_text: Deception +7, Insight +4, Stealth +5
  senses_text: Darkvision 60 ft.; Passive Perception 12
  languages_text: Abyssal, Common
  cr_text: 4 (XP 1,100; PB +2)
  hit_points_text: 97 (13d10 + 26)
splat_stats:
  strength:
    value: 16
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
    value: 14
  save_intelligence:
    value: 2
  wisdom:
    value: 15
  save_wisdom:
    value: 2
  charisma:
    value: 16
  save_charisma:
    value: 3
  armor_class:
    value: 13
  max_hp:
    value: 97
  initiative:
    value: 1
  speed:
    value: 40
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/lamia
  implementation: dnd2024:creature_lamia
  parameters:
    features:
      claw:
        economy: action
        op: attack
        attack_modes:
        - melee
        attack_bonus: 5
        targets:
          count: 1
        hit:
        - op: damage
          average: 7
          amount:
            dice:
              count: 1
              sides: 8
            add: 3
          damage_type: slashing
        - op: damage
          average: 7
          amount:
            dice:
              count: 2
              sides: 6
            add: 0
          damage_type: psychic
        reach_feet: 5
      leap:
        economy: bonus_action
        op: jump
        distance_max_feet: 30
        movement_cost_feet: 10
      multiattack:
        economy: action
        op: sequence
        steps:
        - op: use_feature
          feature: claw
          count: 2
        replacement:
          max_attacks: 1
          choices:
          - feature: corrupting_touch
      spellcasting:
        economy: action
        op: cast_spell_choice
        ability: charisma
        spells:
        - spell:
            key: dnd2024:spells/disguise_self
          frequency: at_will
          overrides:
            appearance:
              sizes:
              - large
              - medium
              anatomy: biped
          cast_level: spell_minimum
        - spell:
            key: dnd2024:spells/minor_illusion
          frequency: at_will
          cast_level: spell_minimum
        - spell:
            key: dnd2024:spells/geas
          frequency: daily
          uses: 1
          resource_scope: per_spell
          recovery: long_rest
          cast_level: spell_minimum
        - spell:
            key: dnd2024:spells/major_image
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
        components_waived:
        - material
        save_dc: 13
      corrupting_touch:
        economy: action
        op: save
        ability: wisdom
        dc: 13
        targets:
          kind: creature
          count: 1
          range_feet: 5
          visible_to: self
        failure:
        - op: damage
          average: 13
          amount:
            dice:
              count: 3
              sides: 8
            add: 0
          damage_type: psychic
        - op: curse
          duration:
            hours: 1
          effects:
          - op: condition
            condition: charmed
            source: self
          - op: condition
            condition: poisoned
        success: []
    skill_totals:
      deception: 7
      insight: 4
      stealth: 5
    defenses: {}
    senses:
      passive_perception: 12
      special:
      - sense: darkvision
        range_feet: 60
    communication:
      speaks:
      - abyssal
      - common
      understands:
      - abyssal
      - common
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
- id: dnd2024:creature_lamia
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
  events: []
  capabilities:
  - action_economy
  - creature_rules
  - curses
  - dice
  - effects
  - resources
  - spellcasting
  - targeting
splat_conversion:
  status: specified
  category: creature
  source_sha256: a334dadf6aa036dfe5cea58ca48397f81be54adf294ffc4cfdb7bb86626676d6
  source_path: builtin/srd-5.2.1/creature/lamia.md
  pages:
  - 303
  - 304
  engine_gaps: &id001
  - dnd2024:creature_lamia
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_2.md
  specification_sha256: b07c9b6d33c148a484f00a5f7c770bf75d3efb5b762a9b27fab721614146412e
splat_clauses:
- key: source_rules
  text: '# Lamia Large Fiend, Chaotic Evil AC 13 Initiative +1 (11) HP 97 (13d10 + 26) Speed 40 ft. MOD
    SAVE MOD SAVE MOD SAVE Str 16 +3 +3 Dex 13 +1 +1 Con 15 +2 +2 Int 14 +2 +2 Wis 15 +2 +2 Cha 16 +3
    +3 Skills Deception +7, Insight +4, Stealth +5 Senses Darkvision 60 ft.; Passive Perception 12 Languages
    Abyssal, Common CR 4 (XP 1,100; PB +2) ### Actions Multiattack. The lamia makes two Claw attacks.
    It can replace one attack with a use of Corrupting Touch. Claw. Melee Attack Roll: +5, reach 5 ft.
    Hit: 7 (1d8 + 3) Slashing damage plus 7 (2d6) Psychic damage. Corrupting Touch. Wisdom Saving Throw:
    DC 13, one creature the lamia can see within 5 feet. Failure: 13 (3d8) Psychic damage, and the target
    is cursed for 1 hour. Until the curse ends, the target has the Charmed and Poisoned conditions. Spellcasting.
    The lamia casts one of the following spells, requiring no Material components and using Charisma as
    the spellcasting ability (spell save DC 13): At Will: Disguise Self (can appear as a Large or Medium
    biped), Minor Illusion 1/Day Each: Geas, Major Image, Scrying ### Bonus Actions Leap. The lamia jumps
    up to 30 feet by spending 10 feet of movement.'
  sha256: 8c04019e876ae826617c9a895c9c72e35be47a0d5d5f6f21303fa4c3822cdbb4
  status: contract_specified
splat_requires_mechanics: *id001
---

# Lamia

<!-- SRD page 303 -->

Large Fiend, Chaotic Evil\
AC 13 Initiative +1 (11)\
HP 97 (13d10 + 26)\
Speed 40 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 16 +3 +3 Dex 13 +1 +1 Con 15 +2 +2\
Int 14 +2 +2 Wis 15 +2 +2 Cha 16 +3 +3\
Skills Deception +7, Insight +4, Stealth +5\
Senses Darkvision 60 ft.; Passive Perception 12\
Languages Abyssal, Common\
CR 4 (XP 1,100; PB +2)\

### Actions

Multiattack. The lamia makes two Claw attacks. It can\
replace one attack with a use of Corrupting Touch.\
Claw. Melee Attack Roll: +5, reach 5 ft. Hit: 7 (1d8 + 3)\
Slashing damage plus 7 (2d6) Psychic damage.\
Corrupting Touch. Wisdom Saving Throw: DC 13, one\
creature the lamia can see within 5 feet. Failure: 13\
(3d8) Psychic damage, and the target is cursed for 1\
hour. Until the curse ends, the target has the Charmed\
and Poisoned conditions.\
Spellcasting. The lamia casts one of the following\
spells, requiring no Material components and using\
Charisma as the spellcasting ability (spell save DC 13):\
At Will: Disguise Self (can appear as a Large or Medium\
biped), Minor Illusion\
1/Day Each: Geas, Major Image, Scrying\

<!-- SRD page 304 -->


### Bonus Actions

Leap. The lamia jumps up to 30 feet by spending 10\
feet of movement.\

Original layout: [[../../srd-5.2.1/pages/303|page 303]], [[../../srd-5.2.1/pages/304|page 304]].
