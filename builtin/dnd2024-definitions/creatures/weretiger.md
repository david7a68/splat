---
splat_schema: 1
splat_id: 0da660ab-67e1-52d2-a3f6-5656c40c6fd8
splat_kind: creature
splat_key: dnd2024:creatures/weretiger
splat_name: Weretiger
splat_origin:
  key: dnd2024:reference/creature/weretiger
splat_sources:
- title: SRD 5.2.1
  locator: pp. 339
  url: https://www.dndbeyond.com/srd
splat_creature:
  size: medium
  type: monstrosity
  alignment: Neutral
  size_options_text: Medium or Small
  tags_text: (Lycanthrope)
  movement_text: 30 ft., 40 ft. (tiger form only)
  skills_text: Perception +5, Stealth +4
  gear_text: Longbow
  senses_text: Darkvision 60 ft.; Passive Perception 15
  languages_text: Common (can’t speak in tiger form)
  cr_text: 4 (XP 1,100; PB +2)
  hit_points_text: 120 (16d8 + 48)
splat_stats:
  strength:
    value: 17
  save_strength:
    value: 3
  dexterity:
    value: 15
  save_dexterity:
    value: 2
  constitution:
    value: 16
  save_constitution:
    value: 3
  intelligence:
    value: 10
  save_intelligence:
    value: 0
  wisdom:
    value: 13
  save_wisdom:
    value: 1
  charisma:
    value: 11
  save_charisma:
    value: 0
  armor_class:
    value: 12
  max_hp:
    value: 120
  initiative:
    value: 2
  speed:
    value: 40
    unit: ft
splat_health:
  zero_hp: dnd2024:monster
splat_rule:
  mechanic: dnd2024:creatures/weretiger
  implementation: dnd2024:creature_weretiger
  parameters:
    skill_totals:
      perception: 5
      stealth: 4
    gear:
    - item: longbow
      count: 1
    senses:
    - sense: darkvision
      range_ft: 60
    passive_perception: 15
    languages:
      speak: true
      known:
      - common
      speech_prohibited_forms:
      - tiger
    features:
      bite:
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
          - type: piercing
            amount:
              dice:
                count: 2
                sides: 8
              add: 3
            average: 12
        - op: saving_throw
          ability: constitution
          dc: 13
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
                  key: dnd2024:creatures/weretiger
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
        - tiger
        - hybrid
      scratch:
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
                count: 2
                sides: 6
              add: 3
            average: 10
      longbow:
        economy: action
        op: attack
        modes:
        - ranged
        attack_bonus: 4
        target:
          kind: creature_or_object
          count: 1
        range_ft:
          normal: 150
          long: 600
        hit:
        - op: damage
          components:
          - type: piercing
            amount:
              dice:
                count: 2
                sides: 8
              add: 2
            average: 11
        allowed_forms:
        - humanoid
        - hybrid
      prowl:
        economy: bonus_action
        allowed_forms:
        - tiger
        - hybrid
        op: sequence
        steps:
        - op: movement
          maximum:
            multiply:
            - 1
            - actor_stat: speed
          opportunity_attacks: false
        - op: action.use
          action: hide
          optional: true
      shape_shift:
        economy: bonus_action
        op: shapechange
        equipment_transformed: false
        statistics_preserved_except:
        - size
        choices:
        - form: hybrid
          size: large
          animal: tiger
        - form: tiger
          size: large
        - form: true_humanoid
      multiattack:
        economy: action
        op: multiattack
        attacks:
          count: 2
          choices:
          - scratch
          - longbow
          combination: any
        replace_attack:
          maximum: 1
          choices:
          - op: feature.use
            feature: bite
    form_overrides:
      tiger:
        speed_ft: 40
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
- id: dnd2024:creature_weretiger
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
  source_sha256: 409eaa6049ba08a550573e8ac3b7713c5f87769f0cdaf47ecf7083c5f16adcd6
  source_path: builtin/srd-5.2.1/creature/weretiger.md
  pages:
  - 339
  engine_gaps: &id001
  - dnd2024:creature_weretiger
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/creatures_3.md
  specification_sha256: 373e2307aa984a671cc8d1021ffcae7ec4541b0ccf8f7174baa8d2dbf6aafd4f
