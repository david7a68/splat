---
splat_schema: 1
splat_id: 2d3a918a-f418-5eea-a832-45d6ee544c4c
splat_kind: creature
splat_key: dnd2024:creatures/ochre_jelly
splat_name: Ochre Jelly
splat_origin:
  key: dnd2024:reference/creature/ochre_jelly
splat_sources:
- title: SRD 5.2.1
  locator: pp. 312
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: large
  type: ooze
  alignment: Unaligned
  movement_text: 20 ft., Climb 20 ft.
  resistances_text: Acid
  immunities_text: Lightning, Slashing; Charmed, Deafened, Exhaustion, Frightened, Grappled, Prone, Restrained
  senses_text: Blindsight 60 ft.; Passive Perception 8
  languages_text: None
  cr_text: 2 (XP 450; PB +2)
  hit_points_text: 52 (7d10 + 14)
splat_stats:
  strength:
    value: 15
  save_strength:
    value: 2
  dexterity:
    value: 6
  save_dexterity:
    value: -2
  constitution:
    value: 14
  save_constitution:
    value: 2
  intelligence:
    value: 2
  save_intelligence:
    value: -4
  wisdom:
    value: 6
  save_wisdom:
    value: -2
  charisma:
    value: 1
  save_charisma:
    value: -5
  armor_class:
    value: 8
  max_hp:
    value: 52
  initiative:
    value: -2
  speed:
    value: 20
    unit: ft
  climb_speed:
    value: 20
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/ochre_jelly
  implementation: dnd2024:creature_ochre_jelly
  parameters:
    damage_resistances:
    - acid
    damage_immunities:
    - lightning
    - slashing
    condition_immunities:
    - charmed
    - deafened
    - exhaustion
    - frightened
    - grappled
    - prone
    - restrained
    senses:
    - sense: blindsight
      range_ft: 60
    passive_perception: 8
    languages:
      speak: true
      known: []
    features:
      amorphous:
        economy: passive
        op: movement.form
        minimum_opening_inches: 1
        extra_movement_cost: 0
      spider_climb:
        economy: passive
        op: movement.climb
        difficult_surfaces: true
        ceilings: true
        ability_check_required: false
      pseudopod:
        economy: action
        op: attack
        modes:
        - melee
        attack_bonus: 4
        target:
          kind: creature_or_object
          count: 1
        reach_ft: 5
        hit:
        - op: damage
          components:
          - type: acid
            amount:
              dice:
                count: 3
                sides: 6
              add: 2
            average: 12
      split:
        economy: reaction
        trigger_any:
        - actor.becomes_bloodied
        - actor.subjected_to_lightning_damage
        - actor.subjected_to_slashing_damage
        requires:
          sizes:
          - large
          - medium
          hp_at_least: 10
        op: actor.replace_with_creatures
        definition:
          key: dnd2024:creatures/ochre_jelly
        count: 2
        each:
          size_steps: -1
          initiative: original_actor
          hp:
            divide:
            - actor_stat: hp
            - 2
            round: down
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
- id: dnd2024:creature_ochre_jelly
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
  source_sha256: 1341280091353ed0b1b2533220d55995f3fe70d9f85d92f5c13de68ae85d9e78
  source_path: builtin/srd-5.2.1/creature/ochre_jelly.md
  pages:
  - 312
  engine_gaps: &id001
  - dnd2024:creature_ochre_jelly
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_3.md
  specification_sha256: fcb80700f01f2f5070000c786e9b4dae93c232f802a626656b74eea77c465173
splat_clauses:
- key: source_rules
  text: '# Ochre Jelly Large Ooze, Unaligned AC 8 Initiative -2 (8) HP 52 (7d10 + 14) Speed 20 ft., Climb
    20 ft. MOD SAVE MOD SAVE MOD SAVE Str 15 +2 +2 Dex 6 -2 -2 Con 14 +2 +2 Int 2 -4 -4 Wis 6 -2 -2 Cha
    1 -5 -5 Resistances Acid Immunities Lightning, Slashing; Charmed, Deafened, Exhaustion, Frightened,
    Grappled, Prone, Restrained Senses Blindsight 60 ft.; Passive Perception 8 Languages None CR 2 (XP
    450; PB +2) ### Traits Amorphous. The jelly can move through a space as narrow as 1 inch without expending
    extra movement to do so. Spider Climb. The jelly can climb difficult surfaces, including along ceilings,
    without needing to make an ability check. ### Actions Pseudopod. Melee Attack Roll: +4, reach 5 ft.
    Hit: 12 (3d6 + 2) Acid damage. ### Reactions Split. Trigger: While the jelly is Large or Medium and
    has 10+ Hit Points, it becomes Bloodied or is subjected to Lightning or Slashing damage. Response:
    The jelly splits into two new Ochre Jellies. Each new jelly is one size smaller than the original
    jelly and acts on its Initiative. The original jelly’s Hit Points are divided evenly between the new
    jellies (round down).'
  sha256: ccff5f6fd52bec7ce5cf56475d512c1c0cc0eb4b4a39e8b9270b72194078ecdd
  status: contract_specified
splat_requires_mechanics: *id001
---

# Ochre Jelly

<!-- SRD page 312 -->

Large Ooze, Unaligned\
AC 8 Initiative −2 (8)\
HP 52 (7d10 + 14)\
Speed 20 ft., Climb 20 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 15 +2 +2 Dex 6 −2 −2 Con 14 +2 +2\
Int 2 −4 −4 Wis 6 −2 −2 Cha 1 −5 −5\
Resistances Acid\
Immunities Lightning, Slashing; Charmed, Deafened,\
Exhaustion, Frightened, Grappled, Prone, Restrained\
Senses Blindsight 60 ft.; Passive Perception 8\
Languages None\
CR 2 (XP 450; PB +2)\

### Traits

Amorphous. The jelly can move through a space\
as narrow as 1 inch without expending extra move-\
ment to do so.\
Spider Climb. The jelly can climb difficult surfaces,\
including along ceilings, without needing to make an\
ability check.\

### Actions

Pseudopod. Melee Attack Roll: +4, reach 5 ft. Hit: 12\
(3d6 + 2) Acid damage.\

### Reactions

Split. Trigger: While the jelly is Large or Medium and\
has 10+ Hit Points, it becomes Bloodied or is subjected\
to Lightning or Slashing damage. Response: The jelly\
splits into two new Ochre Jellies. Each new jelly is one\
size smaller than the original jelly and acts on its Initia-\
tive. The original jelly’s Hit Points are divided evenly\
between the new jellies (round down).\

Original layout: [[../../srd-5.2.1/pages/312|page 312]].
