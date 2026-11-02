---
splat_schema: 1
splat_id: afd0c8f1-476b-56c3-8c8a-d37f4f45c93a
splat_kind: rule
splat_key: dnd2024:glossary/heroic_inspiration
splat_name: Heroic Inspiration
splat_origin:
  key: dnd2024:reference/glossary/heroic_inspiration
splat_sources:
- title: SRD 5.2.1
  locator: pp. 183
  url: https://www.dndbeyond.com/srd
---

# Heroic Inspiration

## Source text

<!-- SRD page 183 -->

If you (a player character) have Heroic Inspiration, you can expend it to reroll any die immediately after rolling it, and you must use the new roll.

If you gain Heroic Inspiration but already have it, it’s lost unless you give it to a player character who lacks it.

Original layout: [[../../srd-5.2.1/pages/183|page 183]].

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:glossary/heroic_inspiration ` |
| ` implementation ` | ` dnd2024:glossary_heroic_inspiration ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` eligible ` | ` player_character ` |
| ` capacity ` | ` 1 ` |

#### Expend

<!-- splat:values path=splat_rule/parameters/expend -->

| Field | Value |
| --- | --- |
| ` timing ` | ` immediately_after_rolling_any_die ` |
| ` reroll_count ` | ` 1 ` |
| ` must_use_new_result ` | ` true ` |

#### Gain while full

<!-- splat:yaml path=splat_rule/parameters/gain_while_full -->

```yaml
default: lost
optional_transfer:
  recipient: player_character_without_heroic_inspiration
  chooser: subject
```
