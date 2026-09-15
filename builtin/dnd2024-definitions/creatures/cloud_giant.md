---
splat_schema: 1
splat_id: ae3f8349-3c70-579f-abc5-99126ce42f69
splat_kind: creature
splat_key: dnd2024:creatures/cloud_giant
splat_name: Cloud Giant
splat_origin:
  key: dnd2024:reference/creature/cloud_giant
splat_sources:
- title: SRD 5.2.1
  locator: pp. 275
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: huge
  type: giant
  alignment: Neutral
  movement_text: 40 ft., Fly 20 ft. (hover)
  skills_text: Insight +7, Perception +11
  senses_text: Passive Perception 21
  languages_text: Common, Giant
  cr_text: 9 (XP 5,000; PB +4)
  hit_points_text: 200 (16d12 + 96)
splat_stats:
  strength:
    value: 27
  save_strength:
    value: 8
  dexterity:
    value: 10
  save_dexterity:
    value: 0
  constitution:
    value: 22
  save_constitution:
    value: 10
  intelligence:
    value: 12
  save_intelligence:
    value: 1
  wisdom:
    value: 16
  save_wisdom:
    value: 7
  charisma:
    value: 16
  save_charisma:
    value: 3
  armor_class:
    value: 14
  max_hp:
    value: 200
  initiative:
    value: 4
  speed:
    value: 40
    unit: ft
  fly_speed:
    value: 20
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/cloud_giant
  implementation: dnd2024:creature_cloud_giant
  parameters:
    skill_totals:
      insight: 7
      perception: 11
    senses: []
    passive_perception: 21
    languages:
      speak: true
      known:
      - common
      - giant
    features:
      thunderous_mace:
        economy: action
        op: attack
        modes:
        - melee
        attack_bonus: 12
        target:
          kind: creature_or_object
          count: 1
        reach_ft: 10
        hit:
        - op: damage
          components:
          - type: bludgeoning
            amount:
              dice:
                count: 3
                sides: 8
              add: 8
            average: 21
          - type: thunder
            amount:
              dice:
                count: 2
                sides: 6
              add: 0
            average: 7
      thundercloud:
        economy: action
        op: attack
        modes:
        - ranged
        attack_bonus: 12
        target:
          kind: creature_or_object
          count: 1
        range_ft:
          normal: 240
          long: 240
        hit:
        - op: damage
          components:
          - type: thunder
            amount:
              dice:
                count: 3
                sides: 6
              add: 8
            average: 18
        - op: condition.apply
          condition: incapacitated
          duration:
            until: target.next_turn_end
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
              key: dnd2024:spells/fog_cloud
            uses:
              unlimited: true
          - spell:
              key: dnd2024:spells/light
            uses:
              unlimited: true
          - spell:
              key: dnd2024:spells/control_weather
            uses:
              count: 1
              recovery: long_rest
          - spell:
              key: dnd2024:spells/gaseous_form
            uses:
              count: 1
              recovery: long_rest
          - spell:
              key: dnd2024:spells/telekinesis
            uses:
              count: 1
              recovery: long_rest
          save_dc: 15
      misty_step:
        economy: bonus_action
        op: spellcasting
        profile:
          ability: same_as_spellcasting
          waived_components: []
        choose_count: 1
        choices:
        - op: spell.cast
          spell:
            key: dnd2024:spells/misty_step
      multiattack:
        economy: action
        op: multiattack
        attacks:
          count: 2
          choices:
          - thunderous_mace
          - thundercloud
          combination: any
        replace_attack:
          maximum: 1
          choices:
          - op: spell.cast
            spell:
              key: dnd2024:spells/fog_cloud
            profile: spellcasting
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
- id: dnd2024:creature_cloud_giant
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
  source_sha256: b382e52c5fb9a0d51615251b7f6611f20710853c4c910b965769643cb53a8a0c
  source_path: builtin/srd-5.2.1/creature/cloud_giant.md
  pages:
  - 275
  engine_gaps: &id001
  - dnd2024:creature_cloud_giant
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_3.md
  specification_sha256: bd9be63bee26747cb7b2d42ad2e3deca2491b430733ed73cdb5707e325b3dc0e
splat_clauses:
- key: source_rules
  text: '# Cloud Giant Huge Giant, Neutral AC 14 Initiative +4 (14) HP 200 (16d12 + 96) Speed 40 ft.,
    Fly 20 ft. (hover) MOD SAVE MOD SAVE MOD SAVE Str 27 +8 +8 Dex 10 +0 +0 Con 22 +6 +10 Int 12 +1 +1
    Wis 16 +3 +7 Cha 16 +3 +3 Skills Insight +7, Perception +11 Senses Passive Perception 21 Languages
    Common, Giant CR 9 (XP 5,000; PB +4) ### Actions Multiattack. The giant makes two attacks, using Thunderous
    Mace or Thundercloud in any combination. It can replace one attack with a use of Spellcasting to cast
    Fog Cloud. Thunderous Mace. Melee Attack Roll: +12, reach 10 ft. Hit: 21 (3d8 + 8) Bludgeoning damage
    plus 7 (2d6) Thunder damage. Thundercloud. Ranged Attack Roll: +12, range 240 ft. Hit: 18 (3d6 + 8)
    Thunder damage, and the target has the Incapacitated condition until the end of its next turn. Spellcasting.
    The giant casts one of the following spells, requiring no Material components and using Charisma as
    the spellcasting ability (spell save DC 15): At Will: Detect Magic, Fog Cloud, Light 1/Day Each: Control
    Weather, Gaseous Form, Telekinesis ### Bonus Actions Misty Step. The giant casts the Misty Step spell,
    using the same spellcasting ability as Spellcasting.'
  sha256: b22dc2be8fa523a430b575998d4c25ceeffe72e0d97300a52fcc1968840ebec5
  status: contract_specified
splat_requires_mechanics: *id001
---

# Cloud Giant

<!-- SRD page 275 -->

Huge Giant, Neutral\
AC 14 Initiative +4 (14)\
HP 200 (16d12 + 96)\
Speed 40 ft., Fly 20 ft. (hover)\
MOD SAVE MOD SAVE MOD SAVE\
Str 27 +8 +8 Dex 10 +0 +0 Con 22 +6 +10\
Int 12 +1 +1 Wis 16 +3 +7 Cha 16 +3 +3\
Skills Insight +7, Perception +11\
Senses Passive Perception 21\
Languages Common, Giant\
CR 9 (XP 5,000; PB +4)\

### Actions

Multiattack. The giant makes two attacks, using Thun-\
derous Mace or Thundercloud in any combination. It\
can replace one attack with a use of Spellcasting to cast\
Fog Cloud.\
Thunderous Mace. Melee Attack Roll: +12, reach 10\
ft. Hit: 21 (3d8 + 8) Bludgeoning damage plus 7 (2d6)\
Thunder damage.\
Thundercloud. Ranged Attack Roll: +12, range 240\
ft. Hit: 18 (3d6 + 8) Thunder damage, and the target\
has the Incapacitated condition until the end of its\
next turn.\
Spellcasting. The giant casts one of the following spells,\
requiring no Material components and using Charisma\
as the spellcasting ability (spell save DC 15):\
At Will: Detect Magic, Fog Cloud, Light\
1/Day Each: Control Weather, Gaseous Form,\
Telekinesis\

### Bonus Actions

Misty Step. The giant casts the Misty Step spell, using\
the same spellcasting ability as Spellcasting.\

Original layout: [[../../srd-5.2.1/pages/275|page 275]].
