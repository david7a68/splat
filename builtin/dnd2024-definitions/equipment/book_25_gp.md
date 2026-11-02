---
splat_schema: 1
splat_id: 917193ad-7c25-5052-9e02-9827e0ec2427
splat_kind: item
splat_key: dnd2024:equipment/book_25_gp
splat_name: Book (25 GP)
splat_origin:
  key: dnd2024:reference/pages/096
splat_sources:
- title: SRD 5.2.1
  locator: pp. 96
  url: https://www.dndbeyond.com/srd
---

# Book (25 GP)

## Source text

<!-- SRD page 96 -->

A Book contains fiction or nonfiction. If you consult an accurate nonfiction Book about its topic, you gain a +5 bonus to Intelligence (Arcana, History, Nature, or Religion) checks you make about that topic.

## Item

<!-- splat:values path=splat_item -->

| Field | Value |
| --- | --- |
| ` category ` | ` adventuring_gear ` |

### Equipment

<!-- splat:values path=splat_item/equipment -->

| Field | Value |
| --- | --- |
| ` attunement ` | ` none ` |

### Price

<!-- splat:values path=splat_item/price -->

| Field | Value |
| --- | --- |
| ` value ` | ` 25 ` |
| ` unit ` | ` gp ` |

### Mass

<!-- splat:values path=splat_item/mass -->

| Field | Value |
| --- | --- |
| ` value ` | ` 5.0 ` |
| ` unit ` | ` lb ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:equipment/book_25_gp ` |
| ` implementation ` | ` dnd2024:equipment_book_25_gp ` |

### Parameters

#### Contents kind

<!-- splat:values path=splat_rule/parameters/contents_kind -->

| Field | Value |
| --- | --- |
| ` choice ` | ` ["fiction", "nonfiction"] ` |

#### Consult

<!-- splat:yaml path=splat_rule/parameters/consult -->

```yaml
requirements:
  kind: nonfiction
  accurate: true
  topic_matches_check: true
bonus: 5
roll: ability_check
ability: intelligence
skills:
- arcana
- history
- nature
- religion
```

#### Mass

<!-- splat:values path=splat_rule/parameters/mass -->

| Field | Value |
| --- | --- |
| ` value ` | ` 5.0 ` |
| ` unit ` | ` lb ` |
