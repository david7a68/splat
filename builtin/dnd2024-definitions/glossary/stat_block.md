---
splat_schema: 2
splat_id: f61b7ff4-6733-5af9-a2db-7e1a86c3f742
splat_kind: rule
splat_key: dnd2024:glossary/stat_block
splat_name: Stat Block
splat_origin: dnd2024:reference/glossary/stat_block
splat_sources:
- SRD 5.2.1, pp. 188, 189, https://www.dndbeyond.com/srd
---

# Stat Block

## Source text

<!-- SRD page 188 -->

A stat block contains the game statistics of a monster. Each stat block includes the following information presented after the monster’s name.

Size. A monster is Tiny, Small, Medium, Large, Huge, or Gargantuan. See also “Size.”

Creature Type. This entry notes the family of beings a monster belongs to, along with any descriptive tags. See also “Creature Type.”

Alignment. An alignment is suggested for the monster, with the GM determining its actual alignment. See also “Alignment.”

AC, Initiative, and HP. These entries give the monster’s Armor Class, Initiative, and Hit Points, which are detailed in “Playing the Game.” In parentheses after the Hit Points, the monster’s Hit Point Dice are provided, along with the contribution of its Constitution, if any, to its Hit Points. Following the Initiative modifier is an Initiative score. Some creatures that are created by magic lack Hit Dice and Initiative information.

Speed. Here the monster’s Speed is provided, along with any special speeds. See also “Burrow Speed,” “Climb Speed,” “Fly Speed,” and “Swim Speed.”

Ability Scores. A table provides the monster’s ability scores, modifiers, and saving throw modifiers, all of which are detailed in “Playing the Game.”

Skills. This entry lists the monster’s skill proficiencies, if any. See also “Playing the Game” (“Proficiency”).

Resistances and Vulnerabilities. These entries list the monster’s Resistances and Vulnerabilities, if any. See also “Resistance” and “Vulnerability.”

Immunities. This section lists the monster’s damage and condition Immunities, if any. See also “Immunity.”

Gear. If the monster has any equipment that can be given away or retrieved, it’s listed in this entry.

Senses. This entry lists the monster’s special senses, such as Darkvision, and its Passive Perception. See also “Passive Perception.”

Languages. This entry lists any languages the monster knows.

<!-- SRD page 189 -->

CR. Challenge Rating summarizes the threat a monster poses and is detailed in “Monsters.” The Experience Points characters receive for defeating a monster and its Proficiency Bonus follow. Some creatures that are created by magic have no CR. See also “Challenge Rating” and “Experience Points.”

Traits. The monster’s traits, if any, are features that are active at all times or in certain situations.

Actions. The monster can take these actions in addition to those detailed in this glossary. See also “Playing the Game” (“Actions”).

Bonus Actions. If the monster has Bonus Action options, they are listed in this section.

Reactions. If the monster can take special Reactions, those are listed in this section.

Attack Notation. The entry for a monster’s attack starts by identifying whether the attack is a melee or a ranged attack and then provides the attack roll’s bonus, its reach or range, and what happens on a hit. An attack is against one target unless its entry says otherwise.

Saving Throw Effect Notation. If an effect forces a saving throw, the effect’s entry starts by identifying the kind of saving throw required and then provides the save’s DC, a description of which creatures must make the save, and what happens on a failed or a successful save.

Damage Notation. A stat block usually provides both a static number and a die expression for each instance of damage. For example, an attack might deal 4 (1d4 + 2) damage on a hit. The GM determines whether you use the static number or the die expression in parentheses; you don’t use both.

Original layout: [[../../srd-5.2.1/pages/188|page 188]], [[../../srd-5.2.1/pages/189|page 189]].

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:glossary/stat_block ` |
| ` implementation ` | ` dnd2024:glossary_stat_block ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` magically_created_creature_optional_fields ` | ` ["hit_dice", "initiative", "challenge_rating"] ` |

#### Schema

<!-- splat:yaml path=splat_rule/parameters/schema -->

```yaml
size: size_category
creature_type: creature_type_and_tags
alignment:
  default: suggested
  actual_chooser: gm
armor_class: integer
initiative:
  modifier: integer
  score: integer
hit_points:
  static: integer
  dice: dice_expression
  constitution_contribution: integer
speed: feet_by_mode
abilities: scores_modifiers_and_save_modifiers
skills: proficiencies
resistances: damage_type_list
vulnerabilities: damage_type_list
immunities: damage_and_condition_lists
gear: giveable_or_retrievable_equipment
senses: special_senses_and_passive_perception
languages: known_languages
challenge_rating:
  rating: number
  experience_points: integer
  proficiency_bonus: integer
traits: always_or_conditionally_active_features
actions: additional_to_glossary_actions
bonus_actions: available_bonus_actions
reactions: available_special_reactions
```

#### Attack

<!-- splat:values path=splat_rule/parameters/attack -->

| Field | Value |
| --- | --- |
| ` fields ` | ` ["melee_or_ranged", "attack_bonus", "reach_or_range", "on_hit"] ` |
| ` default_target_count ` | ` 1 ` |

#### Save effect

<!-- splat:values path=splat_rule/parameters/save_effect -->

| Field | Value |
| --- | --- |
| ` fields ` | ` ["save_ability", "dc", "eligible_creatures", "failure_result", "success_result"] ` |

#### Damage

<!-- splat:yaml path=splat_rule/parameters/damage -->

```yaml
mode_choice:
  chooser: gm
  options:
  - static
  - dice
use_both: false
```
