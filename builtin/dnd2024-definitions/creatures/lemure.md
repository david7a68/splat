---
splat_schema: 1
splat_id: 50248aa3-89ce-55b8-8c50-74dfde4d4220
splat_kind: creature
splat_key: dnd2024:creatures/lemure
splat_name: Lemure
splat_origin:
  key: dnd2024:reference/creature/lemure
splat_sources:
- title: SRD 5.2.1
  locator: pp. 304
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: medium
  type: fiend
  alignment: Lawful Evil
  tags_text: (Devil)
  movement_text: 20 ft.
  resistances_text: Cold
  immunities_text: Fire, Poison; Charmed, Frightened, Poisoned
  senses_text: Darkvision 120 ft. (unimpeded by magical Darkness); Passive Perception 10
  languages_text: Understands Infernal but can’t speak
  cr_text: 0 (XP 10; PB +2)
  hit_points_text: 9 (2d8)
splat_stats:
  strength:
    value: 10
  save_strength:
    value: 0
  dexterity:
    value: 5
  save_dexterity:
    value: -3
  constitution:
    value: 11
  save_constitution:
    value: 0
  intelligence:
    value: 1
  save_intelligence:
    value: -5
  wisdom:
    value: 11
  save_wisdom:
    value: 0
  charisma:
    value: 3
  save_charisma:
    value: -4
  armor_class:
    value: 9
  max_hp:
    value: 9
  initiative:
    value: -3
  speed:
    value: 20
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/lemure
  implementation: dnd2024:creature_lemure
  parameters:
    damage_resistances:
    - cold
    damage_immunities:
    - fire
    - poison
    condition_immunities:
    - charmed
    - frightened
    - poisoned
    senses:
    - sense: darkvision
      range_ft: 120
    passive_perception: 10
    darkvision_ignores_magical_darkness: true
    languages:
      speak: false
      known:
      - infernal
    features:
      vile_slime:
        economy: action
        op: attack
        modes:
        - melee
        attack_bonus: 2
        target:
          kind: creature_or_object
          count: 1
        reach_ft: 5
        hit:
        - op: damage
          components:
          - type: poison
            amount:
              dice:
                count: 1
                sides: 4
              add: 0
            average: 2
      hellish_restoration:
        economy: passive
        trigger: actor.died
        when:
          plane: nine_hells
        revive:
          delay:
            amount:
              dice:
                count: 1
                sides: 10
              add: 0
            unit: day
          hp: maximum
        prevented_by:
        - killer_has_spell:
            key: dnd2024:spells/bless
        - remains_sprinkled_with: holy_water
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
- id: dnd2024:creature_lemure
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
  - spatial_constraints
splat_conversion:
  status: specified
  category: creature
  source_sha256: 385900ca9b61b10dd753f4c9e1d42661b13a8dede39e5f99bbf395131f36fe64
  source_path: builtin/srd-5.2.1/creature/lemure.md
  pages:
  - 304
  engine_gaps: &id001
  - dnd2024:creature_lemure
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_3.md
  specification_sha256: dd96acdb5208acd67069a4d726b594dc9eb72722ef3869b73e9205d3d8feedeb
splat_clauses:
- key: source_rules
  text: '# Lemure Medium Fiend (Devil), Lawful Evil AC 9 Initiative -3 (7) HP 9 (2d8) Speed 20 ft. MOD
    SAVE MOD SAVE MOD SAVE Str 10 +0 +0 Dex 5 -3 -3 Con 11 +0 +0 Int 1 -5 -5 Wis 11 +0 +0 Cha 3 -4 -4
    Resistances Cold Immunities Fire, Poison; Charmed, Frightened, Poisoned Senses Darkvision 120 ft.
    (unimpeded by magical Darkness); Passive Perception 10 Languages Understands Infernal but can’t speak
    CR 0 (XP 10; PB +2) ### Traits Hellish Restoration. If the lemure dies in the Nine Hells, it revives
    with all its Hit Points in 1d10 days unless it is killed by a creature under the effects of a Bless
    spell or its remains are sprinkled with Holy Water. ### Actions Vile Slime. Melee Attack Roll: +2,
    reach 5 ft. Hit: 2 (1d4) Poison damage.'
  sha256: c4da7fc604e6a7d1c76f6e4e76e4b04fa67b5ebc310a51940b36b912a138a641
  status: contract_specified
splat_requires_mechanics: *id001
---

# Lemure

<!-- SRD page 304 -->

Medium Fiend (Devil), Lawful Evil\
AC 9 Initiative −3 (7)\
HP 9 (2d8)\
Speed 20 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 10 +0 +0 Dex 5 −3 −3 Con 11 +0 +0\
Int 1 −5 −5 Wis 11 +0 +0 Cha 3 −4 −4\
Resistances Cold\
Immunities Fire, Poison; Charmed, Frightened, Poisoned\
Senses Darkvision 120 ft. (unimpeded by magical\
Darkness); Passive Perception 10\
Languages Understands Infernal but can’t speak\
CR 0 (XP 10; PB +2)\

### Traits

Hellish Restoration. If the lemure dies in the Nine\
Hells, it revives with all its Hit Points in 1d10 days un-\
less it is killed by a creature under the effects of a Bless\
spell or its remains are sprinkled with Holy Water.\

### Actions

Vile Slime. Melee Attack Roll: +2, reach 5 ft. Hit: 2\
(1d4) Poison damage.\

Original layout: [[../../srd-5.2.1/pages/304|page 304]].
