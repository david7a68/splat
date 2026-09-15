---
splat_schema: 1
splat_id: 063a68d0-1f6c-52ac-94ac-e756933dc5d0
splat_kind: rule
splat_key: dnd2024:glossary/passive_perception
splat_name: Passive Perception
splat_origin:
  key: dnd2024:reference/glossary/passive_perception
splat_sources:
- title: SRD 5.2.1
  locator: pp. 186
  url: https://www.dndbeyond.com/srd
---

# Passive Perception

## Source text

<!-- SRD page 186 -->

Passive Perception is a score that reflects a crea-\
ture’s general awareness of its surroundings. The\
GM uses this score when determining whether a\
creature notices something without consciously\
making a Wisdom (Perception) check.\
A creature’s Passive Perception equals 10 plus the\
creature’s Wisdom (Perception) check bonus. If the\
creature has Advantage on such checks, increase\
the score by 5. If the creature has Disadvantage on\
them, decrease the score by 5. For example, a level\
1 character with a Wisdom of 15 and proficiency in\
Perception has a Passive Perception of 14 (10 + 2 +\
2). If that character has Advantage on Wisdom (Per-\
ception) checks, the score becomes 19.\

Original layout: [[../../srd-5.2.1/pages/186|page 186]].

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:glossary/passive_perception ` |
| ` implementation ` | ` dnd2024:glossary_passive_perception ` |

### Parameters

#### Use

<!-- splat:values path=splat_rule/parameters/use -->

| Field | Value |
| --- | --- |
| ` chooser ` | ` gm ` |
| ` scope ` | ` notice_without_conscious_perception_check ` |

#### Score

<!-- splat:yaml path=splat_rule/parameters/score -->

```yaml
op: add
args:
- 10
- var: wisdom_perception_check_bonus
```

#### Roll mode adjustment

<!-- splat:values path=splat_rule/parameters/roll_mode_adjustment -->

| Field | Value |
| --- | --- |
| ` advantage ` | ` 5 ` |
| ` disadvantage ` | ` -5 ` |
| ` normal ` | ` 0 ` |
