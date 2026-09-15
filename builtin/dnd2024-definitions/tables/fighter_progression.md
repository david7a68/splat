---
splat_schema: 1
splat_id: a8941e4b-da5a-5490-aed3-a271872d1611
splat_kind: rule
splat_key: dnd2024:tables/fighter_progression
splat_name: Fighter Progression
splat_origin:
  key: dnd2024:reference/pages/047
splat_sources:
- title: SRD 5.2.1
  locator: pp. 47
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_progression
  parameters:
    class: fighter
    rows:
    - level: 1
      proficiency_bonus: 2
      features_text: Fighting Style, Second Wind, Weapon Mastery
      second_wind: 2
      weapon_mastery: 3
    - level: 2
      proficiency_bonus: 2
      features_text: Action Surge (one use), Tactical Mind
      second_wind: 2
      weapon_mastery: 3
    - level: 3
      proficiency_bonus: 2
      features_text: Fighter Subclass
      second_wind: 2
      weapon_mastery: 3
    - level: 4
      proficiency_bonus: 2
      features_text: Ability Score Improvement
      second_wind: 3
      weapon_mastery: 4
    - level: 5
      proficiency_bonus: 3
      features_text: Extra Attack, Tactical Shift
      second_wind: 3
      weapon_mastery: 4
    - level: 6
      proficiency_bonus: 3
      features_text: Ability Score Improvement
      second_wind: 3
      weapon_mastery: 4
    - level: 7
      proficiency_bonus: 3
      features_text: Subclass feature
      second_wind: 3
      weapon_mastery: 4
    - level: 8
      proficiency_bonus: 3
      features_text: Ability Score Improvement
      second_wind: 3
      weapon_mastery: 4
    - level: 9
      proficiency_bonus: 4
      features_text: Indomitable (one use), Tactical Master
      second_wind: 3
      weapon_mastery: 4
    - level: 10
      proficiency_bonus: 4
      features_text: Subclass feature
      second_wind: 4
      weapon_mastery: 5
    - level: 11
      proficiency_bonus: 4
      features_text: Two Extra Attacks
      second_wind: 4
      weapon_mastery: 5
    - level: 12
      proficiency_bonus: 4
      features_text: Ability Score Improvement
      second_wind: 4
      weapon_mastery: 5
    - level: 13
      proficiency_bonus: 5
      features_text: Indomitable (two uses), Studied Attacks
      second_wind: 4
      weapon_mastery: 5
    - level: 14
      proficiency_bonus: 5
      features_text: Ability Score Improvement
      second_wind: 4
      weapon_mastery: 5
    - level: 15
      proficiency_bonus: 5
      features_text: Subclass feature
      second_wind: 4
      weapon_mastery: 5
    - level: 16
      proficiency_bonus: 5
      features_text: Ability Score Improvement
      second_wind: 4
      weapon_mastery: 6
    - level: 17
      proficiency_bonus: 6
      features_text: Action Surge (two uses), Indomitable (three uses)
      second_wind: 4
      weapon_mastery: 6
    - level: 18
      proficiency_bonus: 6
      features_text: Subclass feature
      second_wind: 4
      weapon_mastery: 6
    - level: 19
      proficiency_bonus: 6
      features_text: Epic Boon
      second_wind: 4
      weapon_mastery: 6
    - level: 20
      proficiency_bonus: 6
      features_text: Three Extra Attacks
      second_wind: 4
      weapon_mastery: 6
splat_conversion:
  status: partial
  category: table
  source_sha256: 4456eaa81743ed341c13f19d1d7ced344a6adcbb15367941875ee45cd3f2e562
  source_path: builtin/srd-5.2.1/source.txt
  pages:
  - 47
  engine_gaps: []
  conversion_gaps:
  - source_rules
  execution_profiles: []
splat_clauses:
- key: source_rules
  text: '# Fighter Progression From SRD 5.2.1 page 47. Fighter Features Proficiency Second Weapon Level
    Bonus Class Features Wind Mastery 1 +2 Fighting Style, Second Wind, Weapon Mastery 2 3 2 +2 Action
    Surge (one use), Tactical Mind 2 3 3 +2 Fighter Subclass 2 3 4 +2 Ability Score Improvement 3 4 5
    +3 Extra Attack, Tactical Shift 3 4 6 +3 Ability Score Improvement 3 4 7 +3 Subclass feature 3 4 8
    +3 Ability Score Improvement 3 4 9 +4 Indomitable (one use), Tactical Master 3 4 10 +4 Subclass feature
    4 5 11 +4 Two Extra Attacks 4 5 12 +4 Ability Score Improvement 4 5 13 +5 Indomitable (two uses),
    Studied Attacks 4 5 14 +5 Ability Score Improvement 4 5 15 +5 Subclass feature 4 5 16 +5 Ability Score
    Improvement 4 6 17 +6 Action Surge (two uses), Indomitable (three uses) 4 6 18 +6 Subclass feature
    4 6 19 +6 Epic Boon 4 6 20 +6 Three Extra Attacks 4 6 47 System Reference Document 5.2.1'
  sha256: 765e1c0fc46470ae3fec75614bcfb46ed868eaf0fb81ea27282797840e06bb1d
  status: needs_semantic_conversion
---

# Fighter Progression

From SRD 5.2.1 page 47.

     Fighter Features
                  Proficiency                                                                     Second        Weapon
       Level        Bonus          Class Features                                                  Wind         Mastery
         1             +2         Fighting Style, Second Wind, Weapon Mastery                        2              3
         2             +2         Action Surge (one use), Tactical Mind                              2              3
         3             +2         Fighter Subclass                                                   2              3
         4             +2         Ability Score Improvement                                          3              4
         5             +3         Extra Attack, Tactical Shift                                       3              4
         6             +3         Ability Score Improvement                                          3              4
         7             +3         Subclass feature                                                   3              4
         8             +3         Ability Score Improvement                                          3              4
         9             +4         Indomitable (one use), Tactical Master                             3              4
         10            +4         Subclass feature                                                   4              5
         11            +4         Two Extra Attacks                                                  4              5
         12            +4         Ability Score Improvement                                          4              5
         13            +5         Indomitable (two uses), Studied Attacks                            4              5
         14            +5         Ability Score Improvement                                          4              5
         15            +5         Subclass feature                                                   4              5
         16            +5         Ability Score Improvement                                          4              6
         17            +6         Action Surge (two uses), Indomitable (three uses)                  4              6
         18            +6         Subclass feature                                                   4              6
         19            +6         Epic Boon                                                          4              6
         20            +6         Three Extra Attacks                                                4              6



47   System Reference Document 5.2.1
