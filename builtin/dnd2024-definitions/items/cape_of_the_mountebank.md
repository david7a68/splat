---
splat_schema: 1
splat_id: eac6529b-e8f1-5510-b331-9b08d6893970
splat_kind: item
splat_key: dnd2024:items/cape_of_the_mountebank
splat_name: Cape of the Mountebank
splat_origin:
  key: dnd2024:reference/magic_item/cape_of_the_mountebank
splat_sources:
- title: SRD 5.2.1
  locator: pp. 215
  url: https://www.dndbeyond.com/srd
---

# Cape of the Mountebank

## Source text

<!-- SRD page 215 -->

Wondrous Item, Rare\
This cape smells faintly of brimstone. While wear-\
ing it, you can use it to cast Dimension Door as a\
Magic action. This property can’t be used again\
until the next dawn.\
When you teleport with that spell, you leave be-\
hind a cloud of smoke. The space you left is Lightly\
Obscured by that smoke until the end of your\
next turn.\

Original layout: [[../../srd-5.2.1/pages/215|page 215]].

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
| ` mechanic ` | ` dnd2024:items/cape_of_the_mountebank ` |
| ` implementation ` | ` dnd2024:item_cape_of_the_mountebank ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` equipment_state ` | ` worn ` |

#### Uses

<!-- splat:yaml path=splat_rule/parameters/uses -->

```yaml
maximum: 1
initial: 1
recovery:
  event: dawn
  amount:
    expression: '1'
    scope: use
  cap: maximum
```

#### Spell grants

<!-- splat:rows path=splat_rule/parameters/spell_grants -->

| Row | charge_cost | casting_time | method | slot_cost | level | range | duration | concentration |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| ` 0 ` | ` 1 ` | ` magic ` | ` item ` | ` 0 ` | ` lowest_possible ` | ` spell_default ` | ` spell_default ` | ` spell_default ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spell_grants/0/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/dimension_door ` |

##### Components

<!-- splat:yaml path=splat_rule/parameters/spell_grants/0/components -->

```yaml
[]
```

#### On teleport

<!-- splat:yaml path=splat_rule/parameters/on_teleport -->

```yaml
origin_space:
  obscurity: lightly_obscured
  duration:
    until: end_of_user_next_turn
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
| ` 1 ` | ` dnd2024:rules/activating_a_magic_item_spells_cast_from_items ` |
