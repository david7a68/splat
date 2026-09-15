---
splat_schema: 1
splat_id: 7adeeade-0bcf-5c22-a420-eb750d4da8f6
splat_kind: creature
splat_key: dnd2024:creatures/djinni
splat_name: Djinni
splat_origin:
  key: dnd2024:reference/creature/djinni
splat_sources:
- title: SRD 5.2.1
  locator: pp. 280
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: large
  type: elemental
  alignment: Neutral
  tags_text: (Genie)
  movement_text: 30 ft., Fly 90 ft. (hover)
  immunities_text: Lightning, Thunder
  senses_text: Darkvision 120 ft.; Passive Perception 13
  languages_text: Primordial (Auran)
  cr_text: 11 (XP 7,200; PB +4)
  hit_points_text: 218 (19d10 + 114)
splat_stats:
  strength:
    value: 21
  save_strength:
    value: 5
  dexterity:
    value: 15
  save_dexterity:
    value: 6
  constitution:
    value: 22
  save_constitution:
    value: 6
  intelligence:
    value: 15
  save_intelligence:
    value: 2
  wisdom:
    value: 16
  save_wisdom:
    value: 7
  charisma:
    value: 20
  save_charisma:
    value: 5
  armor_class:
    value: 17
  max_hp:
    value: 218
  initiative:
    value: 2
  speed:
    value: 30
    unit: ft
  fly_speed:
    value: 90
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/djinni
  implementation: dnd2024:creature_djinni
  parameters:
    damage_immunities:
    - lightning
    - thunder
    condition_immunities: []
    senses:
    - sense: darkvision
      range_ft: 120
    passive_perception: 13
    languages:
      speak: true
      known:
      - primordial
      primordial_dialects:
      - auran
    features:
      elemental_restoration:
        economy: passive
        trigger: actor.died
        when:
          plane_not: elemental_plane_of_air
        body: disappears
        revive:
          new_body: true
          plane: elemental_plane_of_air
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
      storm_blade:
        economy: action
        op: attack
        modes:
        - melee
        attack_bonus: 9
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
              add: 5
            average: 12
          - type: lightning
            amount:
              dice:
                count: 2
                sides: 6
              add: 0
            average: 7
      storm_bolt:
        economy: action
        op: attack
        modes:
        - ranged
        attack_bonus: 9
        target:
          kind: creature_or_object
          count: 1
        range_ft:
          normal: 120
          long: 120
        hit:
        - op: damage
          components:
          - type: thunder
            amount:
              dice:
                count: 3
                sides: 8
              add: 0
            average: 13
        - op: condition.apply
          condition: prone
          when:
            target_kind: creature
            target_maximum_size: large
      spellcasting:
        economy: action
        op: spellcasting
        profile:
          ability: charisma
          waived_components:
          - material
          spells:
          - spell:
              key: dnd2024:spells/detect_evil_and_good
            uses:
              unlimited: true
          - spell:
              key: dnd2024:spells/detect_magic
            uses:
              unlimited: true
          - spell:
              key: dnd2024:spells/create_food_and_water
            uses:
              count: 2
              recovery: long_rest
            overrides:
              water_replacement_choice:
              - water
              - wine
          - spell:
              key: dnd2024:spells/tongues
            uses:
              count: 2
              recovery: long_rest
          - spell:
              key: dnd2024:spells/wind_walk
            uses:
              count: 2
              recovery: long_rest
          - spell:
              key: dnd2024:spells/creation
            uses:
              count: 1
              recovery: long_rest
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
          save_dc: 17
      create_whirlwind:
        economy: action
        op: area.create
        target_location:
          range_ft: 120
          visible_to_actor: true
        area:
          shape: cylinder
          radius_ft: 20
          height_ft: 60
        duration:
          until: actor.concentration_end
        movement:
          event: actor.turn_start
          maximum_ft: 20
          optional: true
        entry_events:
        - area.enters_creature_space
        - creature.enters_area
        entry_save:
          op: saving_throw
          ability: strength
          dc: 17
          targets:
            kind: creature
            exclude: actor
            maximum_checks_per_turn: 1
          failure:
          - op: condition.apply
            condition: restrained
            duration:
              until: leaves_area_or_successful_repeat_save
            moves_with_area: true
            on_target_turn_start:
            - op: damage
              components:
              - type: thunder
                amount:
                  dice:
                    count: 6
                    sides: 6
                  add: 0
                average: 21
            repeat_save:
              event: target.turn_end
              ability: strength
              dc: 17
              success: effect.end
          success: []
      multiattack:
        economy: action
        op: multiattack
        attacks:
          count: 3
          choices:
          - storm_blade
          - storm_bolt
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
- id: dnd2024:creature_djinni
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
  - actor.turn_start
  - creature.feature_requested
  - target.turn_end
  capabilities:
  - action_economy
  - attack_resolution
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
  source_sha256: 1fbb270a5e437e4ab4fc146a6668e11f68883b70b9a4947e026ff74850500272
  source_path: builtin/srd-5.2.1/creature/djinni.md
  pages:
  - 280
  engine_gaps: &id001
  - dnd2024:creature_djinni
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_3.md
  specification_sha256: 6daa64d0602eb725984f04a4d24663bef6e6359355a2ed82192556ae6062bd05
