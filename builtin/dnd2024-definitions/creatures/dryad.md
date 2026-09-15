---
splat_schema: 1
splat_id: 77df49d6-e151-5948-9bab-834533875dea
splat_kind: creature
splat_key: dnd2024:creatures/dryad
splat_name: Dryad
splat_origin:
  key: dnd2024:reference/creature/dryad
splat_sources:
- title: SRD 5.2.1
  locator: pp. 282
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: medium
  type: fey
  alignment: Neutral
  movement_text: 30 ft.
  skills_text: Perception +4, Stealth +5
  senses_text: Darkvision 60 ft.; Passive Perception 14
  languages_text: Elvish, Sylvan
  cr_text: 1 (XP 200; PB +2)
  hit_points_text: 22 (5d8)
splat_stats:
  strength:
    value: 10
  save_strength:
    value: 0
  dexterity:
    value: 12
  save_dexterity:
    value: 1
  constitution:
    value: 11
  save_constitution:
    value: 0
  intelligence:
    value: 14
  save_intelligence:
    value: 2
  wisdom:
    value: 15
  save_wisdom:
    value: 2
  charisma:
    value: 18
  save_charisma:
    value: 4
  armor_class:
    value: 16
  max_hp:
    value: 22
  initiative:
    value: 1
  speed:
    value: 30
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/dryad
  implementation: dnd2024:creature_dryad
  parameters:
    features:
      magic_resistance:
        economy: passive
        advantage:
          roll: saving_throw
          against:
          - spell
          - magical_effect
      vine_lash:
        economy: action
        op: attack
        attack_modes:
        - melee
        attack_bonus: 6
        targets:
          count: 1
        hit:
        - op: damage
          average: 8
          amount:
            dice:
              count: 1
              sides: 8
            add: 4
          damage_type: slashing
        reach_feet: 10
      thorn_burst:
        economy: action
        op: attack
        attack_modes:
        - ranged
        attack_bonus: 6
        targets:
          count: 1
        hit:
        - op: damage
          average: 7
          amount:
            dice:
              count: 1
              sides: 6
            add: 4
          damage_type: piercing
        range_feet: 60
      multiattack:
        economy: action
        op: sequence
        steps:
        - op: choose
          choices:
          - op: use_feature
            feature: vine_lash
            count: 1
          - op: use_feature
            feature: thorn_burst
            count: 1
        - op: optional
          effect:
            op: cast_spell
            spell:
              key: dnd2024:spells/charm_monster
            via: spellcasting
            cast_level: spell_minimum
      spellcasting:
        economy: action
        op: cast_spell_choice
        ability: charisma
        spells:
        - spell:
            key: dnd2024:spells/animal_friendship
          frequency: at_will
          cast_level: spell_minimum
        - spell:
            key: dnd2024:spells/charm_monster
          frequency: at_will
          overrides:
            duration:
              hours: 24
            ends_on: self_casts_same_spell_again
          cast_level: spell_minimum
        - spell:
            key: dnd2024:spells/druidcraft
          frequency: at_will
          cast_level: spell_minimum
        - spell:
            key: dnd2024:spells/entangle
          frequency: daily
          uses: 1
          resource_scope: per_spell
          recovery: long_rest
          cast_level: spell_minimum
        - spell:
            key: dnd2024:spells/pass_without_trace
          frequency: daily
          uses: 1
          resource_scope: per_spell
          recovery: long_rest
          cast_level: spell_minimum
        components_waived:
        - material
        save_dc: 14
      speak_with_beasts_and_plants:
        economy: passive
        communication:
          as_shared_language: true
          creature_types:
          - beast
          - plant
      tree_stride:
        economy: bonus_action
        op: teleport
        requires:
          tree_1_size_min: large
          tree_1_distance_max_feet: 5
        destination:
          unoccupied: true
          tree_2_size_min: large
          tree_2_distance_max_feet: 5
          tree_2_different_from_tree_1: true
          tree_2_distance_from_tree_1_max_feet: 60
    skill_totals:
      perception: 4
      stealth: 5
    defenses: {}
    senses:
      passive_perception: 14
      special:
      - sense: darkvision
        range_feet: 60
    communication:
      speaks:
      - elvish
      - sylvan
      understands:
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
- id: dnd2024:creature_dryad
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
  - dice
  - effects
  - movement
  - resources
  - spellcasting
  - targeting
