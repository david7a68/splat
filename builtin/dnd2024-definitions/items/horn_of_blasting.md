---
splat_schema: 1
splat_id: 917826ed-6fb1-50a9-bf0b-d900c688c292
splat_kind: item
splat_key: dnd2024:items/horn_of_blasting
splat_name: Horn of Blasting
splat_origin:
  key: dnd2024:reference/magic_item/horn_of_blasting
splat_sources:
- title: SRD 5.2.1
  locator: pp. 226
  url: https://www.dndbeyond.com/srd
---

# Horn of Blasting

## Source text

<!-- SRD page 226 -->

Wondrous Item, Rare

You can take a Magic action to blow the horn, which emits a thunderous blast in a 30-foot Cone that is audible out to 600 feet. Each creature in the Cone makes a DC 15 Constitution saving throw. On a failed save, a creature takes 5d8 Thunder damage and has the Deafened condition for 1 minute. On a successful save, a creature takes half as much damage only. Glass or crystal objects in the Cone that aren’t being worn or carried take 10d8 Thunder damage.

Each use of the horn’s magic has a 20 percent chance of causing the horn to explode. The explosion deals 10d6 Force damage to the user and destroys the horn.

Original layout: [[../../srd-5.2.1/pages/226|page 226]].

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
| ` mechanic ` | ` dnd2024:items/horn_of_blasting ` |
| ` implementation ` | ` dnd2024:item_horn_of_blasting ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` action ` | ` magic ` |
| ` activation ` | ` blow_horn ` |
| ` audible_range_ft ` | ` 600 ` |
| ` attunement_required ` | ` false ` |

#### Area

<!-- splat:values path=splat_rule/parameters/area -->

| Field | Value |
| --- | --- |
| ` shape ` | ` cone ` |
| ` size_ft ` | ` 30 ` |

#### Save

<!-- splat:yaml path=splat_rule/parameters/save -->

```yaml
ability: constitution
dc: 15
failure:
  damage:
    amount:
      roll:
        count: 5
        sides: 8
    type: thunder
  condition: deafened
  condition_duration:
    amount: 1
    unit: minutes
success:
  damage_fraction:
  - 1
  - 2
  condition: null
```

#### Objects

<!-- splat:yaml path=splat_rule/parameters/objects -->

```yaml
materials:
- glass
- crystal
worn_or_carried: false
damage:
  amount:
    roll:
      count: 10
      sides: 8
  type: thunder
```

#### Each use explosion

<!-- splat:yaml path=splat_rule/parameters/each_use_explosion -->

```yaml
probability:
- 1
- 5
target: user
damage:
  amount:
    roll:
      count: 10
      sides: 6
  type: force
destroy_item: true
```
