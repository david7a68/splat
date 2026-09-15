---
splat_schema: 1
splat_id: cf229e93-690c-5692-a3b5-ed157c48597e
splat_kind: creature
splat_key: dnd2024:creatures/werewolf
splat_name: Werewolf
splat_origin:
  key: dnd2024:reference/creature/werewolf
splat_sources:
- title: SRD 5.2.1
  locator: pp. 339
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: medium
  type: monstrosity
  alignment: Chaotic Evil
  size_options_text: Medium or Small
  tags_text: (Lycanthrope)
  movement_text: 30 ft., 40 ft. (wolf form only)
  skills_text: Perception +4, Stealth +4
  gear_text: Longbow
  senses_text: Darkvision 60 ft.; Passive Perception 14
  languages_text: Common (can’t speak in wolf form)
  cr_text: 3 (XP 700; PB +2)
  hit_points_text: 71 (11d8 + 22)
splat_stats:
  strength:
    value: 16
  save_strength:
    value: 3
  dexterity:
    value: 14
  save_dexterity:
    value: 2
  constitution:
    value: 14
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
    value: 10
  save_charisma:
    value: 0
  armor_class:
    value: 15
  max_hp:
    value: 71
  initiative:
    value: 4
  speed:
    value: 40
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/werewolf
  implementation: dnd2024:creature_werewolf
  parameters:
    features:
      pack_tactics:
        economy: trait
        operation: passive_rule
        attack_advantage:
          target_kind: creature
          ally_within_target_ft: 5
          ally_excludes_conditions:
          - incapacitated
      multiattack:
        economy: action
        operation: multiattack
        sequence:
        - operation: attack_choice
          count: 2
          options:
          - scratch
          - longbow
        replacements:
        - replace_count: 1
          maximum_uses: 1
          operation:
            operation: use_feature
            feature: bite
      bite:
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
                      key: dnd2024:creatures/werewolf
                    controller: gm
                    hp: 10
            success:
              immunity:
                effect: this_creature_curse
                duration_seconds: 86400
        allowed_forms:
        - wolf
        - hybrid
      scratch:
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
            type: slashing
      longbow:
        economy: action
        operation: attack
        attack:
          mode: ranged
          targets:
            count: 1
            kind: creature_or_object
          supplied_attack_total: 4
          range_ft: 150
          long_range_ft: 600
        hit:
          damage:
          - average: 11
            roll:
              count: 2
              sides: 8
              modifier: 2
            type: piercing
        allowed_forms:
        - humanoid
        - hybrid
      shape_shift:
        economy: bonus_action
        operation: shape_shift
        forms:
        - form: hybrid
          size: large
        - form: wolf
          size: medium
        - form: true_humanoid
          size: original
        statistics_preserved_except:
        - size
        - form_specific_speed
        equipment_transformed: false
    skill_totals:
      perception: 4
      stealth: 4
    gear:
    - item: longbow
      quantity: 1
    senses:
      darkvision: 60
      passive_perception: 14
    languages:
      speaks:
      - common
      understands: []
      cannot_speak_forms:
      - wolf
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
- id: dnd2024:creature_werewolf
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
  source_sha256: 7bff0bd13d170bee77b6727f6ead43d9152c53dfd574daf4d533b7978d8af005
  source_path: builtin/srd-5.2.1/creature/werewolf.md
  pages:
  - 339
  engine_gaps: &id001
  - dnd2024:creature_werewolf
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_1.md
  specification_sha256: 53c93e3cae79557872af5911d1da50ed971207b102e589bc1f008e6f557df946
