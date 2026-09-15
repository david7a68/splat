---
splat_schema: 1
splat_id: e5d39c22-fefe-5740-9dd2-904b59632f06
splat_kind: creature
splat_key: dnd2024:creatures/incubus
splat_name: Incubus
splat_origin:
  key: dnd2024:reference/creature/incubus
splat_sources:
- title: SRD 5.2.1
  locator: pp. 301
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: medium
  type: fiend
  alignment: Neutral Evil
  movement_text: 30 ft., Fly 60 ft.
  skills_text: Deception +9, Insight +5, Perception +5, Persuasion +9, Stealth +7
  resistances_text: Cold, Fire, Poison, Psychic
  senses_text: Darkvision 60 ft.; Passive Perception 15
  languages_text: Abyssal, Common, Infernal; telepathy 60 ft.
  cr_text: 4 (XP 1,100; PB +2)
  hit_points_text: 66 (12d8 + 12)
splat_stats:
  strength:
    value: 8
  save_strength:
    value: -1
  dexterity:
    value: 17
  save_dexterity:
    value: 3
  constitution:
    value: 13
  save_constitution:
    value: 1
  intelligence:
    value: 15
  save_intelligence:
    value: 2
  wisdom:
    value: 12
  save_wisdom:
    value: 1
  charisma:
    value: 20
  save_charisma:
    value: 5
  armor_class:
    value: 15
  max_hp:
    value: 66
  initiative:
    value: 3
  speed:
    value: 30
    unit: ft
  fly_speed:
    value: 60
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/incubus
  implementation: dnd2024:creature_incubus
  parameters:
    skill_totals:
      deception: 9
      insight: 5
      perception: 5
      persuasion: 9
      stealth: 7
    damage_resistances:
    - cold
    - fire
    - poison
    - psychic
    senses:
    - sense: darkvision
      range_ft: 60
    passive_perception: 15
    languages:
      speak: true
      known:
      - common
      - infernal
      - abyssal
    telepathy:
      range_ft: 60
      two_way: true
    features:
      restless_touch:
        economy: action
        op: attack
        modes:
        - melee
        attack_bonus: 7
        target:
          kind: creature_or_object
          count: 1
        reach_ft: 5
        hit:
        - op: damage
          components:
          - type: psychic
            amount:
              dice:
                count: 3
                sides: 6
              add: 5
            average: 15
        - op: curse.apply
          duration:
            maximum:
              amount: 24
              unit: hour
            until: actor.died
          rules:
            short_rest_benefits: false
      spellcasting:
        economy: action
        op: spellcasting
        profile:
          ability: charisma
          waived_components:
          - material
          spells:
          - spell:
              key: dnd2024:spells/disguise_self
            uses:
              unlimited: true
          - spell:
              key: dnd2024:spells/etherealness
            uses:
              unlimited: true
          - spell:
              key: dnd2024:spells/dream
            uses:
              count: 1
              recovery: long_rest
          - spell:
              key: dnd2024:spells/hypnotic_pattern
            uses:
              count: 1
              recovery: long_rest
          save_dc: 15
      succubus_form:
        economy: passive
        trigger: actor.long_rest_finished
        optional: true
        op: stat_block.replace
        definition:
          key: dnd2024:creatures/succubus
        equipment_transformed: false
      nightmare:
        economy: bonus_action
        op: saving_throw
        ability: wisdom
        dc: 15
        targets:
          kind: creature
          count: 1
          range_ft: 60
          visible_to_actor: true
        failure:
        - op: branch
          if:
            target_hp_at_most: 20
          then:
          - op: condition.apply
            condition: unconscious
            duration:
              maximum:
                amount: 1
                unit: hour
              until_any:
              - target.takes_damage
              - creature_within_5_ft.uses_action_to_wake
          else:
          - op: damage
            components:
            - type: psychic
              amount:
                dice:
                  count: 4
                  sides: 8
                add: 0
              average: 18
        success: []
        recharge:
          die_sides: 6
          success_min: 6
          success_max: 6
          check_event: actor.turn_start
          maximum_uses: 1
          initial_uses: 1
          also_recovers_after:
          - short_rest
          - long_rest
      multiattack:
        economy: action
        op: multiattack
        sequence:
        - count: 2
          feature: restless_touch
    damage_resolution:
      choice:
      - printed_average
      - roll_expression
      chooser: controller
      exclusive: true
    spellcasting_defaults:
      cast_level: lowest_possible_unless_overridden
      upcast_allowed: only_explicit_cast_level_overrides
      required_material_components_available: true
      casting_time_at_least_1_minute:
        uses_normal_time: true
        magic_action_each_turn: true
        concentration_required: true
      other_components: spell_definition_unless_waived
    spatial_resolution:
      provider: caller
      required_facts:
      - distance
      - area_membership
      - visibility
      - occupancy
      - movement_path
      validate_against_typed_constraints: true
