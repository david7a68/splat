---
splat_schema: 1
splat_id: 947560e8-cc55-5932-85a3-4cb7a9603d9b
splat_kind: rule
splat_key: dnd2024:tables/barbarian_progression
splat_name: Barbarian Progression
splat_origin:
  key: dnd2024:reference/pages/028
splat_sources:
- title: SRD 5.2.1
  locator: pp. 28
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_progression
  parameters:
    class: barbarian
    rows:
    - level: 1
      proficiency_bonus: 2
      features_text: Rage, Unarmored Defense, Weapon Mastery
      rages: 2
      rage_damage: 2
      weapon_mastery: 2
    - level: 2
      proficiency_bonus: 2
      features_text: Danger Sense, Reckless Attack
      rages: 2
      rage_damage: 2
      weapon_mastery: 2
    - level: 3
      proficiency_bonus: 2
      features_text: Barbarian Subclass, Primal Knowledge
      rages: 3
      rage_damage: 2
      weapon_mastery: 2
    - level: 4
      proficiency_bonus: 2
      features_text: Ability Score Improvement
      rages: 3
      rage_damage: 2
      weapon_mastery: 3
    - level: 5
      proficiency_bonus: 3
      features_text: Extra Attack, Fast Movement
      rages: 3
      rage_damage: 2
      weapon_mastery: 3
    - level: 6
      proficiency_bonus: 3
      features_text: Subclass feature
      rages: 4
      rage_damage: 2
      weapon_mastery: 3
    - level: 7
      proficiency_bonus: 3
      features_text: Feral Instinct, Instinctive Pounce
      rages: 4
      rage_damage: 2
      weapon_mastery: 3
    - level: 8
      proficiency_bonus: 3
      features_text: Ability Score Improvement
      rages: 4
      rage_damage: 2
      weapon_mastery: 3
    - level: 9
      proficiency_bonus: 4
      features_text: Brutal Strike
      rages: 4
      rage_damage: 3
      weapon_mastery: 3
    - level: 10
      proficiency_bonus: 4
      features_text: Subclass feature
      rages: 4
      rage_damage: 3
      weapon_mastery: 4
    - level: 11
      proficiency_bonus: 4
      features_text: Relentless Rage
      rages: 4
      rage_damage: 3
      weapon_mastery: 4
    - level: 12
      proficiency_bonus: 4
      features_text: Ability Score Improvement
      rages: 5
      rage_damage: 3
      weapon_mastery: 4
    - level: 13
      proficiency_bonus: 5
      features_text: Improved Brutal Strike
      rages: 5
      rage_damage: 3
      weapon_mastery: 4
    - level: 14
      proficiency_bonus: 5
      features_text: Subclass feature
      rages: 5
      rage_damage: 3
      weapon_mastery: 4
    - level: 15
      proficiency_bonus: 5
      features_text: Persistent Rage
      rages: 5
      rage_damage: 3
      weapon_mastery: 4
    - level: 16
      proficiency_bonus: 5
      features_text: Ability Score Improvement
      rages: 5
      rage_damage: 4
      weapon_mastery: 4
    - level: 17
      proficiency_bonus: 6
      features_text: Improved Brutal Strike
      rages: 6
      rage_damage: 4
      weapon_mastery: 4
    - level: 18
      proficiency_bonus: 6
      features_text: Indomitable Might
      rages: 6
      rage_damage: 4
      weapon_mastery: 4
    - level: 19
      proficiency_bonus: 6
      features_text: Epic Boon
      rages: 6
      rage_damage: 4
      weapon_mastery: 4
    - level: 20
      proficiency_bonus: 6
      features_text: Primal Champion
      rages: 6
      rage_damage: 4
      weapon_mastery: 4
splat_conversion:
  status: partial
  category: table
  source_sha256: 4456eaa81743ed341c13f19d1d7ced344a6adcbb15367941875ee45cd3f2e562
  source_path: builtin/srd-5.2.1/source.txt
  pages:
  - 28
  engine_gaps: []
  conversion_gaps:
  - source_rules
  execution_profiles: []
splat_clauses:
- key: source_rules
  text: '# Barbarian Progression From SRD 5.2.1 page 28. Barbarian Features Proficiency Rage Weapon Level
    Bonus Class Features Rages Damage Mastery 1 +2 Rage, Unarmored Defense, Weapon Mastery 2 +2 2 2 +2
    Danger Sense, Reckless Attack 2 +2 2 3 +2 Barbarian Subclass, Primal Knowledge 3 +2 2 4 +2 Ability
    Score Improvement 3 +2 3 5 +3 Extra Attack, Fast Movement 3 +2 3 6 +3 Subclass feature 4 +2 3 7 +3
    Feral Instinct, Instinctive Pounce 4 +2 3 8 +3 Ability Score Improvement 4 +2 3 9 +4 Brutal Strike
    4 +3 3 10 +4 Subclass feature 4 +3 4 11 +4 Relentless Rage 4 +3 4 12 +4 Ability Score Improvement
    5 +3 4 13 +5 Improved Brutal Strike 5 +3 4 14 +5 Subclass feature 5 +3 4 15 +5 Persistent Rage 5 +3
    4 16 +5 Ability Score Improvement 5 +4 4 17 +6 Improved Brutal Strike 6 +4 4 18 +6 Indomitable Might
    6 +4 4 19 +6 Epic Boon 6 +4 4 20 +6 Primal Champion 6 +4 4 28 System Reference Document 5.2.1'
  sha256: e8971fa4d787440d771018abff19b2018903ec4be504788a54d8420bd1654145
  status: needs_semantic_conversion
---

# Barbarian Progression

From SRD 5.2.1 page 28.

     Barbarian Features
                Proficiency                                                                          Rage       Weapon
       Level      Bonus        Class Features                                            Rages      Damage      Mastery
         1           +2        Rage, Unarmored Defense, Weapon Mastery                     2          +2           2
         2           +2        Danger Sense, Reckless Attack                               2          +2           2
         3           +2        Barbarian Subclass, Primal Knowledge                        3          +2           2
         4           +2        Ability Score Improvement                                   3          +2           3
         5           +3        Extra Attack, Fast Movement                                 3          +2           3
         6           +3        Subclass feature                                            4          +2           3
         7           +3        Feral Instinct, Instinctive Pounce                          4          +2           3
         8           +3        Ability Score Improvement                                   4          +2           3
         9           +4        Brutal Strike                                               4          +3           3
        10           +4        Subclass feature                                            4          +3           4
        11           +4        Relentless Rage                                             4          +3           4
        12           +4        Ability Score Improvement                                   5          +3           4
        13           +5        Improved Brutal Strike                                      5          +3           4
        14           +5        Subclass feature                                            5          +3           4
        15           +5        Persistent Rage                                             5          +3           4
        16           +5        Ability Score Improvement                                   5          +4           4
        17           +6        Improved Brutal Strike                                      6          +4           4
        18           +6        Indomitable Might                                           6          +4           4
        19           +6        Epic Boon                                                   6          +4           4
        20           +6        Primal Champion                                             6          +4           4



28   System Reference Document 5.2.1
