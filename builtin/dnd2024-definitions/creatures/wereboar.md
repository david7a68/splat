---
splat_schema: 1
splat_id: 57bf9535-1847-5e11-b533-93abe71c4e10
splat_kind: creature
splat_key: dnd2024:creatures/wereboar
splat_name: Wereboar
splat_origin:
  key: dnd2024:reference/creature/wereboar
splat_sources:
- title: SRD 5.2.1
  locator: pp. 338
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: medium
  type: monstrosity
  alignment: Neutral Evil
  size_options_text: Medium or Small
  tags_text: (Lycanthrope)
  movement_text: 30 ft., 40 ft. (boar form only)
  skills_text: Perception +2
  gear_text: Javelins (6)
  senses_text: Passive Perception 12
  languages_text: Common (can’t speak in boar form)
  cr_text: 4 (XP 1,100; PB +2)
  hit_points_text: 97 (15d8 + 30)
splat_stats:
  strength:
    value: 17
  save_strength:
    value: 3
  dexterity:
    value: 10
  save_dexterity:
    value: 0
  constitution:
    value: 15
  save_constitution:
    value: 2
  intelligence:
    value: 10
  save_intelligence:
    value: 0
  wisdom:
    value: 11
  save_wisdom:
    value: 0
  charisma:
    value: 8
  save_charisma:
    value: -1
  armor_class:
    value: 15
  max_hp:
    value: 97
  initiative:
    value: 2
  speed:
    value: 40
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/wereboar
  implementation: dnd2024:creature_wereboar
  parameters:
    features:
      multiattack:
        economy: action
        operation: multiattack
        sequence:
        - operation: attack_choice
          count: 2
          options:
          - javelin
          - tusk
        replacements:
        - replace_count: 1
          maximum_uses: 1
          operation:
            operation: use_feature
            feature: gore
      gore:
        economy: action
        operation: attack
        attack:
          mode: melee
          targets:
            count: 1
            kind: creature_or_object
          supplied_attack_total: 5
          reach_ft: 5
        hit:
          damage:
          - average: 12
            roll:
              count: 2
              sides: 8
              modifier: 3
            type: piercing
          save:
            operation: saving_throw
            ability: constitution
            dc: 12
            targets:
              kind: humanoid
            failure:
              curse:
                on_target_zero_hp:
                  replace:
                    definition:
                      key: dnd2024:creatures/wereboar
                    controller: gm
                    hp: 10
            success:
              immunity:
                effect: this_creature_curse
                duration_seconds: 86400
        allowed_forms:
        - boar
        - hybrid
      javelin:
        economy: action
        operation: attack
        attack:
          mode: melee_or_ranged
          targets:
            count: 1
            kind: creature_or_object
          supplied_attack_total: 5
          reach_ft: 5
          range_ft: 30
          long_range_ft: 120
        hit:
          damage:
          - average: 13
            roll:
              count: 3
              sides: 6
              modifier: 3
            type: piercing
        allowed_forms:
        - humanoid
        - hybrid
      tusk:
        economy: action
        operation: attack
        attack:
          mode: melee
          targets:
            count: 1
            kind: creature_or_object
          supplied_attack_total: 5
          reach_ft: 5
        hit:
          damage:
          - average: 10
            roll:
              count: 2
              sides: 6
              modifier: 3
            type: piercing
          prone:
            condition: prone
            maximum_size: medium
            requires:
              immediately_preceding_straight_movement_toward_target_ft: 20
          conditional_damage:
            requires:
              maximum_target_size: medium
              immediately_preceding_straight_movement_toward_target_ft: 20
            damage:
            - roll:
                count: 2
                sides: 6
                modifier: 0
              type: piercing
        allowed_forms:
        - boar
        - hybrid
      shape_shift:
        economy: bonus_action
        operation: shape_shift
        forms:
        - form: hybrid
          size: medium
        - form: boar
          size: small
        - form: true_humanoid
          size: original
        statistics_preserved_except:
        - size
        - form_specific_speed
        equipment_transformed: false
    skill_totals:
      perception: 2
    gear:
    - item: javelins
      quantity: 6
    senses:
      passive_perception: 12
    languages:
      speaks:
      - common
      understands: []
      cannot_speak_forms:
      - boar
    form_rules:
      beast:
        speed_ft: 40
        can_speak: false
      true_humanoid:
        speed_ft: 30
      hybrid:
        speed_ft: 30
    resolution_conventions:
      distance_unit: foot
      duration_unit: second
      damage_roll_mode:
        choice:
        - listed_average
        - roll_dice
        chooser: gm
      supplied_totals: before_additional_effect_modifiers
      multiattack_nested_economy_cost: 0
      nested_feature_resource_cost: normal
      spell_level_unless_specified: lowest_possible
      spell_upcast_without_permission: false
      spell_casting_time_at_least_one_minute: normal_casting_procedure
      geometry:
        input: adjudication
        assumptions:
        - range_and_visibility_verified
        - area_membership_verified
        - path_and_destination_legal
      unlisted_success_branch: no_effect
      unlisted_duration: until_removed_by_applicable_rule
