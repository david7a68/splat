---
splat_schema: 1
splat_id: 006b7920-7978-513c-a4b5-c2d7b732bd99
splat_kind: item
splat_key: dnd2024:items/chime_of_opening
splat_name: Chime of Opening
splat_origin:
  key: dnd2024:reference/magic_item/chime_of_opening
splat_sources:
- title: SRD 5.2.1
  locator: pp. 215
  url: https://www.dndbeyond.com/srd
---

# Chime of Opening

## Source text

<!-- SRD page 215 -->

Wondrous Item, Rare

This hollow metal tube measures about 1 foot long and weighs 1 pound. As a Magic action, you can strike the chime to cast Knock. The spell’s customary knocking sound is replaced by the clear, ringing tone of the chime, which is audible out to 300 feet.

The chime can be used 10 times. After the tenth time, it cracks and becomes useless.

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
| ` mechanic ` | ` dnd2024:items/chime_of_opening ` |
| ` implementation ` | ` dnd2024:item_chime_of_opening ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` length_ft ` | ` 1 ` |
| ` weight_lb ` | ` 1 ` |

#### Uses

<!-- splat:values path=splat_rule/parameters/uses -->

| Field | Value |
| --- | --- |
| ` maximum ` | ` 10 ` |
| ` initial ` | ` 10 ` |

#### Spell grants

<!-- splat:rows path=splat_rule/parameters/spell_grants -->

| Row | charge_cost | casting_time | method | slot_cost | level | range | duration | concentration | activation |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| ` 0 ` | ` 1 ` | ` magic ` | ` item ` | ` 0 ` | ` lowest_possible ` | ` spell_default ` | ` spell_default ` | ` spell_default ` | ` strike_chime ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spell_grants/0/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/knock ` |

##### Components

<!-- splat:yaml path=splat_rule/parameters/spell_grants/0/components -->

```yaml
[]
```

##### Sound override

<!-- splat:values path=splat_rule/parameters/spell_grants/0/sound_override -->

| Field | Value |
| --- | --- |
| ` kind ` | ` ringing_tone ` |
| ` audible_ft ` | ` 300 ` |

#### On last use

<!-- splat:values path=splat_rule/parameters/on_last_use -->

| Field | Value |
| --- | --- |
| ` state ` | ` cracked_useless ` |

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
