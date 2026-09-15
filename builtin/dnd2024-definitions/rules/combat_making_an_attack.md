---
splat_schema: 1
splat_id: 29300129-0d51-5aee-9a7c-9600279e98cb
splat_kind: rule
splat_key: dnd2024:rules/combat_making_an_attack
splat_name: Making an Attack
splat_origin:
  key: dnd2024:reference/pages/014
splat_sources:
- title: SRD 5.2.1
  locator: pp. 14, 15
  url: https://www.dndbeyond.com/srd
---

# Making an Attack

## Source text

<!-- SRD page 14 -->

When you take the Attack action, you make an\
attack. Some other actions, Bonus Actions, and Re-\
actions also let you make an attack. Whether you\
strike with a Melee weapon, fire a Ranged weapon,\
unsEEn attackErs and tarGEts\
When you make an attack roll against a target you\
can’t see, you have Disadvantage on the roll. This is\
true whether you’re guessing the target’s location or\
targeting a creature you can hear but not see. If the\
target isn’t in the location you targeted, you miss.\
When a creature can’t see you, you have Advantage\
on attack rolls against it.\
If you are hidden when you make an attack roll,\
you give away your location when the attack hits or\
misses.\

<!-- SRD page 15 -->

or make an attack roll as part of a spell, an attack\
has the following structure:\
1: Choose a Target. Pick a target within your at-\
tack’s range: a creature, an object, or a location.\
2: Determine Modifiers. The GM determines\
whether the target has Cover (see the next sec-\
tion) and whether you have Advantage or Dis-\
advantage against the target. In addition, spells,\
special abilities, and other effects can apply pen-\
alties or bonuses to your attack roll.\
3: Resolve the Attack. Make the attack roll, as\
detailed earlier in “Playing the Game.” On a hit,\
you roll damage unless the particular attack\
has rules that specify otherwise. Some attacks\
cause special effects in addition to or instead of\
damage.\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/combat_making_an_attack ` |
| ` implementation ` | ` dnd2024:rules_combat_making_an_attack ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` sequence ` | ` ["choose_target", "determine_modifiers", "resolve_attack_roll", "resolve_hit_or_miss"] ` |
| ` modifiers ` | ` ["cover", "advantage", "disadvantage", "spell_bonuses", "feature_bonuses", "effect_penalties"] ` |
| ` default_hit ` | ` roll_damage ` |

#### Target

<!-- splat:values path=splat_rule/parameters/target -->

| Field | Value |
| --- | --- |
| ` kinds ` | ` ["creature", "object", "location"] ` |
| ` within_attack_range ` | ` true ` |

#### Attack specific effects

<!-- splat:values path=splat_rule/parameters/attack_specific_effects -->

| Field | Value |
| --- | --- |
| ` may_replace_damage ` | ` true ` |
| ` may_add_to_damage ` | ` true ` |

#### Unseen target

<!-- splat:values path=splat_rule/parameters/unseen_target -->

| Field | Value |
| --- | --- |
| ` disadvantage ` | ` true ` |
| ` wrong_target_location ` | ` automatic_miss ` |

#### Unseen attacker

<!-- splat:values path=splat_rule/parameters/unseen_attacker -->

| Field | Value |
| --- | --- |
| ` advantage ` | ` true ` |

#### Hidden attacker

<!-- splat:values path=splat_rule/parameters/hidden_attacker -->

| Field | Value |
| --- | --- |
| ` reveal_location ` | ` on_hit_or_miss ` |
