---
splat_schema: 1
splat_id: 71a40ee4-b75b-5f4e-918f-cd0094b13923
splat_kind: rule
splat_key: dnd2024:class_features/rogue_level_1_sneak_attack
splat_name: 'Level 1: Sneak Attack'
splat_origin:
  key: dnd2024:reference/pages/061
splat_sources:
- title: SRD 5.2.1
  locator: pp. 61, 62
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_features/rogue_level_1_sneak_attack
  parameters:
    limit:
      count: 1
      period: turn
    trigger: actor.attack_hits_creature
    requires:
      weapon_any:
      - finesse_property
      - ranged_weapon
      any:
      - attack_advantage: true
      - attack_disadvantage: false
        ally_within_target_feet: 5
        ally_incapacitated: false
    extra_damage:
      dice:
        count:
          op: ceil
          args:
          - op: divide
            args:
            - var: actor.class_levels.rogue
            - 2
        sides: 6
      type: weapon.damage_type
    class_scope:
      class: rogue
      minimum_level: 1
  implementation: dnd2024:class_features_rogue_level_1_sneak_attack
splat_contracts:
- id: dnd2024:class_features_rogue_level_1_sneak_attack
  inputs:
    actor:
      type: participant
      required: true
    event:
      required: false
      type: event
    selection:
      required: false
      type: choice
  outputs:
  - effects
  events:
  - attack.hit
  capabilities:
  - character_features
  - damage
  - d20_tests
splat_conversion:
  status: specified
  category: class_feature
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 61
  - 62
  engine_gaps: &id001
  - dnd2024:class_features_rogue_level_1_sneak_attack
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/character_options_2.md
  specification_sha256: d7e287c7f552b79643a7d7aa0d2f1f82fa579f3553cadcab19ed70d86b60088b
  parent_name: Rogue
  line_span:
    first: 5171
    count: 37
splat_clauses:
- key: source_rules
  text: '# Level 1: Sneak Attack You know how to strike subtly and exploit a foe’s distraction. Once per
    turn, you can deal an extra 1d6 damage to one creature you hit with an attack Rogue Features Proficiency
    Level Bonus Class Features 1 +2 Expertise, Sneak Attack, Thieves’ Cant, Weapon Mastery 2 +2 Cunning
    Action 3 +2 Rogue Subclass, Steady Aim 4 +2 Ability Score Improvement 5 +3 Cunning Strike, Uncanny
    Dodge 6 +3 Expertise 7 +3 Evasion, Reliable Talent 8 +3 Ability Score Improvement 9 +4 Subclass feature
    10 +4 Ability Score Improvement 11 +4 Improved Cunning Strike 12 +4 Ability Score Improvement 13 +5
    Subclass feature 14 +5 Devious Strikes 15 +5 Slippery Mind 16 +5 Ability Score Improvement 17 +6 Subclass
    feature 18 +6 Elusive 19 +6 Epic Boon 20 +6 Stroke of Luck roll if you have Advantage on the roll
    and the attack uses a Finesse or a Ranged weapon. The extra damage’s type is the same as the weapon’s
    type. You don’t need Advantage on the attack roll if at least one of your allies is within 5 feet
    of the target, the ally doesn’t have the Incapacitated condition, and you don’t have Disadvantage
    on the attack roll. The extra damage increases as you gain Rogue levels, as shown in the Sneak Attack
    column of the Rogue Features table.'
  sha256: 46e66abcad1e5dd01f452c772cbe14432e541c9da7de5b457c2966cdf5aa8004
  status: contract_specified
splat_requires_mechanics: *id001
---

# Level 1: Sneak Attack

<!-- SRD page 61 -->

You know how to strike subtly and exploit a foe’s\
distraction. Once per turn, you can deal an extra\
1d6 damage to one creature you hit with an attack\

<!-- SRD page 62 -->

Rogue Features\
Proficiency\
Level Bonus Class Features\
1 +2 Expertise, Sneak Attack, Thieves’ Cant, Weapon Mastery\
2 +2 Cunning Action\
3 +2 Rogue Subclass, Steady Aim\
4 +2 Ability Score Improvement\
5 +3 Cunning Strike, Uncanny Dodge\
6 +3 Expertise\
7 +3 Evasion, Reliable Talent\
8 +3 Ability Score Improvement\
9 +4 Subclass feature\
10 +4 Ability Score Improvement\
11 +4 Improved Cunning Strike\
12 +4 Ability Score Improvement\
13 +5 Subclass feature\
14 +5 Devious Strikes\
15 +5 Slippery Mind\
16 +5 Ability Score Improvement\
17 +6 Subclass feature\
18 +6 Elusive\
19 +6 Epic Boon\
20 +6 Stroke of Luck\
roll if you have Advantage on the roll and the attack\
uses a Finesse or a Ranged weapon. The extra dam-\
age’s type is the same as the weapon’s type.\
You don’t need Advantage on the attack roll if at\
least one of your allies is within 5 feet of the target,\
the ally doesn’t have the Incapacitated condition,\
and you don’t have Disadvantage on the attack roll.\
The extra damage increases as you gain Rogue\
levels, as shown in the Sneak Attack column of the\
Rogue Features table.\
