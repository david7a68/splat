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

You can give a sentient item an objective it pursues, perhaps to the exclusion of all else. As long as the wielder’s use of the item aligns with that special purpose, the item remains cooperative. Deviating from this course might cause conflict between the wielder and the item (see “Conflict” below). You can pick a special purpose or roll on the Sentient Item’s Special Purpose table.

**Sentient Item’s Alignment**

| 1d100 | Alignment | 1d100 | Alignment |
| --- | --- | --- | --- |
| 01–15 | Lawful Good | 74–85 | Chaotic Neutral |
| 16–35 | Neutral Good | 86–89 | Lawful Evil |
| 36–50 | Chaotic Good | 90–96 | Neutral Evil |
| 51–63 | Lawful Neutral | 97–00 | Chaotic Evil |
| 64–73 | Neutral |  |  |

**Sentient Item’s Communication**

| 1d10 | Communication |
| --- | --- |
| 1–6 | The item communicates by transmitting emotion to the creature carrying or wielding it. |
| 7–9 | The item speaks one or more languages. |
| 10 | The item speaks one or more languages. In addition, the item can communicate telepathically with any creature that carries or wields it. |

**Sentient Item’s Senses**

| 1d4 | Senses |
| --- | --- |
| 1 | Hearing and standard vision out to 30 feet |
| 2 | Hearing and standard vision out to 60 feet |
| 3 | Hearing and standard vision out to 120 feet |
| 4 | Hearing and Darkvision out to 120 feet |

**Sentient Item’s Special Purpose**

| 1d10 | Special Purpose |
| --- | --- |
| 1 | Aligned. The item seeks to defeat or destroy those of a diametrically opposed alignment. Such an item is never Neutral. |
| 2 | Bane. The item seeks to thwart or destroy creatures of a particular type, such as Constructs, Fiends, or Undead. |
| 3 | Creator Seeker. The item seeks its creator and wants to understand why it was created. |
| 4 | Destiny Seeker. The item believes it and its bearer have key roles to play in future events. |

| 1d10 | Special Purpose |
| --- | --- |
| 5 | Destroyer. The item craves destruction and goads its user to fight arbitrarily. |
| 6 | Glory Seeker. The item seeks renown as the greatest magic item in the world by winning fame or notoriety for its user. |
| 7 | Lore Seeker. The item craves knowledge or is determined to solve a mystery, learn a secret, or unravel a cryptic prophecy. |
| 8 | Protector. The item seeks to defend a particular kind of creature, such as elves or werewolves. |
| 9 | Soulmate Seeker. The item seeks another sentient magic item, perhaps one that is similar to itself. |
| 10 | Templar. The item seeks to defend the servants and interests of a particular deity. |

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

##### Roll

<!-- splat:values path=splat_rule/parameters/alignment_table/roll -->

| Field | Value |
| --- | --- |
| ` count ` | ` 1 ` |
| ` sides ` | ` 100 ` |
| ` add ` | ` 0 ` |

##### Outcomes

<!-- splat:table
path: splat_rule/parameters/alignment_table/outcomes
key: Key
order:
- row_1
- row_2
- row_3
- row_4
- row_5
- row_6
- row_7
- row_8
- row_9
columns:
  Key:
    type: key
  Roll min:
    path: roll_min
    type: integer
  Roll max:
    path: roll_max
    type: integer
  Alignment:
    path: alignment
    type: symbol
-->

| Key | Roll min | Roll max | Alignment |
| --- | --- | --- | --- |
| ` row_1 ` | ` 1 ` | ` 15 ` | ` Lawful good ` |
| ` row_2 ` | ` 16 ` | ` 35 ` | ` Neutral good ` |
| ` row_3 ` | ` 36 ` | ` 50 ` | ` Chaotic good ` |
| ` row_4 ` | ` 51 ` | ` 63 ` | ` Lawful neutral ` |
| ` row_5 ` | ` 64 ` | ` 73 ` | ` Neutral ` |
| ` row_6 ` | ` 74 ` | ` 85 ` | ` Chaotic neutral ` |
| ` row_7 ` | ` 86 ` | ` 89 ` | ` Lawful evil ` |
| ` row_8 ` | ` 90 ` | ` 96 ` | ` Neutral evil ` |
| ` row_9 ` | ` 97 ` | ` 100 ` | ` Chaotic evil ` |

#### Communication table

##### Roll

<!-- splat:values path=splat_rule/parameters/communication_table/roll -->

| Field | Value |
| --- | --- |
| ` count ` | ` 1 ` |
| ` sides ` | ` 10 ` |
| ` add ` | ` 0 ` |

##### Outcomes

<!-- splat:table
path: splat_rule/parameters/communication_table/outcomes
key: Key
order:
- row_1
- row_2
- row_3
columns:
  Key:
    type: key
  Roll min:
    path: roll_min
    type: integer
  Roll max:
    path: roll_max
    type: integer
  Recipient:
    path: recipient
    type: symbol
    optional: true
  Languages minimum:
    path: languages_minimum
    type: integer
    optional: true
  Telepathy recipient:
    path: telepathy_recipient
    type: symbol
    optional: true
  Details:
    type: section
    optional: true
-->

