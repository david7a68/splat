---
splat_schema: 1
splat_id: 6b8e85fc-e125-5c0d-96a0-7c8f04cd52e9
splat_kind: creature
splat_key: dnd2024:creatures/shrieker_fungus
splat_name: Shrieker Fungus
splat_origin:
  key: dnd2024:reference/creature/shrieker_fungus
splat_sources:
- title: SRD 5.2.1
  locator: pp. 286
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: medium
  type: plant
  alignment: Unaligned
  movement_text: 5 ft.
  immunities_text: Blinded, Charmed, Deafened, Frightened
  senses_text: Blindsight 30 ft.; Passive Perception 6
  languages_text: None
  cr_text: 0 (XP 0; PB +2)
  hit_points_text: 13 (3d8)
splat_stats:
  strength:
    value: 1
  save_strength:
    value: -5
  dexterity:
    value: 1
  save_dexterity:
    value: -5
  constitution:
    value: 10
  save_constitution:
    value: 0
  intelligence:
    value: 1
  save_intelligence:
    value: -5
  wisdom:
    value: 3
  save_wisdom:
    value: -4
  charisma:
    value: 1
  save_charisma:
    value: -5
  armor_class:
    value: 5
  max_hp:
    value: 13
  initiative:
    value: -5
  speed:
    value: 5
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/shrieker_fungus
  implementation: dnd2024:creature_shrieker_fungus
  parameters:
    damage_immunities: []
    condition_immunities:
    - blinded
    - charmed
    - deafened
    - frightened
    senses:
    - sense: blindsight
      range_ft: 30
    passive_perception: 6
    languages:
      speak: true
      known: []
    features:
      shriek:
        economy: reaction
        trigger: entity.moved
        op: sound.emit
        audible_radius_ft: 300
        duration:
          maximum:
            amount: 1
            unit: minute
          until: actor.died
        when:
          entity_kind_any:
          - creature
          - bright_light_source
          distance_to_actor_ft_at_most: 30
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
- id: dnd2024:creature_shrieker_fungus
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
  - entity.moved
  capabilities:
  - action_economy
  - creature_rules
  - effects
  - spatial_constraints
splat_conversion:
  status: specified
  category: creature
  source_sha256: f19ca548e3a02bde53d863c5c633ea9cddd2ea2341d01db18c66aeb3256017c1
  source_path: builtin/srd-5.2.1/creature/shrieker_fungus.md
  pages:
  - 286
  engine_gaps: &id001
  - dnd2024:creature_shrieker_fungus
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_3.md
  specification_sha256: 74151e2e43276b4c3786b166dbc3f8d24035f3090138e9752dcae190855d84dc
splat_clauses:
- key: source_rules
  text: '# Shrieker Fungus Medium Plant, Unaligned AC 5 Initiative -5 (5) HP 13 (3d8) Speed 5 ft. MOD
    SAVE MOD SAVE MOD SAVE Str 1 -5 -5 Dex 1 -5 -5 Con 10 +0 +0 Int 1 -5 -5 Wis 3 -4 -4 Cha 1 -5 -5 Immunities
    Blinded, Charmed, Deafened, Frightened Senses Blindsight 30 ft.; Passive Perception 6 Languages None
    CR 0 (XP 0; PB +2) ### Reactions Shriek. Trigger: A creature or a source of Bright Light moves within
    30 feet of the shrieker. Response: The shrieker emits a shriek audible within 300 feet of itself for
    1 minute or until the shrieker dies.'
  sha256: 1db0b8425f92c6cac0aa034745face8158cac96437406b4c83508a3c58331618
  status: contract_specified
splat_requires_mechanics: *id001
---

# Shrieker Fungus

<!-- SRD page 286 -->

Medium Plant, Unaligned\
AC 5 Initiative −5 (5)\
HP 13 (3d8)\
Speed 5 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 1 −5 −5 Dex 1 −5 −5 Con 10 +0 +0\
Int 1 −5 −5 Wis 3 −4 −4 Cha 1 −5 −5\
Immunities Blinded, Charmed, Deafened, Frightened\
Senses Blindsight 30 ft.; Passive Perception 6\
Languages None\
CR 0 (XP 0; PB +2)\

### Reactions

Shriek. Trigger: A creature or a source of Bright Light\
moves within 30 feet of the shrieker. Response: The\
shrieker emits a shriek audible within 300 feet of itself\
for 1 minute or until the shrieker dies.\

Original layout: [[../../srd-5.2.1/pages/286|page 286]].
