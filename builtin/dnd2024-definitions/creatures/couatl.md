---
splat_schema: 1
splat_id: d4c20ad9-5f3e-5c66-9df0-6af616fc1ec9
splat_kind: creature
splat_key: dnd2024:creatures/couatl
splat_name: Couatl
splat_origin:
  key: dnd2024:reference/creature/couatl
splat_sources:
- title: SRD 5.2.1
  locator: pp. 277, 278
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: medium
  type: celestial
  alignment: Lawful Good
  movement_text: 30 ft., Fly 90 ft.
  resistances_text: Bludgeoning, Piercing, Slashing
  immunities_text: Psychic, Radiant
  senses_text: Truesight 120 ft.; Passive Perception 15
  languages_text: All; telepathy 120 ft.
  cr_text: 4 (XP 1,100; PB +2)
  hit_points_text: 60 (8d8 + 24)
splat_stats:
  strength:
    value: 16
  save_strength:
    value: 3
  dexterity:
    value: 20
  save_dexterity:
    value: 5
  constitution:
    value: 17
  save_constitution:
    value: 5
  intelligence:
    value: 18
  save_intelligence:
    value: 4
  wisdom:
    value: 20
  save_wisdom:
    value: 7
  charisma:
    value: 18
  save_charisma:
    value: 4
  armor_class:
    value: 19
  max_hp:
    value: 60
  initiative:
    value: 5
  speed:
    value: 30
    unit: ft
  fly_speed:
    value: 90
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/couatl
  implementation: dnd2024:creature_couatl
  parameters:
    features:
      bite:
        economy: action
        op: attack
        attack_modes:
        - melee
        attack_bonus: 7
        targets:
          count: 1
        hit:
        - op: damage
          average: 11
          amount:
            dice:
              count: 1
              sides: 12
            add: 5
          damage_type: piercing
        - op: condition
          condition: poisoned
          ends:
            event: turn.end
            participant: self
            occurrence: next
        reach_feet: 5
      spellcasting:
        economy: action
        op: cast_spell_choice
        ability: wisdom
        spells:
        - spell:
            key: dnd2024:spells/detect_evil_and_good
          frequency: at_will
          cast_level: spell_minimum
        - spell:
            key: dnd2024:spells/detect_magic
          frequency: at_will
          cast_level: spell_minimum
        - spell:
            key: dnd2024:spells/detect_thoughts
          frequency: at_will
          cast_level: spell_minimum
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
            key: dnd2024:spells/create_food_and_water
          frequency: daily
          uses: 1
          resource_scope: per_spell
          recovery: long_rest
          cast_level: spell_minimum
        - spell:
            key: dnd2024:spells/dream
          frequency: daily
          uses: 1
          resource_scope: per_spell
          recovery: long_rest
          cast_level: spell_minimum
        - spell:
            key: dnd2024:spells/greater_restoration
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
            key: dnd2024:spells/sleep
          frequency: daily
          uses: 1
          resource_scope: per_spell
          recovery: long_rest
          cast_level: spell_minimum
        components_waived:
        - verbal
        - somatic
        - material
        save_dc: 15
      shielded_mind:
        economy: passive
        thought_reading_allowed: false
        incoming_telepathy_requires_self_permission: true
      divine_aid:
        economy: bonus_action
        resource:
          capacity: 2
          cost: 1
          recovery: long_rest
          scope: shared_choices
        op: cast_spell_choice
        choices:
        - spell:
            key: dnd2024:spells/bless
        - spell:
            key: dnd2024:spells/lesser_restoration
        - spell:
            key: dnd2024:spells/sanctuary
        ability: wisdom
        save_dc: 15
        components_waived:
        - verbal
        - somatic
        - material
      constrict:
        economy: action
        op: save
        ability: strength
        dc: 15
        targets:
          kind: creature
          count: 1
          range_feet: 5
          visible_to: self
          size_max: medium
        failure:
        - op: damage
          average: 8
          amount:
            dice:
              count: 1
              sides: 6
            add: 5
          damage_type: bludgeoning
        - op: condition
          condition: grappled
          escape_dc: 13
          source: self
        - op: condition
          condition: restrained
          ends:
            event: grapple.end
            source: self
        success: []
    defenses:
      resistances:
        damage_types:
        - bludgeoning
        - piercing
        - slashing
      immunities:
        damage_types:
        - psychic
        - radiant
    senses:
      passive_perception: 15
      special:
      - sense: truesight
        range_feet: 120
    communication:
      speaks: all
      understands: all
      telepathy:
        range_feet: 120
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
- id: dnd2024:creature_couatl
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
  - grapple.end
  - turn.end
  capabilities:
  - action_economy
  - creature_rules
  - dice
  - effects
  - grappling
  - resources
  - spellcasting
  - targeting