splat_clauses:
- key: source_rules
  text: '# Djinni Large Elemental (Genie), Neutral AC 17 Initiative +2 (12) HP 218 (19d10 + 114) Speed
    30 ft., Fly 90 ft. (hover) MOD SAVE MOD SAVE MOD SAVE Str 21 +5 +5 Dex 15 +2 +6 Con 22 +6 +6 Int 15
    +2 +2 Wis 16 +3 +7 Cha 20 +5 +5 Immunities Lightning, Thunder Senses Darkvision 120 ft.; Passive Perception
    13 Languages Primordial (Auran) CR 11 (XP 7,200; PB +4) ### Traits Elemental Restoration. If the djinni
    dies outside the Elemental Plane of Air, its body dissolves into mist, and it gains a new body in
    1d4 days, reviving with all its Hit Points somewhere on the Plane of Air. Magic Resistance. The djinni
    has Advantage on saving throws against spells and other magical effects. Wishes. The djinni has a
    30 percent chance of knowing the Wish spell. If the djinni knows it, the djinni can cast it only on
    behalf of a non-genie creature who communicates a wish in a way the djinni can understand. If the
    djinni casts the spell for the creature, the djinni suffers none of the spell’s stress. Once the djinni
    has cast it three times, the djinni can’t do so again for 365 days. ### Actions Multiattack. The djinni
    makes three attacks, using Storm Blade or Storm Bolt in any combination. Storm Blade. Melee Attack
    Roll: +9, reach 5 feet. Hit: 12 (2d6 + 5) Slashing damage plus 7 (2d6) Lightning damage. Storm Bolt.
    Ranged Attack Roll: +9, range 120 feet. Hit: 13 (3d8) Thunder damage. If the target is a Large or
    smaller creature, it has the Prone condition. Create Whirlwind. The djinni conjures a whirlwind at
    a point it can see within 120 feet. The whirlwind fills a 20-foot-radius, 60-foot-high Cylinder centered
    on that point. The whirlwind lasts until the djinni’s Concentration on it ends. The djinni can move
    the whirlwind up to 20 feet at the start of each of its turns. Whenever the whirlwind enters a creature’s
    space or a creature enters the whirlwind, that creature is subjected to the following effect. Strength
    Saving Throw: DC 17 (a creature makes this save only once per turn, and the djinni is unaffected).
    Failure: While in the whirlwind, the target has the Restrained condition and moves with the whirlwind.
    At the start of each of its turns, the Restrained target takes 21 (6d6) Thunder damage. At the end
    of each of its turns, the target repeats the save, ending the effect on itself on a success. Spellcasting.
    The djinni casts one of the following spells, requiring no Material components and using Charisma
    as the spellcasting ability (spell save DC 17): At Will: Detect Evil and Good, Detect Magic 2/Day
    Each: Create Food and Water (can create wine instead of water), Tongues, Wind Walk 1/Day Each: Creation,
    Gaseous Form, Invisibility, Major Image, Plane Shift'
  sha256: 88a4a52845f03cf698b7e3a5f7199d4e44ca778c486d2f87daf264bfbc9f900e
  status: contract_specified
