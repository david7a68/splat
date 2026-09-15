---
splat_schema: 1
splat_id: ace29b26-c1ca-5c14-b3f1-327e2642ecd2
splat_kind: rule
splat_key: dnd2024:tables/rogue_progression
splat_name: Rogue Progression
splat_origin:
  key: dnd2024:reference/pages/062
splat_sources:
- title: SRD 5.2.1
  locator: pp. 62
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:class_progression
  parameters:
    class: rogue
    rows:
    - level: 1
      proficiency_bonus: 2
      features_text: Expertise, Sneak Attack, Thieves’ Cant, Weapon Mastery
      sneak_attack: 1d6
    - level: 2
      proficiency_bonus: 2
      features_text: Cunning Action
      sneak_attack: 1d6
    - level: 3
      proficiency_bonus: 2
      features_text: Rogue Subclass, Steady Aim
      sneak_attack: 2d6
    - level: 4
      proficiency_bonus: 2
      features_text: Ability Score Improvement
      sneak_attack: 2d6
    - level: 5
      proficiency_bonus: 3
      features_text: Cunning Strike, Uncanny Dodge
      sneak_attack: 3d6
    - level: 6
      proficiency_bonus: 3
      features_text: Expertise
      sneak_attack: 3d6
    - level: 7
      proficiency_bonus: 3
      features_text: Evasion, Reliable Talent
      sneak_attack: 4d6
    - level: 8
      proficiency_bonus: 3
      features_text: Ability Score Improvement
      sneak_attack: 4d6
    - level: 9
      proficiency_bonus: 4
      features_text: Subclass feature
      sneak_attack: 5d6
    - level: 10
      proficiency_bonus: 4
      features_text: Ability Score Improvement
      sneak_attack: 5d6
    - level: 11
      proficiency_bonus: 4
      features_text: Improved Cunning Strike
      sneak_attack: 6d6
    - level: 12
      proficiency_bonus: 4
      features_text: Ability Score Improvement
      sneak_attack: 6d6
    - level: 13
      proficiency_bonus: 5
      features_text: Subclass feature
      sneak_attack: 7d6
    - level: 14
      proficiency_bonus: 5
      features_text: Devious Strikes
      sneak_attack: 7d6
    - level: 15
      proficiency_bonus: 5
      features_text: Slippery Mind
      sneak_attack: 8d6
    - level: 16
      proficiency_bonus: 5
      features_text: Ability Score Improvement
      sneak_attack: 8d6
    - level: 17
      proficiency_bonus: 6
      features_text: Subclass feature
      sneak_attack: 9d6
    - level: 18
      proficiency_bonus: 6
      features_text: Elusive
      sneak_attack: 9d6
    - level: 19
      proficiency_bonus: 6
      features_text: Epic Boon
      sneak_attack: 10d6
    - level: 20
      proficiency_bonus: 6
      features_text: Stroke of Luck
      sneak_attack: 10d6
splat_conversion:
  status: partial
  category: table
  source_sha256: 4456eaa81743ed341c13f19d1d7ced344a6adcbb15367941875ee45cd3f2e562
  source_path: builtin/srd-5.2.1/source.txt
  pages:
  - 62
  engine_gaps: []
  conversion_gaps:
  - source_rules
  execution_profiles: []
