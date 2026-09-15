---
splat_schema: 1
splat_id: a9727af5-4b2b-5b0e-a005-383c8eec0ac7
splat_kind: creature
splat_key: dnd2024:creatures/werebear
splat_name: Werebear
splat_origin:
  key: dnd2024:reference/creature/werebear
splat_sources:
- title: SRD 5.2.1
  locator: pp. 337, 338
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: medium
  type: monstrosity
  alignment: Neutral Good
  size_options_text: Medium or Small
  tags_text: (Lycanthrope)
  movement_text: 30 ft., 40 ft. (bear form only), Climb 30 ft. (bear form only)
  skills_text: Perception +7
  gear_text: Handaxes (4)
  senses_text: Darkvision 60 ft.; Passive Perception 17
  languages_text: Common (can’t speak in bear form)
  cr_text: 5 (XP 1,800; PB +3)
  hit_points_text: 135 (18d8 + 54)
splat_stats:
  strength:
    value: 19
  save_strength:
    value: 4
  dexterity:
    value: 10
  save_dexterity:
    value: 0
  constitution:
    value: 17
  save_constitution:
    value: 3
  intelligence:
    value: 11
  save_intelligence:
    value: 0
  wisdom:
    value: 12
  save_wisdom:
    value: 1
  charisma:
    value: 12
  save_charisma:
    value: 1
  armor_class:
    value: 15
  max_hp:
    value: 135
  initiative:
    value: 3
  speed:
    value: 40
    unit: ft
  climb_speed:
    value: 30
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/werebear
  implementation: dnd2024:creature_werebear
  parameters:
    skill_totals:
      perception: 7
    gear:
    - item: handaxes
      count: 4
    senses:
    - sense: darkvision
      range_ft: 60
    passive_perception: 17
    languages:
      speak: true
      known:
      - common
      speech_prohibited_forms:
      - bear
    features:
      bite:
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
          - type: piercing
            amount:
              dice:
                count: 2
                sides: 12
              add: 4
            average: 17
        - op: saving_throw
          ability: constitution
          dc: 14
          targets:
            kind: target
            creature_types:
            - humanoid
          failure:
          - op: curse.apply
            on_target_zero_hp:
              instead:
                op: stat_block.replace
                definition:
                  key: dnd2024:creatures/werebear
                controller: gm
                hp: 10
          success:
          - op: feature.immunity
            source: actor
            feature: bite_curse
            duration:
              amount: 24
              unit: hour
          effect_identity: bite_curse
        allowed_forms:
        - bear
        - hybrid
      handaxe:
        economy: action
        op: attack
        modes:
        - melee
        - ranged
        attack_bonus: 7
        target:
          kind: creature_or_object
          count: 1
        reach_ft: 5
        range_ft:
          normal: 20
          long: 60
        hit:
        - op: damage
          components:
          - type: slashing
            amount:
              dice:
                count: 3
                sides: 6
              add: 4
            average: 14
        allowed_forms:
        - humanoid
        - hybrid
      rend:
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
          - type: slashing
            amount:
              dice:
                count: 2
                sides: 8
              add: 4
            average: 13
        allowed_forms:
        - bear
        - hybrid
      shape_shift:
        economy: bonus_action
        op: shapechange
        equipment_transformed: false
        statistics_preserved_except:
        - size
        choices:
        - form: hybrid
          size: large
          animal: bear
        - form: bear
          size: large
        - form: true_humanoid
      multiattack:
        economy: action
        op: multiattack
        attacks:
          count: 2
          choices:
          - handaxe
          - rend
          combination: any
        replace_attack:
          maximum: 1
          choices:
          - op: feature.use
            feature: bite
    form_overrides:
      bear:
        speed_ft: 40
        climb_speed_ft: 30
        can_speak: false
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
- id: dnd2024:creature_werebear
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
  - creature_lifecycle
  - creature_rules
  - damage
  - effects
  - saving_throws
  - spatial_constraints
splat_conversion:
  status: specified
  category: creature
  source_sha256: 145c64c32d94a02cd8ccbdffcb114353e9f44b6fb3492da7c81ff38fffd7676a
  source_path: builtin/srd-5.2.1/creature/werebear.md
  pages:
  - 337
  - 338
  engine_gaps: &id001
  - dnd2024:creature_werebear
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_3.md
  specification_sha256: d6419230b768628069fe2d67859f9e7b53d86d692cb4456b38c5a2a26d7c331d
