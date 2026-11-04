---
splat_schema: 2
splat_id: 917193ad-7c25-5052-9e02-9827e0ec2427
splat_kind: item
splat_key: dnd2024:equipment/book_25_gp
splat_name: Book (25 GP)
splat_origin: dnd2024:reference/pages/096
splat_sources:
- SRD 5.2.1, pp. 96, https://www.dndbeyond.com/srd
---

# Book (25 GP)


## Item

| Field | Value |
| --- | --- |
| Type | Adventuring Gear |
| Rarity |  |
| Attunement | No |


## Source text

<!-- SRD page 96 -->

A Book contains fiction or nonfiction. If you consult an accurate nonfiction Book about its topic, you gain a +5 bonus to Intelligence (Arcana, History, Nature, or Religion) checks you make about that topic.









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

> [!note]- Splat data (edit in Splat)
> <!-- splat:yaml path=splat_item_extras -->
> 
> ```yaml
> price:
>   unit: gp
>   value: 25
> mass:
>   unit: lb
>   value: 5.0
> ```

