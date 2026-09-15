---
splat_schema: 1
splat_id: adf66340-9c97-5579-95aa-fe5f626a86db
splat_kind: item
splat_key: dnd2024:items/talisman_of_the_sphere
splat_name: Talisman of the Sphere
splat_origin:
  key: dnd2024:reference/magic_item/talisman_of_the_sphere
splat_sources:
- title: SRD 5.2.1
  locator: pp. 249
  url: https://www.dndbeyond.com/srd
---

# Talisman of the Sphere

## Source text

<!-- SRD page 249 -->

Wondrous Item, Legendary (Requires Attunement)\
While holding or wearing this talisman, you have\
Advantage on any Intelligence (Arcana) check you\
make to control a Sphere of Annihilation. In addition,\
when you start your turn in control of a Sphere of\
Annihilation, you can take a Magic action to move it\
10 feet plus a number of additional feet equal to 10\
times your Intelligence modifier. This movement\
doesn’t have to be in a straight line.\

Original layout: [[../../srd-5.2.1/pages/249|page 249]].

## Item

<!-- splat:values path=splat_item -->

| Field | Value |
| --- | --- |
| ` category ` | ` wondrous_item ` |
| ` rarity ` | ` legendary ` |

### Equipment

<!-- splat:values path=splat_item/equipment -->

| Field | Value |
| --- | --- |
| ` attunement ` | ` required ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:items/talisman_of_the_sphere ` |
| ` implementation ` | ` dnd2024:item_talisman_of_the_sphere ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` equipment_state_any ` | ` ["held", "worn"] ` |

#### Check advantage

<!-- splat:yaml path=splat_rule/parameters/check_advantage -->

```yaml
ability: intelligence
skill: arcana
task: control_sphere_of_annihilation
item:
  key: dnd2024:items/sphere_of_annihilation
```

#### Move sphere

<!-- splat:yaml path=splat_rule/parameters/move_sphere -->

```yaml
requires: controlling_sphere_at_turn_start
action: magic
distance_ft:
  op: add
  args:
  - 10
  - op: multiply
    args:
    - 10
    - stat:
        of: actor
        key: intelligence_modifier
straight_line_required: false
```

#### Attunement

<!-- splat:values path=splat_rule/parameters/attunement -->

| Field | Value |
| --- | --- |
| ` required ` | ` true ` |

#### Common rules

<!-- splat:rows path=splat_rule/parameters/common_rules -->

| Row | key |
| --- | --- |
| ` 0 ` | ` dnd2024:rules/magic_item_categories_wondrous_items ` |
