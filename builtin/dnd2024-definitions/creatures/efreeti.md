---
splat_schema: 1
splat_id: 735cd1e4-7c04-5899-8994-59dcb0679174
splat_kind: creature
splat_key: dnd2024:creatures/efreeti
splat_name: Efreeti
splat_origin:
  key: dnd2024:reference/creature/efreeti
splat_sources:
- title: SRD 5.2.1
  locator: pp. 283
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: large
  type: elemental
  alignment: Neutral
  tags_text: (Genie)
  movement_text: 40 ft., Fly 60 ft. (hover)
  immunities_text: Fire
  senses_text: Darkvision 120 ft.; Passive Perception 12
  languages_text: Primordial (Ignan)
  cr_text: 11 (XP 7,200; PB +4)
  hit_points_text: 212 (17d10 + 119)
splat_stats:
  strength:
    value: 22
  save_strength:
    value: 6
  dexterity:
    value: 12
  save_dexterity:
    value: 1
  constitution:
    value: 24
  save_constitution:
    value: 7
  intelligence:
    value: 16
  save_intelligence:
    value: 3
  wisdom:
    value: 15
  save_wisdom:
    value: 6
  charisma:
    value: 19
  save_charisma:
    value: 8
  armor_class:
    value: 17
  max_hp:
    value: 212
  initiative:
    value: 1
  speed:
    value: 40
    unit: ft
  fly_speed:
    value: 60
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/efreeti
  implementation: dnd2024:creature_efreeti
  parameters:
    damage_immunities:
    - fire
    condition_immunities: []
    senses:
    - sense: darkvision
      range_ft: 120
    passive_perception: 12
    languages:
      speak: true
      known:
      - primordial
      primordial_dialects:
      - ignan
    features:
      elemental_restoration:
        economy: passive
        trigger: actor.died
        when:
          plane_not: elemental_plane_of_fire
        body: disappears
        revive:
          new_body: true
          plane: elemental_plane_of_fire
          location_choice: gm
          hp: maximum
          delay:
            amount:
              dice:
                count: 1
                sides: 4
              add: 0
            unit: day
      magic_resistance:
        economy: passive
        op: roll.advantage
        rolls:
        - saving_throw
        against:
        - spell
        - magical_effect
      wishes:
        economy: passive
        known_chance_percent: 30
        spell:
          key: dnd2024:spells/wish
        requester:
          excluded_subtype: genie
          wish_communicated_and_understood: true
        caster_stress: false
        uses: 3
        after_third_cast_lockout:
          amount: 365
          unit: day
        reset_uses_after_lockout: true
      heated_blade:
        economy: action
        op: attack
        modes:
        - melee
        attack_bonus: 10
        target:
          kind: creature_or_object
          count: 1
        reach_ft: 5
        hit:
        - op: damage
          components:
          - type: slashing
            amount:
              dice:
                count: 2
                sides: 6
              add: 6
            average: 13
          - type: fire
            amount:
              dice:
                count: 2
                sides: 12
              add: 0
            average: 13
      hurl_flame:
        economy: action
        op: attack
        modes:
        - ranged
        attack_bonus: 8
        target:
          kind: creature_or_object
          count: 1
        range_ft:
          normal: 120
          long: 120
        hit:
        - op: damage
          components:
          - type: fire
            amount:
              dice:
                count: 7
                sides: 6
              add: 0
            average: 24
      spellcasting:
        economy: action
        op: spellcasting
        profile:
          ability: charisma
          waived_components:
          - material
          spells:
          - spell:
              key: dnd2024:spells/detect_magic
            uses:
              unlimited: true
          - spell:
              key: dnd2024:spells/elementalism
            uses:
              unlimited: true
          - spell:
              key: dnd2024:spells/gaseous_form
            uses:
              count: 1
              recovery: long_rest
          - spell:
              key: dnd2024:spells/invisibility
            uses:
              count: 1
              recovery: long_rest
          - spell:
              key: dnd2024:spells/major_image
            uses:
              count: 1
              recovery: long_rest
          - spell:
              key: dnd2024:spells/plane_shift
            uses:
              count: 1
              recovery: long_rest
          - spell:
              key: dnd2024:spells/tongues
            uses:
              count: 1
              recovery: long_rest
          - spell:
              key: dnd2024:spells/wall_of_fire
            uses:
              count: 1
              recovery: long_rest
            cast_level: 7
          save_dc: 16
      multiattack:
        economy: action
        op: multiattack
        attacks:
          count: 3
          choices:
          - heated_blade
          - hurl_flame
          combination: any
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
- id: dnd2024:creature_efreeti
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
  - actor.died
  - creature.feature_requested
  capabilities:
  - action_economy
  - attack_resolution
  - creature_rules
  - damage
  - effects
  - resources
  - rest_recovery
  - spatial_constraints
  - spellcasting
