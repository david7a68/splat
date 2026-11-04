---
splat_schema: 2
splat_id: 00889495-d8d4-52f0-b527-78b0157399c2
splat_kind: rule
splat_key: dnd2024:rules/d20_tests
splat_name: D20 Tests
splat_origin: dnd2024:reference/pages/006
splat_sources:
- SRD 5.2.1, pp. 6, https://www.dndbeyond.com/srd
---

# D20 Tests

## Source text

<!-- SRD page 6 -->

When the outcome of an action is uncertain, the game uses a d20 roll to determine success or failure. These rolls are called D20 Tests, and they come in three kinds: ability checks, saving throws, and attack rolls. They follow these steps:

4: Roll 1d20. You always want to roll high. If the roll has Advantage or Disadvantage (described later in “Playing the Game”), you roll two d20s, but you use the number from only one of them— the higher one if you have Advantage or the lower one if you have Disadvantage.

5: Add Modifiers. Add these modifiers to the number rolled on the d20:

•  The Relevant Ability Modifier. “Playing the Game” and “Rules Glossary” explain which ability modifiers to use for various D20 Tests.

•  Your Proficiency Bonus If Relevant. Each creature has a Proficiency Bonus, a number added when making a D20 Test that uses something, such as a skill, in which the creature has proficiency. See “Proficiency” later in “Playing the Game.”

•  Circumstantial Bonuses and Penalties. A class feature, a spell, or another rule might give a bonus or penalty to the die roll.

6: Compare the Total to a Target Number. If the total of the d20 and its modifiers equals or exceeds the target number, the D20 Test succeeds. Otherwise, it fails. The Game Master determines target numbers and tells players whether their rolls are successful. The target number for an ability check or a saving throw is called a Difficulty Class (DC). The target number for an attack roll is called an Armor Class (AC), which appears on a character sheet or in a stat block (see “Rules Glossary”).

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/d20_tests ` |
| ` implementation ` | ` dnd2024:rules_d20_tests ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` test_kinds ` | ` ["ability_check", "saving_throw", "attack_roll"] ` |
| ` target_number_authority ` | ` gm ` |

#### Die

<!-- splat:values path=splat_rule/parameters/die -->

| Field | Value |
| --- | --- |
| ` count ` | ` 1 ` |
| ` sides ` | ` 20 ` |
| ` add ` | ` 0 ` |

#### Advantage

<!-- splat:yaml path=splat_rule/parameters/advantage -->

```yaml
dice:
  count: 2
  sides: 20
  add: 0
keep: highest
```

#### Disadvantage

<!-- splat:yaml path=splat_rule/parameters/disadvantage -->

```yaml
dice:
  count: 2
  sides: 20
  add: 0
keep: lowest
```

#### Total

<!-- splat:values path=splat_rule/parameters/total -->

| Field | Value |
| --- | --- |
| ` op ` | ` sum ` |
| ` terms ` | ` ["selected_die", "relevant_ability_modifier", "applicable_proficiency_bonus", "circumstantial_bonuses_and_penalties"] ` |

#### Success

<!-- splat:values path=splat_rule/parameters/success -->

| Field | Value |
| --- | --- |
| ` op ` | ` gte ` |
| ` left ` | ` total ` |
| ` right ` | ` target_number ` |

#### Targets

<!-- splat:values path=splat_rule/parameters/targets -->

| Field | Value |
| --- | --- |
| ` ability_check ` | ` difficulty_class ` |
| ` saving_throw ` | ` difficulty_class ` |
| ` attack_roll ` | ` armor_class ` |
