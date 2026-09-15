---
splat_schema: 1
splat_id: 9b635d79-3b98-56da-bd2d-f3b909067e3c
splat_kind: rule
splat_key: dnd2024:rules/sentient_magic_items_special_purpose
splat_name: Special Purpose
splat_origin:
  key: dnd2024:reference/pages/208
splat_sources:
- title: SRD 5.2.1
  locator: pp. 208
  url: https://www.dndbeyond.com/srd
---

# Special Purpose

## Source text

<!-- SRD page 208 -->

You can give a sentient item an objective it pursues,\
perhaps to the exclusion of all else. As long as the\
wielder’s use of the item aligns with that special\
purpose, the item remains cooperative. Deviating\
from this course might cause conflict between the\
wielder and the item (see “Conflict” below). You can\
pick a special purpose or roll on the Sentient Item’s\
Special Purpose table.\
Sentient Item’s Alignment\
1d100 Alignment 1d100 Alignment\
01–15 Lawful Good 74–85 Chaotic Neutral\
16–35 Neutral Good 86–89 Lawful Evil\
36–50 Chaotic Good 90–96 Neutral Evil\
51–63 Lawful Neutral 97–00 Chaotic Evil\
64–73 Neutral\
Sentient Item’s Communication\
1d10 Communication\
1–6 The item communicates by transmitting emo-\
tion to the creature carrying or wielding it.\
7–9 The item speaks one or more languages.\
10 The item speaks one or more languages. In ad-\
dition, the item can communicate telepathically\
with any creature that carries or wields it.\
Sentient Item’s Senses\
1d4 Senses\
1 Hearing and standard vision out to 30 feet\
2 Hearing and standard vision out to 60 feet\
3 Hearing and standard vision out to 120 feet\
4 Hearing and Darkvision out to 120 feet\
Sentient Item’s Special Purpose\
1d10 Special Purpose\
1 Aligned. The item seeks to defeat or destroy\
those of a diametrically opposed alignment.\
Such an item is never Neutral.\
2 Bane. The item seeks to thwart or destroy\
creatures of a particular type, such as Con-\
structs, Fiends, or Undead.\
3 Creator Seeker. The item seeks its creator\
and wants to understand why it was created.\
4 Destiny Seeker. The item believes it and its\
bearer have key roles to play in future events.\
1d10 Special Purpose\
5 Destroyer. The item craves destruction and\
goads its user to fight arbitrarily.\
6 Glory Seeker. The item seeks renown as the\
greatest magic item in the world by winning\
fame or notoriety for its user.\
7 Lore Seeker. The item craves knowledge or is\
determined to solve a mystery, learn a secret,\
or unravel a cryptic prophecy.\
8 Protector. The item seeks to defend a par-\
ticular kind of creature, such as elves or\
werewolves.\
9 Soulmate Seeker. The item seeks another\
sentient magic item, perhaps one that is similar\
to itself.\
10 Templar. The item seeks to defend the ser-\
vants and interests of a particular deity.\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:rules/sentient_magic_items_special_purpose ` |
| ` implementation ` | ` dnd2024:rules_sentient_magic_items_special_purpose ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` purpose_optional ` | ` true ` |
| ` purpose_methods ` | ` ["choose", "roll"] ` |
| ` cooperative_when ` | ` wielder_use_aligns_with_purpose ` |
| ` possible_conflict_when ` | ` wielder_deviates_from_purpose ` |

#### Alignment table

<!-- splat:yaml path=splat_rule/parameters/alignment_table -->

```yaml
roll:
  count: 1
  sides: 100
  add: 0
outcomes:
- roll_min: 1
  roll_max: 15
  alignment: lawful_good
- roll_min: 16
  roll_max: 35
  alignment: neutral_good
- roll_min: 36
  roll_max: 50
  alignment: chaotic_good
- roll_min: 51
  roll_max: 63
  alignment: lawful_neutral
- roll_min: 64
  roll_max: 73
  alignment: neutral
- roll_min: 74
  roll_max: 85
  alignment: chaotic_neutral
- roll_min: 86
  roll_max: 89
  alignment: lawful_evil
- roll_min: 90
  roll_max: 96
  alignment: neutral_evil
- roll_min: 97
  roll_max: 100
  alignment: chaotic_evil
```

#### Communication table

<!-- splat:yaml path=splat_rule/parameters/communication_table -->

```yaml
roll:
  count: 1
  sides: 10
  add: 0
outcomes:
- roll_min: 1
  roll_max: 6
  mode: transmit_emotions
  recipient: carrier_or_wielder
- roll_min: 7
  roll_max: 9
  mode: speech
  languages_minimum: 1
- roll_min: 10
  roll_max: 10
  mode:
  - speech
  - telepathy
  languages_minimum: 1
  telepathy_recipient: carrier_or_wielder
```

#### Senses table

<!-- splat:yaml path=splat_rule/parameters/senses_table -->

```yaml
roll:
  count: 1
  sides: 4
  add: 0
outcomes:
- roll: 1
  hearing_feet: 30
  vision_feet: 30
  vision_type: standard
- roll: 2
  hearing_feet: 60
  vision_feet: 60
  vision_type: standard
- roll: 3
  hearing_feet: 120
  vision_feet: 120
  vision_type: standard
- roll: 4
  hearing_feet: 120
  vision_feet: 120
  vision_type: darkvision
```

#### Purpose table

<!-- splat:yaml path=splat_rule/parameters/purpose_table -->

```yaml
roll:
  count: 1
  sides: 10
  add: 0
outcomes:
- roll: 1
  id: aligned
  objective: defeat_or_destroy_opposed_alignment
  neutral_allowed: false
- roll: 2
  id: bane
  objective: thwart_or_destroy
  target_choice: creature_type
- roll: 3
  id: creator_seeker
  objectives:
  - find_creator
  - understand_creation
- roll: 4
  id: destiny_seeker
  objective: fulfill_item_and_bearer_future_roles
- roll: 5
  id: destroyer
  objectives:
  - destruction
  - arbitrary_fighting
- roll: 6
  id: glory_seeker
  objectives:
  - greatest_item_renown
  - user_fame_or_notoriety
- roll: 7
  id: lore_seeker
  objective_choices:
  - knowledge
  - solve_mystery
  - learn_secret
  - decipher_prophecy
- roll: 8
  id: protector
  objective: defend
  target_choice: creature_kind
- roll: 9
  id: soulmate_seeker
  objective: find_other_sentient_item
- roll: 10
  id: templar
  objective: defend_servants_and_interests
  target_choice: deity
```

#### Conflict rules

<!-- splat:values path=splat_rule/parameters/conflict_rules -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:rules/sentient_magic_items_conflict ` |
