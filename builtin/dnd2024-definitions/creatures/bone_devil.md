---
splat_schema: 1
splat_id: b763d8e5-bd11-55b7-8cc4-76a939ba4a6c
splat_kind: creature
splat_key: dnd2024:creatures/bone_devil
splat_name: Bone Devil
splat_origin:
  key: dnd2024:reference/creature/bone_devil
splat_sources:
- title: SRD 5.2.1
  locator: pp. 267, 268
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: large
  type: fiend
  alignment: Lawful Evil
  tags_text: (Devil)
  movement_text: 40 ft., Fly 40 ft.
  skills_text: Deception +7, Insight +6
  resistances_text: Cold
  immunities_text: Fire, Poison; Poisoned
  senses_text: Darkvision 120 ft. (unimpeded by magical Darkness); Passive Perception 12
  languages_text: Infernal; telepathy 120 ft.
  cr_text: 9 (XP 5,000; PB +4)
  hit_points_text: 161 (17d10 + 68)
splat_stats:
  strength:
    value: 18
  save_strength:
    value: 8
  dexterity:
    value: 16
  save_dexterity:
    value: 3
  constitution:
    value: 18
  save_constitution:
    value: 4
  intelligence:
    value: 13
  save_intelligence:
    value: 5
  wisdom:
    value: 14
  save_wisdom:
    value: 6
  charisma:
    value: 16
  save_charisma:
    value: 7
  armor_class:
    value: 16
  max_hp:
    value: 161
  initiative:
    value: 7
  speed:
    value: 40
    unit: ft
  fly_speed:
    value: 40
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/bone_devil
  implementation: dnd2024:creature_bone_devil
  parameters:
    skill_totals:
      deception: 7
      insight: 6
    damage_resistances:
    - cold
    damage_immunities:
    - fire
    - poison
    condition_immunities:
    - poisoned
    senses:
    - sense: darkvision
      range_ft: 120
    passive_perception: 12
    darkvision_ignores_magical_darkness: true
    languages:
      speak: true
      known:
      - infernal
    telepathy:
      range_ft: 120
      two_way: true
    features:
      diabolical_restoration:
        economy: passive
        trigger: actor.died
        when:
          plane_not: nine_hells
        body: disappears
        revive:
          new_body: true
          plane: nine_hells
          location_choice: gm
          hp: maximum
          delay:
            amount: 0
            unit: second
      magic_resistance:
        economy: passive
        op: roll.advantage
        rolls:
        - saving_throw
        against:
        - spell
        - magical_effect
      claw:
        economy: action
        op: attack
        modes:
        - melee
        attack_bonus: 8
        target:
          kind: creature_or_object
          count: 1
        reach_ft: 10
        hit:
        - op: damage
          components:
          - type: slashing
            amount:
              dice:
                count: 2
                sides: 8
              add: 4
            average: 13
      infernal_sting:
        economy: action
        op: attack
        modes:
        - melee
        attack_bonus: 8
        target:
          kind: creature_or_object
          count: 1
        reach_ft: 10
        hit:
        - op: damage
          components:
          - type: piercing
            amount:
              dice:
                count: 2
                sides: 10
              add: 4
            average: 15
          - type: poison
            amount:
              dice:
                count: 4
                sides: 8
              add: 0
            average: 18
        - op: condition.apply
          condition: poisoned
          duration:
            until: actor.next_turn_start
          while_active:
          - op: healing.prohibit
      multiattack:
        economy: action
        op: multiattack
        sequence:
        - count: 2
          feature: claw
        - count: 1
          feature: infernal_sting
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
- id: dnd2024:creature_bone_devil
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
  source_sha256: 6d5bb3bc2022f871b09072328f6e3f47871a5cb20439d5e0babc9bbd87e93bab
  source_path: builtin/srd-5.2.1/creature/bone_devil.md
  pages:
  - 267
  - 268
  engine_gaps: &id001
  - dnd2024:creature_bone_devil
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_3.md
  specification_sha256: 162247591adb1e645d751997eb63e588bcba383ae93c01e1551cf17c52f9a693
splat_clauses:
- key: source_rules
  text: '# Bone Devil Large Fiend (Devil), Lawful Evil AC 16 Initiative +7 (17) HP 161 (17d10 + 68) Speed
    40 ft., Fly 40 ft. MOD SAVE MOD SAVE MOD SAVE Str 18 +4 +8 Dex 16 +3 +3 Con 18 +4 +4 Int 13 +1 +5
    Wis 14 +2 +6 Cha 16 +3 +7 Skills Deception +7, Insight +6 Resistances Cold Immunities Fire, Poison;
    Poisoned Senses Darkvision 120 ft. (unimpeded by magical Darkness); Passive Perception 12 Languages
    Infernal; telepathy 120 ft. CR 9 (XP 5,000; PB +4) ### Traits Diabolical Restoration. If the devil
    dies outside the Nine Hells, its body disappears in sulfurous smoke, and it gains a new body instantly,
    reviving with all its Hit Points somewhere in the Nine Hells. Magic Resistance. The devil has Advantage
    on saving throws against spells and other magical effects. ### Actions Multiattack. The devil makes
    two Claw attacks and one Infernal Sting attack. Claw. Melee Attack Roll: +8, reach 10 ft. Hit: 13
    (2d8 + 4) Slashing damage. Infernal Sting. Melee Attack Roll: +8, reach 10 ft. Hit: 15 (2d10 + 4)
    Piercing damage plus 18 (4d8) Poison damage, and the target has the Poisoned condition until the start
    of the devil’s next turn. While Poisoned, the target can’t regain Hit Points.'
  sha256: 8d0952a1b0e519e3089121987a30205279936fac9cf3ea3bb86b41f68987ef23
  status: contract_specified
splat_requires_mechanics: *id001
---

# Bone Devil

<!-- SRD page 267 -->

Large Fiend (Devil), Lawful Evil\
AC 16 Initiative +7 (17)\
HP 161 (17d10 + 68)\
Speed 40 ft., Fly 40 ft.\
MOD SAVE MOD SAVE MOD SAVE\
Str 18 +4 +8 Dex 16 +3 +3 Con 18 +4 +4\
Int 13 +1 +5 Wis 14 +2 +6 Cha 16 +3 +7\
Skills Deception +7, Insight +6\
Resistances Cold\
Immunities Fire, Poison; Poisoned\
Senses Darkvision 120 ft. (unimpeded by magical\
Darkness); Passive Perception 12\
Languages Infernal; telepathy 120 ft.\
CR 9 (XP 5,000; PB +4)\

### Traits

Diabolical Restoration. If the devil dies outside the\
Nine Hells, its body disappears in sulfurous smoke, and\
it gains a new body instantly, reviving with all its Hit\
Points somewhere in the Nine Hells.\
Magic Resistance. The devil has Advantage on saving\
throws against spells and other magical effects.\

### Actions

Multiattack. The devil makes two Claw attacks and one\
Infernal Sting attack.\
Claw. Melee Attack Roll: +8, reach 10 ft. Hit: 13 (2d8\
+ 4) Slashing damage.\
Infernal Sting. Melee Attack Roll: +8, reach 10 ft. Hit:\
15 (2d10 + 4) Piercing damage plus 18 (4d8) Poison\

<!-- SRD page 268 -->

damage, and the target has the Poisoned condition until\
the start of the devil’s next turn. While Poisoned, the\
target can’t regain Hit Points.\

Original layout: [[../../srd-5.2.1/pages/267|page 267]], [[../../srd-5.2.1/pages/268|page 268]].