splat_contracts:
- id: dnd2024:creature_wereboar
  inputs:
    actor:
      type: participant
      required: true
    feature:
      required: false
      type: choice
    targets:
      required: false
      type: participant_list
    objects:
      required: false
      type: object_list
    destination:
      required: false
      type: location
    adjudication:
      required: false
      type: resolution
    trigger:
      required: false
      type: event
  outputs:
  - effects
  events:
  - creature.feature_requested
  capabilities:
  - creature_features
  - action_economy
  - attacks
  - saving_throws
  - conditions
  - damage
  - shape_shifting
  - spatial_queries
  - limited_resources
  - curses
  - hit_points
  - defenses
splat_conversion:
  status: specified
  category: creature
  source_sha256: 67f6698d2c8b31ad886b494de748ffd5ff41562a37a6b1a96fa05e407370299a
  source_path: builtin/srd-5.2.1/creature/wereboar.md
  pages:
  - 338
  engine_gaps: &id001
  - dnd2024:creature_wereboar
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_1.md
  specification_sha256: 767354309995f44312098603f67b6ca1e21d10f01366db7daa174e741a55f7d1
splat_clauses:
- key: source_rules
  text: '# Wereboar Medium or Small Monstrosity (Lycanthrope), Neutral Evil AC 15 Initiative +2 (12) HP
    97 (15d8 + 30) Speed 30 ft., 40 ft. (boar form only) MOD SAVE MOD SAVE MOD SAVE Str 17 +3 +3 Dex 10
    +0 +0 Con15 +2 +2 Int 10 +0 +0 Wis 11 +0 +0 Cha 8 -1 -1 Skills Perception +2 Gear Javelins (6) Senses
    Passive Perception 12 Languages Common (can’t speak in boar form) CR 4 (XP 1,100; PB +2) ### Actions
    Multiattack. The wereboar makes two attacks, using Javelin or Tusk in any combination. It can replace
    one attack with a Gore attack. Gore (Boar or Hybrid Form Only). Melee Attack Roll: +5, reach 5 ft.
    Hit: 12 (2d8 + 3) Piercing damage. If the target is a Humanoid, it is subjected to the following effect.
    Constitution Saving Throw: DC 12. Failure: The target is cursed. If the cursed target drops to 0 Hit
    Points, it instead becomes a Wereboar under the GM’s control and has 10 Hit Points. Success: The target
    is immune to this wereboar’s curse for 24 hours. Javelin (Humanoid or Hybrid Form Only). Melee or
    Ranged Attack Roll: +5, reach 5 ft. or range 30/120 ft. Hit: 13 (3d6 + 3) Piercing damage. Tusk (Boar
    or Hybrid Form Only). Melee Attack Roll: +5, reach 5 ft. Hit: 10 (2d6 + 3) Piercing damage. If the
    target is a Medium or smaller creature and the wereboar moved 20+ feet straight toward it immediately
    before the hit, the target takes an extra 7 (2d6) Piercing damage and has the Prone condition. ###
    Bonus Actions Shape-Shift. The wereboar shape-shifts into a Medium boar-humanoid hybrid or a Small
    boar, or it returns to its true humanoid form. Its game statistics, other than its size, are the same
    in each form. Any equipment it is wearing or carrying isn’t transformed.'
  sha256: a56075556c8987531996e37394f0c9240e22d4143db13cfb99a1d8f34321f266
  status: contract_specified
splat_requires_mechanics: *id001
---

# Wereboar

<!-- SRD page 338 -->

Medium or Small Monstrosity (Lycanthrope), Neutral Evil\
AC 15 Initiative +2 (12)\
HP 97 (15d8 + 30)\
Speed 30 ft., 40 ft. (boar form only)\
MOD SAVE MOD SAVE MOD SAVE\
Str 17 +3 +3 Dex 10 +0 +0 Con15 +2 +2\
Int 10 +0 +0 Wis 11 +0 +0 Cha 8 −1 −1\
Skills Perception +2\
Gear Javelins (6)\
Senses Passive Perception 12\
Languages Common (can’t speak in boar form)\
CR 4 (XP 1,100; PB +2)\

### Actions

Multiattack. The wereboar makes two attacks, using\
Javelin or Tusk in any combination. It can replace one\
attack with a Gore attack.\
Gore (Boar or Hybrid Form Only). Melee Attack Roll:\
+5, reach 5 ft. Hit: 12 (2d8 + 3) Piercing damage. If\
the target is a Humanoid, it is subjected to the follow-\
ing effect. Constitution Saving Throw: DC 12. Failure:\
The target is cursed. If the cursed target drops to 0 Hit\
Points, it instead becomes a Wereboar under the GM’s\
control and has 10 Hit Points. Success: The target is im-\
mune to this wereboar’s curse for 24 hours.\
Javelin (Humanoid or Hybrid Form Only). Melee or\
Ranged Attack Roll: +5, reach 5 ft. or range 30/120 ft.\
Hit: 13 (3d6 + 3) Piercing damage.\
Tusk (Boar or Hybrid Form Only). Melee Attack Roll:\
+5, reach 5 ft. Hit: 10 (2d6 + 3) Piercing damage. If the\
target is a Medium or smaller creature and the were-\
boar moved 20+ feet straight toward it immediately\
before the hit, the target takes an extra 7 (2d6) Piercing\
damage and has the Prone condition.\

### Bonus Actions

Shape-Shift. The wereboar shape-shifts into a Medium\
boar-humanoid hybrid or a Small boar, or it returns to\
its true humanoid form. Its game statistics, other than\
its size, are the same in each form. Any equipment it is\
wearing or carrying isn’t transformed.\

Original layout: [[../../srd-5.2.1/pages/338|page 338]].
