---
splat_schema: 1
splat_id: 6cdc34e3-3dda-5178-8857-75a5d374f66a
splat_kind: creature
splat_key: dnd2024:creatures/panther
splat_name: Panther
splat_origin:
  key: dnd2024:reference/creature/panther
splat_sources:
- title: SRD 5.2.1
  locator: pp. 358
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: medium
  type: beast
  alignment: Unaligned
  movement_text: 50 ft., Climb 40 ft.
  skills_text: Perception +4, Stealth +7
  senses_text: Darkvision 60 ft.; Passive Perception 14
  languages_text: None
  cr_text: 1/4 (XP 50; PB +2)
  hit_points_text: 13 (3d8)
splat_stats:
  strength:
    value: 14
  save_strength:
    value: 2
  dexterity:
    value: 16
  save_dexterity:
    value: 3
  constitution:
    value: 10
  save_constitution:
    value: 0
  intelligence:
    value: 3
  save_intelligence:
    value: -4
  wisdom:
    value: 14
  save_wisdom:
    value: 2
  charisma:
    value: 7
  save_charisma:
    value: -2
  armor_class:
    value: 13
  max_hp:
    value: 13
  initiative:
    value: 3
  speed:
    value: 50
    unit: ft
  climb_speed:
    value: 40
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/panther
  implementation: dnd2024:creature_panther
  parameters:
    skill_totals:
      perception: 4
      stealth: 7
    senses:
    - sense: darkvision
      range_ft: 60
    passive_perception: 14
    languages:
      speak: true
      known: []
    features:
      rend:
        economy: action
        op: attack
        modes:
        - melee
        attack_bonus: 5
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
                count: 1
                sides: 6
              add: 3
            average: 6
      nimble_escape:
        economy: bonus_action
        op: action.choose
        choices:
        - disengage
        - hide
    damage_resolution:
      choice:
      - printed_average
      - roll_expression
      chooser: controller
      exclusive: true
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
- id: dnd2024:creature_panther
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
  - spatial_constraints
splat_conversion:
  status: specified
  category: creature
  source_sha256: 9eff9eeedcc8fdf7bfae566ff825e3a1624d361616c701fcafa3b525c1dc27e7
  source_path: builtin/srd-5.2.1/creature/panther.md
  pages:
  - 358
  engine_gaps: &id001
  - dnd2024:creature_panther
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_3.md
  specification_sha256: 8f097a9c71597afe0a598b4f66d941e4672fbb03e4a2f14c743ad48efcddc416
splat_clauses:
- key: source_rules
  text: '# Panther Medium Beast, Unaligned AC 13 Initiative +3 (13) HP 13 (3d8) Speed 50 ft., Climb 40
    ft. MOD SAVE MOD SAVE MOD SAVE Str 14 +2 +2 Dex 16 +3 +3 Con 10 +0 +0 Int 3 -4 -4 Wis 14 +2 +2 Cha
    7 -2 -2 Skills Perception +4, Stealth +7 Senses Darkvision 60 ft.; Passive Perception 14 Languages
    None CR 1/4 (XP 50; PB +2) ### Actions Rend. Melee Attack Roll: +5, reach 5 ft. Hit: 6 (1d6 + 3) Slashing
    damage. ### Bonus Actions Nimble Escape. The panther takes the Disengage or Hide action.'
  sha256: d2d8c6eec4762c5d2b4c113c898c785460d558a1d367d4b40f6d524e2c1cded0
  status: contract_specified
splat_requires_mechanics: *id001
---

# Panther

<!-- SRD page 358 -->

Medium Beast, Unaligned\
AC 13  Initiative +3 (13)\
HP 13 (3d8)\
Speed 50 ft., Climb 40 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str  14 +2 +2 Dex 16 +3 +3 Con 10 +0 +0\
Int 3 −4 −4 Wis 14 +2 +2 Cha 7 −2 −2\
Skills Perception +4, Stealth +7\
Senses Darkvision 60 ft.; Passive Perception 14\
Languages None\
CR 1/4 (XP 50; PB +2)\

### Actions

Rend. Melee Attack Roll: +5, reach 5 ft. Hit: 6 (1d6 + 3)\
Slashing damage.\

### Bonus Actions

Nimble Escape. The panther takes the Disengage or\
Hide action.\

Original layout: [[../../srd-5.2.1/pages/358|page 358]].