splat_conversion:
  status: specified
  category: creature
  source_sha256: eb04223f597e6cce770d4b87bd1befc63b1847bc56faa43b746c3ec347163d29
  source_path: builtin/srd-5.2.1/creature/efreeti.md
  pages:
  - 283
  engine_gaps: &id001
  - dnd2024:creature_efreeti
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_3.md
  specification_sha256: e67750d0ce40f8daf821774de6df2439e657ecae2570bc2d5da1c120ef64b658
splat_clauses:
- key: source_rules
  text: '# Efreeti Large Elemental (Genie), Neutral AC 17 Initiative +1 (11) HP 212 (17d10 + 119) Speed
    40 ft., Fly 60 ft. (hover) MOD SAVE MOD SAVE MOD SAVE Str 22 +6 +6 Dex 12 +1 +1 Con 24 +7 +7 Int 16
    +3 +3 Wis 15 +2 +6 Cha 19 +4 +8 Immunities Fire Senses Darkvision 120 ft.; Passive Perception 12 Languages
    Primordial (Ignan) CR 11 (XP 7,200; PB +4) ### Traits Elemental Restoration. If the efreeti dies outside
    the Elemental Plane of Fire, its body dissolves into ash, and it gains a new body in 1d4 days, reviving
    with all its Hit Points somewhere on the Plane of Fire. Magic Resistance. The efreeti has Advantage
    on saving throws against spells and other magical effects. Wishes. The efreeti has a 30 percent chance
    of knowing the Wish spell. If the efreeti knows it, the efreeti can cast it only on behalf of a non-genie
    creature who communicates a wish in a way the efreeti can understand. If the efreeti casts the spell
    for the creature, the efreeti suffers none of the spell’s stress. Once the efreeti has cast it three
    times, the efreeti can’t do so again for 365 days. ### Actions Multiattack. The efreeti makes three
    attacks, using Heated Blade or Hurl Flame in any combination. Heated Blade. Melee Attack Roll: +10,
    reach 5 ft. Hit: 13 (2d6 + 6) Slashing damage plus 13 (2d12) Fire damage. Hurl Flame. Ranged Attack
    Roll: +8, range 120 ft. Hit: 24 (7d6) Fire damage. Spellcasting. The efreeti casts one of the following
    spells, requiring no Material components and using Charisma as the spellcasting ability (spell save
    DC 16): At Will: Detect Magic, Elementalism 1/Day Each: Gaseous Form, Invisibility, Major Image, Plane
    Shift, Tongues, Wall of Fire (level 7 version)'
  sha256: 80cb484f8ffa6c10afb0a300425dcb20a42b5170f38649671de7fe9687cfd9a4
  status: contract_specified
splat_requires_mechanics: *id001
---

# Efreeti

<!-- SRD page 283 -->

Large Elemental (Genie), Neutral\
AC 17 Initiative +1 (11)\
HP 212 (17d10 + 119)\
Speed 40 ft., Fly 60 ft. (hover)\
MOD SAVE MOD SAVE MOD SAVE\
Str 22 +6 +6 Dex 12 +1 +1 Con 24 +7 +7\
Int 16 +3 +3 Wis 15 +2 +6 Cha 19 +4 +8\
Immunities Fire\
Senses Darkvision 120 ft.; Passive Perception 12\
Languages Primordial (Ignan)\
CR 11 (XP 7,200; PB +4)\

### Traits

Elemental Restoration. If the efreeti dies outside the\
Elemental Plane of Fire, its body dissolves into ash, and\
it gains a new body in 1d4 days, reviving with all its Hit\
Points somewhere on the Plane of Fire.\
Magic Resistance. The efreeti has Advantage on saving\
throws against spells and other magical effects.\
Wishes. The efreeti has a 30 percent chance of know-\
ing the Wish spell. If the efreeti knows it, the efreeti\
can cast it only on behalf of a non-genie creature who\
communicates a wish in a way the efreeti can under-\
stand. If the efreeti casts the spell for the creature, the\
efreeti suffers none of the spell’s stress. Once the efreeti\
has cast it three times, the efreeti can’t do so again\
for 365 days.\

### Actions

Multiattack. The efreeti makes three attacks, using\
Heated Blade or Hurl Flame in any combination.\
Heated Blade. Melee Attack Roll: +10, reach 5 ft.\
Hit: 13 (2d6 + 6) Slashing damage plus 13 (2d12)\
Fire damage.\
Hurl Flame. Ranged Attack Roll: +8, range 120 ft. Hit:\
24 (7d6) Fire damage.\
Spellcasting. The efreeti casts one of the following\
spells, requiring no Material components and using\
Charisma as the spellcasting ability (spell save DC 16):\
At Will: Detect Magic, Elementalism\
1/Day Each: Gaseous Form, Invisibility, Major Image,\
Plane Shift, Tongues, Wall of Fire (level 7 version)\

Original layout: [[../../srd-5.2.1/pages/283|page 283]].
