---
splat_schema: 1
splat_id: 498123da-5d02-5904-91b8-c3c11d1026c3
splat_kind: rule
splat_key: dnd2024:class_features/bard_level_1_bardic_inspiration
splat_name: 'Level 1: Bardic Inspiration'
splat_origin:
  key: dnd2024:reference/pages/031
splat_sources:
- title: SRD 5.2.1
  locator: pp. 31, 32
  url: https://www.dndbeyond.com/srd
---

# Level 1: Bardic Inspiration

## Source text

<!-- SRD page 31 -->

You can supernaturally inspire others through\
words, music, or dance. This inspiration is repre-\
sented by your Bardic Inspiration die, which is a d6.\
Using Bardic Inspiration. As a Bonus Action, you\
can inspire another creature within 60 feet of your-\
self who can see or hear you. That creature gains\
Prepared ——Spell Slots per Spell Level——\
Cantrips Spells 1 2 3 4 5 6 7 8 9\
2 4 2 — — — — — — — —\
2 5 3 — — — — — — — —\
2 6 4 2 — — — — — — —\
3 7 4 3 — — — — — — —\
3 9 4 3 2 — — — — — —\
3 10 4 3 3 — — — — — —\
3 11 4 3 3 1 — — — — —\
3 12 4 3 3 2 — — — — —\
3 14 4 3 3 3 1 — — — —\
4 15 4 3 3 3 2 — — — —\
4 16 4 3 3 3 2 1 — — —\
4 16 4 3 3 3 2 1 — — —\
4 17 4 3 3 3 2 1 1 — —\
4 17 4 3 3 3 2 1 1 — —\
4 18 4 3 3 3 2 1 1 1 —\
4 18 4 3 3 3 2 1 1 1 —\
4 19 4 3 3 3 2 1 1 1 1\
4 20 4 3 3 3 3 1 1 1 1\
4 21 4 3 3 3 3 2 1 1 1\
4 22 4 3 3 3 3 2 2 1 1\

<!-- SRD page 32 -->

one of your Bardic Inspiration dice. A creature can\
have only one Bardic Inspiration die at a time.\
Once within the next hour when the creature fails\
a D20 Test, the creature can roll the Bardic Inspira-\
tion die and add the number rolled to the d20, po-\
tentially turning the failure into a success. A Bardic\
Inspiration die is expended when it’s rolled.\
Number of Uses. You can confer a Bardic Inspira-\
tion die a number of times equal to your Charisma\
modifier (minimum of once), and you regain all ex-\
pended uses when you finish a Long Rest.\
At Higher Levels. Your Bardic Inspiration die\
changes when you reach certain Bard levels, as\
shown in the Bardic Die column of the Bard Fea-\
tures table. The die becomes a d8 at level 5, a d10 at\
level 10, and a d12 at level 15.\

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:class_features/bard_level_1_bardic_inspiration ` |
| ` implementation ` | ` dnd2024:class_features_bard_level_1_bardic_inspiration ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` action ` | ` bonus_action ` |
| ` cost ` | ` 1 ` |
| ` maximum_dice_per_creature ` | ` 1 ` |

#### Resource

<!-- splat:yaml path=splat_rule/parameters/resource -->

```yaml
id: bard.bardic_inspiration
maximum:
  op: max
  args:
  - 1
  - var: actor.abilities.charisma.modifier
recovery:
- event: rest.long_completed
  amount: all
```

#### Target

<!-- splat:values path=splat_rule/parameters/target -->

| Field | Value |
| --- | --- |
| ` creature ` | ` true ` |
| ` self ` | ` false ` |
| ` range_feet ` | ` 60 ` |
| ` perceives_actor_any ` | ` ["sight", "hearing"] ` |

#### Die

<!-- splat:yaml path=splat_rule/parameters/die -->

```yaml
sides_by_bard_level:
  '1': 6
  '5': 8
  '10': 10
  '15': 12
count: 1
```

#### Duration

<!-- splat:values path=splat_rule/parameters/duration -->

| Field | Value |
| --- | --- |
| ` hours ` | ` 1 ` |

#### Use

<!-- splat:values path=splat_rule/parameters/use -->

| Field | Value |
| --- | --- |
| ` trigger ` | ` target.d20_test.failed ` |
| ` optional ` | ` true ` |
| ` roll_die ` | ` true ` |
| ` add_to ` | ` d20 ` |
| ` recompute_success ` | ` true ` |
| ` consume ` | ` on_roll ` |
| ` maximum_uses ` | ` 1 ` |

#### Class scope

<!-- splat:values path=splat_rule/parameters/class_scope -->

| Field | Value |
| --- | --- |
| ` class ` | ` bard ` |
| ` minimum_level ` | ` 1 ` |
