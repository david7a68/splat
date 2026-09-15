---
splat_schema: 1
splat_id: 83a28d81-b401-51e6-a020-e6523168810e
splat_kind: creature
splat_key: dnd2024:creatures/rust_monster
splat_name: Rust Monster
splat_origin:
  key: dnd2024:reference/creature/rust_monster
splat_sources:
- title: SRD 5.2.1
  locator: pp. 320, 321
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: medium
  type: monstrosity
  alignment: Unaligned
  movement_text: 40 ft.
  senses_text: Darkvision 60 ft.; Passive Perception 11
  languages_text: None
  cr_text: 1/2 (XP 100; PB +2)
  hit_points_text: 33 (6d8 + 6)
splat_stats:
  strength:
    value: 13
  save_strength:
    value: 1
  dexterity:
    value: 12
  save_dexterity:
    value: 1
  constitution:
    value: 13
  save_constitution:
    value: 1
  intelligence:
    value: 2
  save_intelligence:
    value: -4
  wisdom:
    value: 13
  save_wisdom:
    value: 1
  charisma:
    value: 6
  save_charisma:
    value: -2
  armor_class:
    value: 14
  max_hp:
    value: 33
  initiative:
    value: 1
  speed:
    value: 40
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/rust_monster
  implementation: dnd2024:creature_rust_monster
  parameters:
    senses:
    - sense: darkvision
      range_ft: 60
    passive_perception: 11
    languages:
      speak: true
      known: []
    features:
      iron_scent:
        economy: passive
        op: location.pinpoint
        material: ferrous_metal
        range_ft: 30
      bite:
        economy: action
        op: attack
        modes:
        - melee
        attack_bonus: 3
        target:
          kind: creature_or_object
          count: 1
        reach_ft: 5
        hit:
        - op: damage
          components:
          - type: piercing
            amount:
              dice:
                count: 1
                sides: 8
              add: 1
            average: 5
      antennae:
        economy: action
        op: item.target
        target:
          kind: object
          magical: false
          material: metal
          categories:
          - armor
          - weapon
          worn_or_carried_by_creature: true
          range_ft: 5
        bearer_save:
          op: saving_throw
          ability: dexterity
          dc: 11
          targets:
            kind: item_bearer
          failure:
          - op: item.penalty
            cumulative: true
            amount: -1
            armor:
              stat: provided_ac
              destroy_at_ac: 10
            weapon:
              stat: attack_roll
              destroy_at_penalty: -5
            remove_penalty_spell:
              key: dnd2024:spells/mending
          success: []
      destroy_metal:
        economy: action
        op: object.destroy_volume
        range_ft: 5
        requires_touch: true
        target:
          magical: false
          material: metal
          worn_or_carried: false
        volume:
          shape: cube
          edge_ft: 1
      reflexive_antennae:
        economy: reaction
        trigger: attack.hits_actor
        effect:
          op: feature.use
          feature: antennae
      multiattack:
        economy: action
        op: multiattack
        sequence:
        - feature: bite
          count: 1
        - feature: antennae
          count: 2
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
- id: dnd2024:creature_rust_monster
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
  - attack.hits_actor
  - creature.feature_requested
  capabilities:
  - action_economy
  - attack_resolution
  - creature_rules
  - damage
  - effects
  - item_state
  - saving_throws
  - spatial_constraints
splat_conversion:
  status: specified
  category: creature
  source_sha256: ebd69ac080948dc7c8d6c6f6d8b55fd9f7fc8ea48b10b4a529f8873d66ec59b5
  source_path: builtin/srd-5.2.1/creature/rust_monster.md
  pages:
  - 320
  - 321
  engine_gaps: &id001
  - dnd2024:creature_rust_monster
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_3.md
  specification_sha256: f3fd91bcb1d100cac5f605afc057a90855708fec22416f4ad3fccd707970e5b2
splat_clauses:
- key: source_rules
  text: '# Rust Monster Medium Monstrosity, Unaligned AC 14 Initiative +1 (11) HP 33 (6d8 + 6) Speed 40
    ft. MOD SAVE MOD SAVE MOD SAVE Str 13 +1 +1 Dex 12 +1 +1 Con 13 +1 +1 Int 2 -4 -4 Wis 13 +1 +1 Cha
    6 -2 -2 Senses Darkvision 60 ft.; Passive Perception 11 Languages None CR 1/2 (XP 100; PB +2) ###
    Traits Iron Scent. The rust monster can pinpoint the location of ferrous metal within 30 feet of itself.
    ### Actions Multiattack. The rust monster makes one Bite attack and uses Antennae twice. Bite. Melee
    Attack Roll: +3, reach 5 ft. Hit: 5 (1d8 + 1) Piercing damage. Antennae. The rust monster targets
    one nonmagical metal object—armor or a weapon—worn or carried by a creature within 5 feet of itself.
    Dexterity Saving Throw: DC 11, the creature with the object. Failure: The object takes a -1 penalty
    to the AC it offers (armor) or to its attack rolls (weapon). Armor is destroyed if the penalty reduces
    its AC to 10, and a weapon is destroyed if its penalty reaches -5. The penalty can be removed by casting
    the Mending spell on the armor or weapon. Destroy Metal. The rust monster touches a nonmagical metal
    object within 5 feet of itself that isn’t being worn or carried. The touch destroys a 1-foot Cube
    of the object. ### Reactions Reflexive Antennae. Trigger: An attack roll hits the rust monster. Response:
    The rust monster uses Antennae.'
  sha256: 9a20721e5d9cc3e9386d74a52eb8a537d282d42b4daf2531f80bee232655b8b5
  status: contract_specified
splat_requires_mechanics: *id001
---

# Rust Monster

<!-- SRD page 320 -->

Medium Monstrosity, Unaligned\
AC 14  Initiative +1 (11)\
HP 33 (6d8 + 6)\
Speed 40 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 13 +1 +1 Dex 12 +1 +1 Con 13 +1 +1\
Int 2 −4 −4 Wis 13 +1 +1 Cha 6 −2 −2\
Senses Darkvision 60 ft.; Passive Perception 11\
Languages None\
CR 1/2 (XP 100; PB +2)\

### Traits

Iron Scent. The rust monster can pinpoint the location\
of ferrous metal within 30 feet of itself.\

### Actions

Multiattack. The rust monster makes one Bite attack\
and uses Antennae twice.\
Bite. Melee Attack Roll: +3, reach 5 ft. Hit: 5 (1d8 + 1)\
Piercing damage.\

<!-- SRD page 321 -->

Antennae. The rust monster targets one nonmagical\
metal object—armor or a weapon—worn or carried\
by a creature within 5 feet of itself. Dexterity Saving\
Throw: DC 11, the creature with the object. Failure:\
The object takes a −1 penalty to the AC it offers (ar-\
mor) or to its attack rolls (weapon). Armor is destroyed\
if the penalty reduces its AC to 10, and a weapon is\
destroyed if its penalty reaches −5. The penalty can be\
removed by casting the Mending spell on the armor\
or weapon.\
Destroy Metal. The rust monster touches a nonmagi-\
cal metal object within 5 feet of itself that isn’t being\
worn or carried. The touch destroys a 1-foot Cube of\
the object.\

### Reactions

Reflexive Antennae. Trigger: An attack roll hits the rust\
monster. Response: The rust monster uses Antennae.\

Original layout: [[../../srd-5.2.1/pages/320|page 320]], [[../../srd-5.2.1/pages/321|page 321]].