| Key | Roll min | Roll max | Recipient | Languages minimum | Telepathy recipient | Details |
| --- | --- | --- | --- | --- | --- | --- |
| ` row_1 ` | ` 1 ` | ` 6 ` | ` Carrier or wielder ` |  |  | [Rules](#^splat-d5dd2f24-row_1 "section:row_1") |
| ` row_2 ` | ` 7 ` | ` 9 ` |  | ` 1 ` |  | [Rules](#^splat-d5dd2f24-row_2 "section:row_2") |
| ` row_3 ` | ` 10 ` | ` 10 ` |  | ` 1 ` | ` Carrier or wielder ` | [Rules](#^splat-d5dd2f24-row_3 "section:row_3") |

###### Row 1

^splat-d5dd2f24-row_1

###### Mode

<!-- splat:yaml path=splat_rule/parameters/communication_table/outcomes/@row_1/mode -->

```yaml
transmit_emotions
```

###### Row 2

^splat-d5dd2f24-row_2

###### Mode

<!-- splat:yaml path=splat_rule/parameters/communication_table/outcomes/@row_2/mode -->

```yaml
speech
```

###### Row 3

^splat-d5dd2f24-row_3

###### Mode

<!-- splat:yaml path=splat_rule/parameters/communication_table/outcomes/@row_3/mode -->

```yaml
- speech
- telepathy
```

#### Senses table

##### Roll

<!-- splat:values path=splat_rule/parameters/senses_table/roll -->

| Field | Value |
| --- | --- |
| ` count ` | ` 1 ` |
| ` sides ` | ` 4 ` |
| ` add ` | ` 0 ` |

##### Outcomes

<!-- splat:table
path: splat_rule/parameters/senses_table/outcomes
key: Key
order:
- row_1
- row_2
- row_3
- row_4
columns:
  Key:
    type: key
  Roll:
    path: roll
    type: integer
  Hearing feet:
    path: hearing_feet
    type: integer
  Vision feet:
    path: vision_feet
    type: integer
  Vision type:
    path: vision_type
    type: symbol
-->

| Key | Roll | Hearing feet | Vision feet | Vision type |
| --- | --- | --- | --- | --- |
| ` row_1 ` | ` 1 ` | ` 30 ` | ` 30 ` | ` Standard ` |
| ` row_2 ` | ` 2 ` | ` 60 ` | ` 60 ` | ` Standard ` |
| ` row_3 ` | ` 3 ` | ` 120 ` | ` 120 ` | ` Standard ` |
| ` row_4 ` | ` 4 ` | ` 120 ` | ` 120 ` | ` Darkvision ` |

#### Purpose table

##### Roll

<!-- splat:values path=splat_rule/parameters/purpose_table/roll -->

| Field | Value |
| --- | --- |
| ` count ` | ` 1 ` |
| ` sides ` | ` 10 ` |
| ` add ` | ` 0 ` |

##### Outcomes

<!-- splat:table
path: splat_rule/parameters/purpose_table/outcomes
key: Variant
order:
- aligned
- bane
- creator_seeker
- destiny_seeker
- destroyer
- glory_seeker
- lore_seeker
- protector
- soulmate_seeker
- templar
columns:
  Variant:
    path: id
    type: symbol
  Roll:
    path: roll
    type: integer
  Objective:
    path: objective
    type: symbol
    optional: true
  Neutral allowed:
    path: neutral_allowed
    type: boolean
    optional: true
  Target choice:
    path: target_choice
    type: symbol
    optional: true
  Objectives:
    path: objectives
    type: yaml
    optional: true
  Objective choices:
    path: objective_choices
    type: yaml
    optional: true
-->

| Variant | Roll | Objective | Neutral allowed | Target choice | Objectives | Objective choices |
| --- | --- | --- | --- | --- | --- | --- |
| ` Aligned ` | ` 1 ` | ` Defeat or destroy opposed alignment ` | false |  |  |  |
| ` Bane ` | ` 2 ` | ` Thwart or destroy ` |  | ` Creature type ` |  |  |
| ` Creator seeker ` | ` 3 ` |  |  |  | ` ["find_creator", "understand_creation"] ` |  |
| ` Destiny seeker ` | ` 4 ` | ` Fulfill item and bearer future roles ` |  |  |  |  |
| ` Destroyer ` | ` 5 ` |  |  |  | ` ["destruction", "arbitrary_fighting"] ` |  |
| ` Glory seeker ` | ` 6 ` |  |  |  | ` ["greatest_item_renown", "user_fame_or_notoriety"] ` |  |
| ` Lore seeker ` | ` 7 ` |  |  |  |  | ` ["knowledge", "solve_mystery", "learn_secret", "decipher_prophecy"] ` |
| ` Protector ` | ` 8 ` | ` Defend ` |  | ` Creature kind ` |  |  |
| ` Soulmate seeker ` | ` 9 ` | ` Find other sentient item ` |  |  |  |  |
| ` Templar ` | ` 10 ` | ` Defend servants and interests ` |  | ` Deity ` |  |  |

#### Conflict rules

<!-- splat:values path=splat_rule/parameters/conflict_rules -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:rules/sentient_magic_items_conflict ` |
