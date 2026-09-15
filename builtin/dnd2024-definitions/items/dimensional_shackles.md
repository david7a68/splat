---
splat_schema: 1
splat_id: a7e4f139-6804-548c-bcd7-a863aa5b3776
splat_kind: item
splat_key: dnd2024:items/dimensional_shackles
splat_name: Dimensional Shackles
splat_origin:
  key: dnd2024:reference/magic_item/dimensional_shackles
splat_sources:
- title: SRD 5.2.1
  locator: pp. 218
  url: https://www.dndbeyond.com/srd
---

# Dimensional Shackles

## Source text

<!-- SRD page 218 -->

Wondrous Item, Rare\
You can take a Utilize action to place these shackles\
on a creature that has the Incapacitated condition.\
The shackles adjust to fit a creature of Small to\
Large size. The shackles prevent a creature bound\
by them from using any method of extradimen-\
sional movement, including teleportation or travel\
to a different plane of existence. They don’t prevent\
the creature from passing through an interdimen-\
sional portal.\
You and any creature you designate when you\
use the shackles can take a Utilize action to remove\
them. Once every 30 days, the bound creature can\
make a DC 30 Strength (Athletics) check. On a suc-\
cessful check, the creature breaks free and destroys\
the shackles.\

Original layout: [[../../srd-5.2.1/pages/218|page 218]].

## Item

<!-- splat:values path=splat_item -->

| Field | Value |
| --- | --- |
| ` category ` | ` wondrous_item ` |
| ` rarity ` | ` rare ` |

### Equipment

<!-- splat:values path=splat_item/equipment -->

| Field | Value |
| --- | --- |
| ` attunement ` | ` none ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:items/dimensional_shackles ` |
| ` implementation ` | ` dnd2024:item_dimensional_shackles ` |

### Parameters

#### Apply

<!-- splat:values path=splat_rule/parameters/apply -->

| Field | Value |
| --- | --- |
| ` action ` | ` utilize ` |
| ` target_condition ` | ` incapacitated ` |
| ` sizes ` | ` ["small", "medium", "large"] ` |
| ` designate_removers ` | ` true ` |

#### Bound

<!-- splat:values path=splat_rule/parameters/bound -->

| Field | Value |
| --- | --- |
| ` prevent_movement ` | ` ["teleportation", "planar_travel", "other_extradimensional"] ` |
| ` allow ` | ` passing_through_interdimensional_portal ` |

#### Remove

<!-- splat:values path=splat_rule/parameters/remove -->

| Field | Value |
| --- | --- |
| ` action ` | ` utilize ` |
| ` allowed_actors ` | ` ["applier", "designated_creatures"] ` |

#### Break free

<!-- splat:yaml path=splat_rule/parameters/break_free -->

```yaml
interval:
  amount: 30
  unit: day
actor: bound_creature
check:
  ability: strength
  skill: athletics
  dc: 30
success:
  freed: true
  item_destroyed: true
```

#### Attunement

<!-- splat:values path=splat_rule/parameters/attunement -->

| Field | Value |
| --- | --- |
| ` required ` | ` false ` |

#### Common rules

<!-- splat:rows path=splat_rule/parameters/common_rules -->

| Row | key |
| --- | --- |
| ` 0 ` | ` dnd2024:rules/magic_item_categories_wondrous_items ` |