splat_clauses:
- key: source_rules
  text: '# Werebear Medium or Small Monstrosity (Lycanthrope), Neutral Good AC 15 Initiative +3 (13) HP
    135 (18d8 + 54) Speed 30 ft., 40 ft. (bear form only), Climb 30 ft. (bear form only) MOD SAVE MOD
    SAVE MOD SAVE Str 19 +4 +4 Dex 10 +0 +0 Con 17 +3 +3 Int 11 +0 +0 Wis 12 +1 +1 Cha 12 +1 +1 Skills
    Perception +7 Gear Handaxes (4) Senses Darkvision 60 ft.; Passive Perception 17 Languages Common (can’t
    speak in bear form) CR 5 (XP 1,800; PB +3) ### Actions Multiattack. The werebear makes two attacks,
    using Handaxe or Rend in any combination. It can replace one attack with a Bite attack. Bite (Bear
    or Hybrid Form Only). Melee Attack Roll: +7, reach 5 ft. Hit: 17 (2d12 + 4) Piercing damage. If the
    target is a Humanoid, it is subjected to the following effect. Constitution Saving Throw: DC 14. Failure:
    The target is cursed. If the cursed target drops to 0 Hit Points, it instead becomes a Werebear under
    the GM’s control and has 10 Hit Points. Success: The target is immune to this werebear’s curse for
    24 hours. Handaxe (Humanoid or Hybrid Form Only). Melee or Ranged Attack Roll: +7, reach 5 ft or range
    20/60 ft. Hit: 14 (3d6 + 4) Slashing damage. Rend (Bear or Hybrid Form Only). Melee Attack Roll: +7,
    reach 5 ft. Hit: 13 (2d8 + 4) Slashing damage. ### Bonus Actions Shape-Shift. The werebear shape-shifts
    into a Large bear-humanoid hybrid form or a Large bear, or it returns to its true humanoid form. Its
    game statistics, other than its size, are the same in each form. Any equipment it is wearing or carrying
    isn’t transformed.'
  sha256: e3547a7d55956cfa637d384fea72ad5ad465f57fcda02472e683f20e256a5292
  status: contract_specified
splat_requires_mechanics: *id001
---

# Werebear

<!-- SRD page 337 -->

Medium or Small Monstrosity (Lycanthrope), Neutral Good\
AC 15 Initiative +3 (13)\
HP 135 (18d8 + 54)\
Speed 30 ft., 40 ft. (bear form only), Climb 30 ft. (bear\
form only)\
MOD SAVE MOD SAVE MOD SAVE\
Str 19 +4 +4 Dex 10 +0 +0 Con 17 +3 +3\
Int 11 +0 +0 Wis 12 +1 +1 Cha 12 +1 +1\
Skills Perception +7\
Gear Handaxes (4)\
Senses Darkvision 60 ft.; Passive Perception 17\
Languages Common (can’t speak in bear form)\
CR 5 (XP 1,800; PB +3)\

### Actions

Multiattack. The werebear makes two attacks, using\
Handaxe or Rend in any combination. It can replace\
one attack with a Bite attack.\
Bite (Bear or Hybrid Form Only). Melee Attack Roll:\
+7, reach 5 ft. Hit: 17 (2d12 + 4) Piercing damage. If\
the target is a Humanoid, it is subjected to the follow-\
ing effect. Constitution Saving Throw: DC 14. Failure:\
The target is cursed. If the cursed target drops to 0 Hit\
Points, it instead becomes a Werebear under the GM’s\

<!-- SRD page 338 -->

control and has 10 Hit Points. Success: The target is im-\
mune to this werebear’s curse for 24 hours.\
Handaxe (Humanoid or Hybrid Form Only). Melee\
or Ranged Attack Roll: +7, reach 5 ft or range 20/60 ft.\
Hit: 14 (3d6 + 4) Slashing damage.\
Rend (Bear or Hybrid Form Only). Melee Attack Roll:\
+7, reach 5 ft. Hit: 13 (2d8 + 4) Slashing damage.\

### Bonus Actions

Shape-Shift. The werebear shape-shifts into a Large\
bear-humanoid hybrid form or a Large bear, or it re-\
turns to its true humanoid form. Its game statistics,\
other than its size, are the same in each form. Any\
equipment it is wearing or carrying isn’t transformed.\

Original layout: [[../../srd-5.2.1/pages/337|page 337]], [[../../srd-5.2.1/pages/338|page 338]].
