---
splat_schema: 1
splat_id: 401d65af-0bd4-543e-8af7-905173350417
splat_kind: item
splat_key: dnd2024:items/elemental_gem
splat_name: Elemental Gem
splat_origin:
  key: dnd2024:reference/magic_item/elemental_gem
splat_sources:
- title: SRD 5.2.1
  locator: pp. 220
  url: https://www.dndbeyond.com/srd
---

# Elemental Gem

## Source text

<!-- SRD page 220 -->

Wondrous Item, Uncommon

This gem contains a mote of elemental energy. When you take a Utilize action to break the gem, an elemental is summoned (see “Monsters” for its stat block), and the gem ceases to be magical. The elemental appears in an unoccupied space as close to the broken gem as possible, understands your languages, obeys your commands, and takes its turn immediately after you on your Initiative count. The elemental disappears after 1 hour, when it dies, or when you dismiss it as a Bonus Action. The type of gem determines the elemental, as shown in the following table.

| Gem | Summoned Elemental |
| --- | --- |
| Blue sapphire | Air Elemental |
| Emerald | Water Elemental |
| Red corundum | Fire Elemental |
| Yellow diamond | Earth Elemental |

Original layout: [[../../srd-5.2.1/pages/220|page 220]].

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
| ` attunement ` | ` none ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:items/elemental_gem ` |
| ` implementation ` | ` dnd2024:item_elemental_gem ` |

### Parameters

#### Activation

<!-- splat:yaml path=splat_rule/parameters/activation -->

```yaml
action: utilize
operation: break_gem
item_magic_ends: true
summon:
  creature:
    value_from: variant.creature
  placement:
    unoccupied: true
    selection: nearest_item
  understands: summoner_languages
  commands: obey
  initiative: immediately_after_summoner_same_count
  duration:
    value: 1
    unit: hour
  end_on:
  - death
  - summoner_bonus_action_dismissal
```

#### Variants

<!-- splat:yaml path=splat_rule/parameters/variants -->

```yaml
blue_sapphire:
  creature:
    key: dnd2024:creatures/air_elemental
emerald:
  creature:
    key: dnd2024:creatures/water_elemental
red_corundum:
  creature:
    key: dnd2024:creatures/fire_elemental
yellow_diamond:
  creature:
    key: dnd2024:creatures/earth_elemental
```

#### Attunement

<!-- splat:values path=splat_rule/parameters/attunement -->

| Field | Value |
| --- | --- |
| ` required ` | ` false ` |
