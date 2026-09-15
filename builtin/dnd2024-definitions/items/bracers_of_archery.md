---
splat_schema: 1
splat_id: 9ee9164a-1319-5110-b960-906302f7f1cd
splat_kind: item
splat_key: dnd2024:items/bracers_of_archery
splat_name: Bracers of Archery
splat_origin:
  key: dnd2024:reference/magic_item/bracers_of_archery
splat_sources:
- title: SRD 5.2.1
  locator: pp. 214
  url: https://www.dndbeyond.com/srd
---

# Bracers of Archery

## Source text

<!-- SRD page 214 -->

Wondrous Item, Uncommon (Requires Attunement)\
While wearing these bracers, you have proficiency\
with the Longbow and Shortbow, and you gain a +2\
bonus to damage rolls made with such weapons.\

Original layout: [[../../srd-5.2.1/pages/214|page 214]].

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
| ` mechanic ` | ` dnd2024:items/bracers_of_archery ` |
| ` implementation ` | ` dnd2024:item_bracers_of_archery ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` equipment_state ` | ` worn ` |

#### Weapon proficiencies

<!-- splat:rows path=splat_rule/parameters/weapon_proficiencies -->

| Row | key |
| --- | --- |
| ` 0 ` | ` dnd2024:equipment/longbow ` |
| ` 1 ` | ` dnd2024:equipment/shortbow ` |

#### Damage bonus

<!-- splat:yaml path=splat_rule/parameters/damage_bonus -->

```yaml
weapons:
- key: dnd2024:equipment/longbow
- key: dnd2024:equipment/shortbow
amount: 2
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
