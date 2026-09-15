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
---

# Gaining a Level

## Source text

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

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/level_advancement_gaining_a_level ` |
| ` implementation ` | ` dnd2024:rules_level_advancement_gaining_a_level ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` sequence ` | ` ["choose_class", "increase_hp_and_hit_dice", "grant_class_features", "update_proficiency", "update_ability_modifiers"] ` |
| ` class_choice ` | ` ["existing", "eligible_multiclass"] ` |
| ` hit_die_gain ` | ` 1 ` |
| ` grant_features_at ` | ` new_class_level ` |
| ` resolve_feature_choices ` | ` true ` |
| ` proficiency_source ` | ` total_character_level ` |
| ` propagate_stat_changes ` | ` true ` |

#### Hp gain

<!-- splat:yaml path=splat_rule/parameters/hp_gain -->

```yaml
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
```

#### Constitution hp adjustment

<!-- splat:yaml path=splat_rule/parameters/constitution_hp_adjustment -->

```yaml
op: multiply
args:
- var: constitution_modifier_increase
- var: total_character_level
```
