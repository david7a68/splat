---
splat_schema: 1
splat_id: 6776851c-fb4c-5fac-a0f3-e0e73424054d
splat_kind: item
splat_key: dnd2024:items/periapt_of_health
splat_name: Periapt of Health
splat_origin:
  key: dnd2024:reference/magic_item/periapt_of_health
splat_sources:
- title: SRD 5.2.1
  locator: pp. 234
  url: https://www.dndbeyond.com/srd
---

# Periapt of Health

## Source text

<!-- SRD page 234 -->

Wondrous Item, Uncommon (Requires Attunement)\
While wearing this pendant, you can take a Magic\
action to regain 2d4 + 2 Hit Points. Once used, this\
property can’t be used again until the next dawn.\
In addition, you have Advantage on saving throws\
to avoid or end the Poisoned condition while you\
wear this pendant.\

Original layout: [[../../srd-5.2.1/pages/234|page 234]].

## Item

<!-- splat:values path=splat_item -->

| Field | Value |
| --- | --- |
| ` category ` | ` wondrous_item ` |
| ` rarity ` | ` uncommon ` |

### Equipment

<!-- splat:values path=splat_item/equipment -->

| Field | Value |
| --- | --- |
| ` attunement ` | ` required ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:items/periapt_of_health ` |
| ` implementation ` | ` dnd2024:item_periapt_of_health ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` equipment_state ` | ` worn ` |

#### Healing

<!-- splat:yaml path=splat_rule/parameters/healing -->

```yaml
action: magic
resource:
  uses: 1
  recovery: dawn
hit_points:
  dice: 2
  sides: 4
  bonus: 2
```

#### Saving throws

<!-- splat:values path=splat_rule/parameters/saving_throws -->

| Field | Value |
| --- | --- |
| ` advantage ` | ` true ` |
| ` purpose ` | ` ["avoid_poisoned", "end_poisoned"] ` |

#### Attunement

<!-- splat:values path=splat_rule/parameters/attunement -->

| Field | Value |
| --- | --- |
| ` required ` | ` true ` |