splat_clauses:
- key: source_rules
  text: '# Weretiger Medium or Small Monstrosity (Lycanthrope), Neutral AC 12 Initiative +2 (12) HP 120
    (16d8 + 48) Speed 30 ft., 40 ft. (tiger form only) MOD SAVE MOD SAVE MOD SAVE Str 17 +3 +3 Dex 15
    +2 +2 Con 16 +3 +3 Int 10 +0 +0 Wis 13 +1 +1 Cha 11 +0 +0 Skills Perception +5, Stealth +4 Gear Longbow
    Senses Darkvision 60 ft.; Passive Perception 15 Languages Common (can’t speak in tiger form) CR 4
    (XP 1,100; PB +2) ### Actions Multiattack. The weretiger makes two attacks, using Scratch or Longbow
    in any combination. It can replace one attack with a Bite attack. Bite (Tiger or Hybrid Form Only).
    Melee Attack Roll: +5, reach 5 ft. Hit: 12 (2d8 + 3) Piercing damage. If the target is a Humanoid,
    it is subjected to the following effect. Constitution Saving Throw: DC 13. Failure: The target is
    cursed. If the cursed target drops to 0 Hit Points, it instead becomes a Weretiger under the GM’s
    control and has 10 Hit Points. Success: The target is immune to this weretiger’s curse for 24 hours.
    Scratch. Melee Attack Roll: +5, reach 5 ft. Hit: 10 (2d6 + 3) Slashing damage. Longbow (Humanoid or
    Hybrid Form Only). Ranged Attack Roll: +4, range 150/600 ft. Hit: 11 (2d8 + 2) Piercing damage. ###
    Bonus Actions Prowl (Tiger or Hybrid Form Only). The weretiger moves up to its Speed without provoking
    Opportunity Attacks. At the end of this movement, the weretiger can take the Hide action. Shape-Shift.
    The weretiger shape-shifts into a Large tiger-humanoid hybrid or a Large tiger, or it returns to its
    true humanoid form. Its game statistics, other than its size, are the same in each form. Any equipment
    it is wearing or carrying isn’t transformed.'
  sha256: 4bbde9425c5227fa9fbdbd85e474ba5537457e65f89de216828eab9f8156fb13
  status: contract_specified
splat_requires_mechanics: *id001
---

# Weretiger

<!-- SRD page 339 -->

Medium or Small Monstrosity (Lycanthrope), Neutral\
AC 12 Initiative +2 (12)\
HP 120 (16d8 + 48)\
Speed 30 ft., 40 ft. (tiger form only)\
MOD SAVE MOD SAVE MOD SAVE\
Str 17 +3 +3 Dex 15 +2 +2 Con 16 +3 +3\
Int 10 +0 +0 Wis 13 +1 +1 Cha 11 +0 +0\
Skills Perception +5, Stealth +4\
Gear Longbow\
Senses Darkvision 60 ft.; Passive Perception 15\
Languages Common (can’t speak in tiger form)\
CR 4 (XP 1,100; PB +2)\

### Actions

Multiattack. The weretiger makes two attacks, using\
Scratch or Longbow in any combination. It can replace\
one attack with a Bite attack.\
Bite (Tiger or Hybrid Form Only). Melee Attack Roll:\
+5, reach 5 ft. Hit: 12 (2d8 + 3) Piercing damage. If\
the target is a Humanoid, it is subjected to the follow-\
ing effect. Constitution Saving Throw: DC 13. Failure:\
The target is cursed. If the cursed target drops to 0 Hit\
Points, it instead becomes a Weretiger under the GM’s\
control and has 10 Hit Points. Success: The target is im-\
mune to this weretiger’s curse for 24 hours.\
Scratch. Melee Attack Roll: +5, reach 5 ft. Hit: 10 (2d6\
+ 3) Slashing damage.\
Longbow (Humanoid or Hybrid Form Only). Ranged\
Attack Roll: +4, range 150/600 ft. Hit: 11 (2d8 + 2)\
Piercing damage.\

### Bonus Actions

Prowl (Tiger or Hybrid Form Only). The weretiger\
moves up to its Speed without provoking Opportunity\
Attacks. At the end of this movement, the weretiger can\
take the Hide action.\
Shape-Shift. The weretiger shape-shifts into a Large\
tiger-humanoid hybrid or a Large tiger, or it returns to\
its true humanoid form. Its game statistics, other than\
its size, are the same in each form. Any equipment it is\
wearing or carrying isn’t transformed.\

Original layout: [[../../srd-5.2.1/pages/339|page 339]].
