---
splat_schema: 1
splat_id: dd975fbb-07cb-584a-9d14-9f7f3c0f42b7
splat_kind: rule
splat_key: dnd2024:rules/level_advancement_gaining_a_level
splat_name: Gaining a Level
splat_origin:
  key: dnd2024:reference/pages/023
splat_sources:
- title: SRD 5.2.1
  locator: pp. 23
  url: https://www.dndbeyond.com/srd
splat_rule:
  mechanic: dnd2024:rules/level_advancement_gaining_a_level
  parameters:
    sequence:
    - choose_class
    - increase_hp_and_hit_dice
    - grant_class_features
    - update_proficiency
    - update_ability_modifiers
    class_choice:
    - existing
    - eligible_multiclass
    hit_die_gain: 1
    hp_gain:
      mode:
      - roll_class_hit_die
      - fixed_class_value
      constitution_modifier_added: true
      minimum: 1
      fixed:
        barbarian: 7
        bard: 5
        cleric: 5
        druid: 5
        fighter: 6
        monk: 5
        paladin: 6
        ranger: 6
        rogue: 5
        sorcerer: 4
        warlock: 5
        wizard: 4
    grant_features_at: new_class_level
    resolve_feature_choices: true
    proficiency_source: total_character_level
    propagate_stat_changes: true
    constitution_hp_adjustment:
      op: multiply
      args:
      - var: constitution_modifier_increase
      - var: total_character_level
  implementation: dnd2024:rules_level_advancement_gaining_a_level
splat_contracts:
- id: dnd2024:rules_level_advancement_gaining_a_level
  inputs:
    subject:
      type: participant
      required: true
  outputs:
  - character_statistics
  - features
  - hit_dice
  events:
  - character.level_gained
  capabilities:
  - level_advancement
splat_conversion:
  status: specified
  category: rule
  source_sha256: 8974902d109d6e63672d7c490bde9ccf052410503d9cfa768237154fbc5e3d87
  source_path: builtin/srd-5.2.1/source.pdf
  pages:
  - 23
  engine_gaps: &id001
  - dnd2024:rules_level_advancement_gaining_a_level
  conversion_gaps: []
  execution_profiles: []
  research_path: docs/research/srd-conversion/other_rules_1.md
  specification_sha256: f4912bb65bf6af0d14440e1dc064ff6060d4d190e6f35a6cf6849ba4c2a46afd
  parent_name: Level Advancement
  line_span:
    first: 1766
    count: 45
splat_clauses:
- key: source_rules
  text: '# Gaining a Level When you gain a level, follow these steps: 1: Choose a Class. Most characters
    advance in the same class. However, you might decide to gain a level in another class using the rules
    in the “Multiclassing” section. 2: Adjust Hit Points and Hit Point Dice. Each time you gain a level,
    you gain an additional Hit Die. Roll that die, add your Constitution modifier to the roll, and add
    the total (minimum of 1) to your Hit Point maximum. Instead of rolling, you can use the fixed value
    shown in the Fixed Hit Points by Class table. Fixed Hit Points by Class Class Hit Points per Level
    Barbarian 7 + Con. modifier Fighter, Paladin, or Ranger 6 + Con. modifier Bard, Cleric, Druid, Monk,
    5 + Con. modifier Rogue, or Warlock Sorcerer or Wizard 4 + Con. modifier 3: Record New Class Features.
    Look at your class features table in “Classes,” and note the features you gain at your new level in
    that class. Make any choices offered by a new feature. 4: Adjust Proficiency Bonus. A character’s
    Proficiency Bonus increases at certain levels, as shown in the Character Advancement table and your
    class features table in “Classes.” When your Proficiency Bonus increases, increase all the numbers
    on your character sheet that include your Proficiency Bonus. 5: Adjust Ability Modifiers. If you choose
    a feat that increases one or more of your ability scores, your ability modifier also changes if the
    new score is an even number. When that happens, adjust all the numbers on your character sheet that
    use that ability modifier. When your Constitution modifier increases by 1, your Hit Point maximum
    increases by 1 for each level you have attained. For example, if a character reaches level 8 and increases
    their Constitution score from 17 to 18, the Constitution modifier increases to +4. The character’s
    Hit Point maximum then increases by 8, in addition to the Hit Points gained for reaching level 8.'
  sha256: 04ec96d1b115a510df733bc755dc5ac6781ebf1b1ec584604d9700d0fbbf917f
  status: contract_specified
splat_requires_mechanics: *id001
---

# Gaining a Level

<!-- SRD page 23 -->

When you gain a level, follow these steps:\
1: Choose a Class. Most characters advance in the\
same class. However, you might decide to gain a\
level in another class using the rules in the “Mul-\
ticlassing” section.\
2: Adjust Hit Points and Hit Point Dice. Each time\
you gain a level, you gain an additional Hit Die.\
Roll that die, add your Constitution modifier to\
the roll, and add the total (minimum of 1) to your\
Hit Point maximum. Instead of rolling, you can\
use the fixed value shown in the Fixed Hit Points\
by Class table.\
Fixed Hit Points by Class\
Class Hit Points per Level\
Barbarian 7 + Con. modifier\
Fighter, Paladin, or Ranger 6 + Con. modifier\
Bard, Cleric, Druid, Monk, 5 + Con. modifier\
Rogue, or Warlock\
Sorcerer or Wizard 4 + Con. modifier\
3: Record New Class Features. Look at your class\
features table in “Classes,” and note the features\
you gain at your new level in that class. Make any\
choices offered by a new feature.\
4: Adjust Proficiency Bonus. A character’s Pro-\
ficiency Bonus increases at certain levels, as\
shown in the Character Advancement table and\
your class features table in “Classes.” When your\
Proficiency Bonus increases, increase all the\
numbers on your character sheet that include\
your Proficiency Bonus.\
5: Adjust Ability Modifiers. If you choose a feat\
that increases one or more of your ability scores,\
your ability modifier also changes if the new\
score is an even number. When that happens, ad-\
just all the numbers on your character sheet that\
use that ability modifier. When your Constitution\
modifier increases by 1, your Hit Point maximum\
increases by 1 for each level you have attained.\
For example, if a character reaches level 8 and\
increases their Constitution score from 17 to 18,\
the Constitution modifier increases to +4. The\
character’s Hit Point maximum then increases by\
8, in addition to the Hit Points gained for reach-\
ing level 8.\