splat_clauses:
- key: source_rules
  text: '# Werewolf Medium or Small Monstrosity (Lycanthrope), Chaotic Evil AC 15 Initiative +4 (14) HP
    71 (11d8 + 22) Speed 30 ft., 40 ft. (wolf form only) MOD SAVE MOD SAVE MOD SAVE Str 16 +3 +3 Dex 14
    +2 +2 Con 14 +2 +2 Int 10 +0 +0 Wis 11 +0 +0 Cha 10 +0 +0 Skills Perception +4, Stealth +4 Gear Longbow
    Senses Darkvision 60 ft.; Passive Perception 14 Languages Common (can’t speak in wolf form) CR 3 (XP
    700; PB +2) ### Traits Pack Tactics. The werewolf has Advantage on an attack roll against a creature
    if at least one of the werewolf’s allies is within 5 feet of the creature and the ally doesn’t have
    the Incapacitated condition. ### Actions Multiattack. The werewolf makes two attacks, using Scratch
    or Longbow in any combination. It can replace one attack with a Bite attack. Bite (Wolf or Hybrid
    Form Only). Melee Attack Roll: +5, reach 5 ft. Hit: 12 (2d8 + 3) Piercing damage. If the target is
    a Humanoid, it is subjected to the following effect. Constitution Saving Throw: DC 12. Failure: The
    target is cursed. If the cursed target drops to 0 Hit Points, it instead becomes a Werewolf under
    the GM’s control and has 10 Hit Points. Success: The target is immune to this werewolf’s curse for
    24 hours. Scratch. Melee Attack Roll: +5, reach 5 ft. Hit: 10 (2d6 + 3) Slashing damage. Longbow (Humanoid
    or Hybrid Form Only). Ranged Attack Roll: +4, range 150/600 ft. Hit: 11 (2d8 + 2) Piercing damage.
    ### Bonus Actions Shape-Shift. The werewolf shape-shifts into a Large wolf-humanoid hybrid or a Medium
    wolf, or it returns to its true humanoid form. Its game statistics, other than its size, are the same
    in each form. Any equipment it is wearing or carrying isn’t transformed.'
  sha256: e5de839fa59e239afad0d02b2af9a9bd3823ad2cd8ab214adfb944d8a4f6142c
  status: contract_specified
splat_requires_mechanics: *id001
---

# Werewolf

<!-- SRD page 339 -->

Medium or Small Monstrosity (Lycanthrope), Chaotic Evil\
AC 15 Initiative +4 (14)\
HP 71 (11d8 + 22)\
Speed 30 ft., 40 ft. (wolf form only)\
MOD SAVE MOD SAVE MOD SAVE\
Str 16 +3 +3 Dex 14 +2 +2 Con 14 +2 +2\
Int 10 +0 +0 Wis 11 +0 +0 Cha 10 +0 +0\
Skills Perception +4, Stealth +4\
Gear Longbow\
Senses Darkvision 60 ft.; Passive Perception 14\
Languages Common (can’t speak in wolf form)\
CR 3 (XP 700; PB +2)\

### Traits

Pack Tactics. The werewolf has Advantage on an attack\
roll against a creature if at least one of the werewolf’s\
allies is within 5 feet of the creature and the ally doesn’t\
have the Incapacitated condition.\

### Actions

Multiattack. The werewolf makes two attacks, using\
Scratch or Longbow in any combination. It can replace\
one attack with a Bite attack.\
Bite (Wolf or Hybrid Form Only). Melee Attack Roll:\
+5, reach 5 ft. Hit: 12 (2d8 + 3) Piercing damage. If\
the target is a Humanoid, it is subjected to the follow-\
ing effect. Constitution Saving Throw: DC 12. Failure:\
The target is cursed. If the cursed target drops to 0 Hit\
Points, it instead becomes a Werewolf under the GM’s\
control and has 10 Hit Points. Success: The target is im-\
mune to this werewolf’s curse for 24 hours.\
Scratch. Melee Attack Roll: +5, reach 5 ft. Hit: 10 (2d6\
+ 3) Slashing damage.\
Longbow (Humanoid or Hybrid Form Only). Ranged\
Attack Roll: +4, range 150/600 ft. Hit: 11 (2d8 + 2)\
Piercing damage.\

### Bonus Actions

Shape-Shift. The werewolf shape-shifts into a Large\
wolf-humanoid hybrid or a Medium wolf, or it returns\
to its true humanoid form. Its game statistics, other than\
its size, are the same in each form. Any equipment it is\
wearing or carrying isn’t transformed.\

Original layout: [[../../srd-5.2.1/pages/339|page 339]].