splat_conversion:
  status: specified
  category: creature
  source_sha256: 89ba0f4988cafe913d43b291e8ad5e9c68eeaa8d82185c167271d91ab0d608aa
  source_path: builtin/srd-5.2.1/creature/dryad.md
  pages:
  - 282
  engine_gaps: &id001
  - dnd2024:creature_dryad
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_2.md
  specification_sha256: b9d57e5d06a21e556b91bbdb32972bc1f7e4ab714cb104eca3093f670487330e
splat_clauses:
- key: source_rules
  text: '# Dryad Medium Fey, Neutral AC 16 Initiative +1 (11) HP 22 (5d8) Speed 30 ft. MOD SAVE MOD SAVE
    MOD SAVE Str 10 +0 +0 Dex 12 +1 +1 Con 11 +0 +0 Int 14 +2 +2 Wis 15 +2 +2 Cha 18 +4 +4 Skills Perception
    +4, Stealth +5 Senses Darkvision 60 ft.; Passive Perception 14 Languages Elvish, Sylvan CR 1 (XP 200;
    PB +2) ### Traits Magic Resistance. The dryad has Advantage on saving throws against spells and other
    magical effects. Speak with Beasts and Plants. The dryad can communicate with Beasts and Plants as
    if they shared a language. ### Actions Multiattack. The dryad makes one Vine Lash or Thorn Burst attack,
    and it can use Spellcasting to cast Charm Monster. Vine Lash. Melee Attack Roll: +6, reach 10 ft.
    Hit: 8 (1d8 + 4) Slashing damage. Thorn Burst. Ranged Attack Roll: +6, range 60 ft. Hit: 7 (1d6 +
    4) Piercing damage. Spellcasting. The dryad casts one of the following spells, requiring no Material
    components and using Charisma as the spellcasting ability (spell save DC 14): At Will: Animal Friendship,
    Charm Monster (lasts 24 hours; ends early if the dryad casts the spell again), Druidcraft 1/Day Each:
    Entangle, Pass without Trace ### Bonus Actions Tree Stride. If within 5 feet of a Large or bigger
    tree, the dryad teleports to an unoccupied space within 5 feet of a second Large or bigger tree that
    is within 60 feet of the previous tree.'
  sha256: f8b3eba7f0af67091eaa2fb17fdf33f47c04873fa189ef5d3fecd201a57a0b83
  status: contract_specified
splat_requires_mechanics: *id001
---

# Dryad

<!-- SRD page 282 -->

Medium Fey, Neutral\
AC 16 Initiative +1 (11)\
HP 22 (5d8)\
Speed 30 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 10 +0 +0 Dex 12 +1 +1 Con 11 +0 +0\
Int 14 +2 +2 Wis 15 +2 +2 Cha 18 +4 +4\
Skills Perception +4, Stealth +5\
Senses Darkvision 60 ft.; Passive Perception 14\
Languages Elvish, Sylvan\
CR 1 (XP 200; PB +2)\

### Traits

Magic Resistance. The dryad has Advantage on saving\
throws against spells and other magical effects.\
Speak with Beasts and Plants. The dryad can com-\
municate with Beasts and Plants as if they shared\
a language.\

### Actions

Multiattack. The dryad makes one Vine Lash or\
Thorn Burst attack, and it can use Spellcasting to cast\
Charm Monster.\
Vine Lash. Melee Attack Roll: +6, reach 10 ft. Hit: 8\
(1d8 + 4) Slashing damage.\
Thorn Burst. Ranged Attack Roll: +6, range 60 ft. Hit: 7\
(1d6 + 4) Piercing damage.\
Spellcasting. The dryad casts one of the following\
spells, requiring no Material components and using\
Charisma as the spellcasting ability (spell save DC 14):\
At Will: Animal Friendship, Charm Monster (lasts 24\
hours; ends early if the dryad casts the spell again),\
Druidcraft\
1/Day Each: Entangle, Pass without Trace\

### Bonus Actions

Tree Stride. If within 5 feet of a Large or bigger tree, the\
dryad teleports to an unoccupied space within 5 feet\
of a second Large or bigger tree that is within 60 feet of\
the previous tree.\

Original layout: [[../../srd-5.2.1/pages/282|page 282]].