splat_conversion:
  status: specified
  category: creature
  source_sha256: e0c7015aa04b7cf64f5e1d8ca37abc25e582057b4bac62ca4b4fa89874f178e6
  source_path: builtin/srd-5.2.1/creature/couatl.md
  pages:
  - 277
  - 278
  engine_gaps: &id001
  - dnd2024:creature_couatl
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_2.md
  specification_sha256: de329743dffbd347d427a809c2f5c6185bc23a41d4fb1c267fc75866b3f03e2b
splat_clauses:
- key: source_rules
  text: '# Couatl Medium Celestial, Lawful Good AC 19 Initiative +5 (15) HP 60 (8d8 + 24) Speed 30 ft.,
    Fly 90 ft. MOD SAVE MOD SAVE MOD SAVE Str 16 +3 +3 Dex 20 +5 +5 Con 17 +3 +5 Int 18 +4 +4 Wis 20 +5
    +7 Cha 18 +4 +4 Resistances Bludgeoning, Piercing, Slashing Immunities Psychic, Radiant Senses Truesight
    120 ft.; Passive Perception 15 Languages All; telepathy 120 ft. CR 4 (XP 1,100; PB +2) ### Traits
    Shielded Mind. The couatl’s thoughts can’t be read by any means, and other creatures can communicate
    with it telepathically only if it allows them. ### Actions Bite. Melee Attack Roll: +7, reach 5 ft.
    Hit: 11 (1d12 + 5) Piercing damage, and the target has the Poisoned condition until the end of the
    couatl’s next turn. Constrict. Strength Saving Throw: DC 15, one Medium or smaller creature the couatl
    can see within 5 feet. Failure: 8 (1d6 + 5) Bludgeoning damage. The target has the Grappled condition
    (escape DC 13), and it has the Restrained condition until the grapple ends. Spellcasting. The couatl
    casts one of the following spells, requiring no spell components and using Wisdom as the spellcasting
    ability (spell save DC 15): At Will: Detect Evil and Good, Detect Magic, Detect Thoughts, Shapechange
    (Beast or Humanoid form only, no Temporary Hit Points gained from the spell, and no Concentration
    or Temporary Hit Points required to maintain the spell) 1/Day Each: Create Food and Water, Dream,
    Greater Restoration, Scrying, Sleep ### Bonus Actions Divine Aid (2/Day). The couatl casts Bless,
    Lesser Restoration, or Sanctuary, requiring no spell components and using the same spellcasting ability
    as Spellcasting.'
  sha256: c2bb9370fa999e78fa1647d5f4006f344edd6c675ea1546abcdf7b6578d08940
  status: contract_specified
splat_requires_mechanics: *id001
---

# Couatl

<!-- SRD page 277 -->

Medium Celestial, Lawful Good\
AC 19 Initiative +5 (15)\
HP 60 (8d8 + 24)\
Speed 30 ft., Fly 90 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 16 +3 +3 Dex 20 +5 +5 Con 17 +3 +5\
Int 18 +4 +4 Wis 20 +5 +7 Cha 18 +4 +4\
Resistances Bludgeoning, Piercing, Slashing\
Immunities Psychic, Radiant\
Senses Truesight 120 ft.; Passive Perception 15\
Languages All; telepathy 120 ft.\
CR 4 (XP 1,100; PB +2)\

### Traits

Shielded Mind. The couatl’s thoughts can’t be read by\
any means, and other creatures can communicate with\
it telepathically only if it allows them.\

### Actions

Bite. Melee Attack Roll: +7, reach 5 ft. Hit: 11 (1d12\
+ 5) Piercing damage, and the target has the Poisoned\
condition until the end of the couatl’s next turn.\
Constrict. Strength Saving Throw: DC 15, one Medium\
or smaller creature the couatl can see within 5 feet.\
Failure: 8 (1d6 + 5) Bludgeoning damage. The target has\
the Grappled condition (escape DC 13), and it has the\
Restrained condition until the grapple ends.\
Spellcasting. The couatl casts one of the following\
spells, requiring no spell components and using Wis-\
dom as the spellcasting ability (spell save DC 15):\
At Will: Detect Evil and Good, Detect Magic, Detect\
Thoughts, Shapechange (Beast or Humanoid form\
only, no Temporary Hit Points gained from the spell,\

<!-- SRD page 278 -->

and no Concentration or Temporary Hit Points re-\
quired to maintain the spell)\
1/Day Each: Create Food and Water, Dream, Greater\
Restoration, Scrying, Sleep\

### Bonus Actions

Divine Aid (2/Day). The couatl casts Bless, Lesser Res-\
toration, or Sanctuary, requiring no spell components\
and using the same spellcasting ability as Spellcasting.\

Original layout: [[../../srd-5.2.1/pages/277|page 277]], [[../../srd-5.2.1/pages/278|page 278]].
