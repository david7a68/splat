---
splat_schema: 1
splat_id: ce429d70-8e73-5bfa-8c4b-cb8c874d9b9a
splat_kind: rule
splat_key: dnd2024:rules/damage_and_healing_instant_death
splat_name: Instant Death
splat_origin:
  key: dnd2024:reference/pages/017
splat_sources:
- title: SRD 5.2.1
  locator: pp. 17
  url: https://www.dndbeyond.com/srd
---

# Instant Death

## Source text

<!-- SRD page 17 -->

Here are the main ways a creature can die instantly.\
Monster Death. A monster dies the instant it\
drops to 0 Hit Points, although a Game Master can\
ignore this rule for an individual monster and treat\
it like a character.\
Hit Point Maximum of 0. A creature dies if its Hit\
Point maximum reaches 0. Certain effects drain life\
energy, reducing a creature’s Hit Point maximum.\
Massive Damage. When damage reduces a char-\
acter to 0 Hit Points and damage remains, the char-\
acter dies if the remainder equals or exceeds their\
Hit Point maximum. For example, if your character\
has a Hit Point maximum of 12, currently has 6 Hit\
Points, and takes 18 damage, the character drops to\
0 Hit Points, but 12 damage remains. The character\
then dies, since 12 equals their Hit Point maximum.\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/damage_and_healing_instant_death ` |
| ` implementation ` | ` dnd2024:rules_damage_and_healing_instant_death ` |

### Parameters

#### Death conditions

<!-- splat:rows path=splat_rule/parameters/death_conditions -->

| Row | subject | hp | hp_maximum | hp_reduced_to | remaining_damage_gte |
| --- | --- | --- | --- | --- | --- |
| ` 0 ` | ` monster ` | ` 0 ` |  |  |  |
| ` 1 ` |  |  | ` 0 ` |  |  |
| ` 2 ` | ` character ` |  |  | ` 0 ` | ` hp_maximum ` |

##### Exception

<!-- splat:values path=splat_rule/parameters/death_conditions/0/exception -->

| Field | Value |
| --- | --- |
| ` authority ` | ` gm ` |
| ` choice ` | ` use_character_death_rules ` |

#### Death rules

<!-- splat:values path=splat_rule/parameters/death_rules -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:rules/damage_and_healing_death_saving_throws ` |