splat_clauses:
- key: source_rules
  text: '# Rogue Progression From SRD 5.2.1 page 62. Rogue Features Proficiency Level Bonus Class Features
    Sneak Attack 1 +2 Expertise, Sneak Attack, Thieves’ Cant, Weapon Mastery 1d6 2 +2 Cunning Action 1d6
    3 +2 Rogue Subclass, Steady Aim 2d6 4 +2 Ability Score Improvement 2d6 5 +3 Cunning Strike, Uncanny
    Dodge 3d6 6 +3 Expertise 3d6 7 +3 Evasion, Reliable Talent 4d6 8 +3 Ability Score Improvement 4d6
    9 +4 Subclass feature 5d6 10 +4 Ability Score Improvement 5d6 11 +4 Improved Cunning Strike 6d6 12
    +4 Ability Score Improvement 6d6 13 +5 Subclass feature 7d6 14 +5 Devious Strikes 7d6 15 +5 Slippery
    Mind 8d6 16 +5 Ability Score Improvement 8d6 17 +6 Subclass feature 9d6 18 +6 Elusive 9d6 19 +6 Epic
    Boon 10d6 20 +6 Stroke of Luck 10d6 roll if you have Advantage on the roll and the attack Whenever
    you finish a Long Rest, you can change uses a Finesse or a Ranged weapon. The extra damthe kinds of
    weapons you chose. For example, you age’s type is the same as the weapon’s type. could switch to using
    the mastery properties of You don’t need Advantage on the attack roll if at Scimitars and Shortswords.
    least one of your allies is within 5 feet of the target, the ally doesn’t have the Incapacitated condition,
    Level 2: Cunning Action and you don’t have Disadvantage on the attack roll. Your quick thinking and
    agility allow you to move The extra damage increases as you gain Rogue and act quickly. On your turn,
    you can take one of levels, as shown in the Sneak Attack column of the the following actions as a
    Bonus Action: Dash, Dis- Rogue Features table. engage, or Hide. Level 1: Thieves’ Cant Level 3: Rogue
    Subclass You picked up various languages in the commu- You gain a Rogue subclass of your choice. The
    Thief nities where you plied your roguish talents. You subclass is detailed after this class’s description.
    A know Thieves’ Cant and one other language of your subclass is a specialization that grants you features
    choice, which you choose from the language tables at certain Rogue levels. For the rest of your career,
    in “Character Creation.” you gain each of your subclass’s features that are of your Rogue level or
    lower. Level 1: Weapon Mastery Your training with weapons allows you to use the Level 3: Steady Aim
    mastery properties of two kinds of weapons of your As a Bonus Action, you give yourself Advantage
    on choice with which you have proficiency, such as your next attack roll on the current turn. You
    can Daggers and Shortbows. use this feature only if you haven’t moved during this turn, and after
    you use it, your Speed is 0 until the end of the current turn. 62 System Reference Document 5.2.1'
  sha256: 89b1599538f53b10ed1ca468aa822bfc024a053f4a63713ab32b1608904c4f50
  status: needs_semantic_conversion
---

# Rogue Progression

From SRD 5.2.1 page 62.

     Rogue Features
              Proficiency
      Level   Bonus         Class Features                                                        Sneak Attack
        1         +2        Expertise, Sneak Attack, Thieves’ Cant, Weapon Mastery                     1d6
        2         +2        Cunning Action                                                             1d6
        3         +2        Rogue Subclass, Steady Aim                                                 2d6
        4         +2        Ability Score Improvement                                                  2d6
        5         +3        Cunning Strike, Uncanny Dodge                                              3d6
        6         +3        Expertise                                                                  3d6
        7         +3        Evasion, Reliable Talent                                                   4d6
        8         +3        Ability Score Improvement                                                  4d6
        9         +4        Subclass feature                                                           5d6
       10         +4        Ability Score Improvement                                                  5d6
       11         +4        Improved Cunning Strike                                                    6d6
       12         +4        Ability Score Improvement                                                  6d6
       13         +5        Subclass feature                                                            7d6
       14         +5        Devious Strikes                                                             7d6
       15         +5        Slippery Mind                                                              8d6
       16         +5        Ability Score Improvement                                                  8d6
       17         +6        Subclass feature                                                           9d6
       18         +6        Elusive                                                                    9d6
       19         +6        Epic Boon                                                                 10d6
       20         +6        Stroke of Luck                                                            10d6


     roll if you have Advantage on the roll and the attack         Whenever you finish a Long Rest, you can change
     uses a Finesse or a Ranged weapon. The extra dam-           the kinds of weapons you chose. For example, you
     age’s type is the same as the weapon’s type.                could switch to using the mastery properties of
       You don’t need Advantage on the attack roll if at         Scimitars and Shortswords.
     least one of your allies is within 5 feet of the target,
     the ally doesn’t have the Incapacitated condition,          Level 2: Cunning Action
     and you don’t have Disadvantage on the attack roll.         Your quick thinking and agility allow you to move
       The extra damage increases as you gain Rogue              and act quickly. On your turn, you can take one of
     levels, as shown in the Sneak Attack column of the          the following actions as a Bonus Action: Dash, Dis-
     Rogue Features table.                                       engage, or Hide.

     Level 1: Thieves’ Cant                                      Level 3: Rogue Subclass
     You picked up various languages in the commu-               You gain a Rogue subclass of your choice. The Thief
     nities where you plied your roguish talents. You            subclass is detailed after this class’s description. A
     know Thieves’ Cant and one other language of your           subclass is a specialization that grants you features
     choice, which you choose from the language tables           at certain Rogue levels. For the rest of your career,
     in “Character Creation.”                                    you gain each of your subclass’s features that are of
                                                                 your Rogue level or lower.
     Level 1: Weapon Mastery
     Your training with weapons allows you to use the            Level 3: Steady Aim
     mastery properties of two kinds of weapons of your          As a Bonus Action, you give yourself Advantage on
     choice with which you have proficiency, such as             your next attack roll on the current turn. You can
     Daggers and Shortbows.                                      use this feature only if you haven’t moved during
                                                                 this turn, and after you use it, your Speed is 0 until
                                                                 the end of the current turn.




62   System Reference Document 5.2.1