splat_requires_mechanics: *id001
---

# Djinni

<!-- SRD page 280 -->

Large Elemental (Genie), Neutral\
AC 17  Initiative +2 (12)\
HP 218 (19d10 + 114)\
Speed 30 ft., Fly 90 ft. (hover)\
MOD SAVE MOD SAVE MOD SAVE\
Str 21 +5 +5 Dex 15 +2 +6 Con 22 +6 +6\
Int 15 +2 +2 Wis 16 +3 +7 Cha 20 +5 +5\
Immunities Lightning, Thunder\
Senses Darkvision 120 ft.; Passive Perception 13\
Languages Primordial (Auran)\
CR 11 (XP 7,200; PB +4)\

### Traits

Elemental Restoration. If the djinni dies outside the\
Elemental Plane of Air, its body dissolves into mist, and\
it gains a new body in 1d4 days, reviving with all its Hit\
Points somewhere on the Plane of Air.\
Magic Resistance. The djinni has Advantage on saving\
throws against spells and other magical effects.\
Wishes. The djinni has a 30 percent chance of knowing\
the Wish spell. If the djinni knows it, the djinni can cast\
it only on behalf of a non-genie creature who commu-\
nicates a wish in a way the djinni can understand. If the\
djinni casts the spell for the creature, the djinni suffers\
none of the spell’s stress. Once the djinni has cast it\
three times, the djinni can’t do so again for 365 days.\

### Actions

Multiattack. The djinni makes three attacks, using\
Storm Blade or Storm Bolt in any combination.\
Storm Blade. Melee Attack Roll: +9, reach 5 feet.\
Hit: 12 (2d6 + 5) Slashing damage plus 7 (2d6) Light-\
ning damage.\
Storm Bolt. Ranged Attack Roll: +9, range 120 feet.\
Hit: 13 (3d8) Thunder damage. If the target is a Large or\
smaller creature, it has the Prone condition.\
Create Whirlwind. The djinni conjures a whirlwind at\
a point it can see within 120 feet. The whirlwind fills a\
20-foot-radius, 60-foot-high Cylinder centered on that\
point. The whirlwind lasts until the djinni’s Concentra-\
tion on it ends. The djinni can move the whirlwind up\
to 20 feet at the start of each of its turns.\
Whenever the whirlwind enters a creature’s space or\
a creature enters the whirlwind, that creature is sub-\
jected to the following effect. Strength Saving Throw:\
DC 17 (a creature makes this save only once per turn,\
and the djinni is unaffected). Failure: While in the\
whirlwind, the target has the Restrained condition\
and moves with the whirlwind. At the start of each of\
its turns, the Restrained target takes 21 (6d6) Thunder\
damage. At the end of each of its turns, the target re-\
peats the save, ending the effect on itself on a success.\
Spellcasting. The djinni casts one of the following\
spells, requiring no Material components and using\
Charisma as the spellcasting ability (spell save DC 17):\
At Will: Detect Evil and Good, Detect Magic\
2/Day Each: Create Food and Water (can create wine\
instead of water), Tongues, Wind Walk\
1/Day Each: Creation, Gaseous Form, Invisibility, Major\
Image, Plane Shift\

Original layout: [[../../srd-5.2.1/pages/280|page 280]].