splat_contracts:
- id: dnd2024:creature_incubus
  inputs:
    actor:
      type: participant
      required: true
    targets:
      required: false
      type: participant_list
    objects:
      required: false
      type: object_list
    choices:
      required: false
      type: choice
    scene:
      required: false
      type: resolution
    event:
      required: false
      type: event
  outputs:
  - effects
  - roll_requests
  - state_changes
  events:
  - actor.long_rest_finished
  - actor.turn_start
  - creature.feature_requested
  capabilities:
  - action_economy
  - attack_resolution
  - creature_lifecycle
  - creature_rules
  - damage
  - effects
  - resources
  - rest_recovery
  - saving_throws
  - spatial_constraints
  - spellcasting
splat_conversion:
  status: specified
  category: creature
  source_sha256: 9361bd5d1585c63db62aa3fe930c0a17ba63aff90a134a8ef7b0f3fb19317465
  source_path: builtin/srd-5.2.1/creature/incubus.md
  pages:
  - 301
  engine_gaps: &id001
  - dnd2024:creature_incubus
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_3.md
  specification_sha256: 26a843313e7de7a767247a0f2057923cca183fd744c81ad5901030fa7f0f4baa
splat_clauses:
- key: source_rules
  text: '# Incubus Medium Fiend, Neutral Evil AC 15 Initiative +3 (13) HP 66 (12d8 + 12) Speed 30 ft.,
    Fly 60 ft. MOD SAVE MOD SAVE MOD SAVE Str 8 -1 -1 Dex 17 +3 +3 Con 13 +1 +1 Int 15 +2 +2 Wis 12 +1
    +1 Cha 20 +5 +5 Skills Deception +9, Insight +5, Perception +5, Persuasion +9, Stealth +7 Resistances
    Cold, Fire, Poison, Psychic Senses Darkvision 60 ft.; Passive Perception 15 Languages Abyssal, Common,
    Infernal; telepathy 60 ft. CR 4 (XP 1,100; PB +2) ### Traits Succubus Form. When the incubus finishes
    a Long Rest, it can shape-shift into a Succubus, using that stat block instead of this one. Any equipment
    it is wearing or carrying isn’t transformed. ### Actions Multiattack. The incubus makes two Restless
    Touch attacks. Restless Touch. Melee Attack Roll: +7, reach 5 ft. Hit: 15 (3d6 + 5) Psychic damage,
    and the target is cursed for 24 hours or until the incubus dies. Until the curse ends, the target
    gains no benefit from finishing Short Rests. Spellcasting. The incubus casts one of the following
    spells, requiring no Material components and using Charisma as the spellcasting ability (spell save
    DC 15): At Will: Disguise Self, Etherealness 1/Day Each: Dream, Hypnotic Pattern ### Bonus Actions
    Nightmare (Recharge 6). Wisdom Saving Throw: DC 15, one creature the incubus can see within 60 feet.
    Failure: If the target has 20 Hit Points or fewer, it has the Unconscious condition for 1 hour, until
    it takes damage, or until a creature within 5 feet of it takes an action to wake it. Otherwise, the
    target takes 18 (4d8) Psychic damage.'
  sha256: cb1bdcce040ab928059579be3341c151af0baa3e62d44a2e4bd7335c3925a1e8
  status: contract_specified
splat_requires_mechanics: *id001
---

# Incubus

<!-- SRD page 301 -->

Medium Fiend, Neutral Evil\
AC 15  Initiative +3 (13)\
HP 66 (12d8 + 12)\
Speed 30 ft., Fly 60 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 8 −1 −1 Dex 17 +3 +3 Con 13 +1 +1\
Int 15 +2 +2 Wis 12 +1 +1 Cha 20 +5 +5\
Skills Deception +9, Insight +5, Perception +5, Persuasion\
+9, Stealth +7\
Resistances Cold, Fire, Poison, Psychic\
Senses Darkvision 60 ft.; Passive Perception 15\
Languages Abyssal, Common, Infernal; telepathy 60 ft.\
CR 4 (XP 1,100; PB +2)\

### Traits

Succubus Form. When the incubus finishes a Long\
Rest, it can shape-shift into a Succubus, using that stat\
block instead of this one. Any equipment it is wearing\
or carrying isn’t transformed.\

### Actions

Multiattack. The incubus makes two Restless\
Touch attacks.\
Restless Touch. Melee Attack Roll: +7, reach 5 ft.\
Hit: 15 (3d6 + 5) Psychic damage, and the target is\
cursed for 24 hours or until the incubus dies. Until the\
curse ends, the target gains no benefit from finishing\
Short Rests.\
Spellcasting. The incubus casts one of the following\
spells, requiring no Material components and using\
Charisma as the spellcasting ability (spell save DC 15):\
At Will: Disguise Self, Etherealness\
1/Day Each: Dream, Hypnotic Pattern\

### Bonus Actions

Nightmare (Recharge 6). Wisdom Saving Throw: DC\
15, one creature the incubus can see within 60 feet.\
Failure: If the target has 20 Hit Points or fewer, it has\
the Unconscious condition for 1 hour, until it takes\
damage, or until a creature within 5 feet of it takes an\
action to wake it. Otherwise, the target takes 18 (4d8)\
Psychic damage.\

Original layout: [[../../srd-5.2.1/pages/301|page 301]].
