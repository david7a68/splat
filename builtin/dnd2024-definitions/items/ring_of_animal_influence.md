---
splat_schema: 2
splat_id: b69fb1ef-c538-59b7-a5a5-64ba0ec3e3a2
splat_kind: item
splat_key: dnd2024:items/ring_of_animal_influence
splat_name: Ring of Animal Influence
splat_origin: dnd2024:reference/magic_item/ring_of_animal_influence
splat_sources:
- SRD 5.2.1, pp. 237, https://www.dndbeyond.com/srd
---

# Ring of Animal Influence


## Item

| Field | Value |
| --- | --- |
| Type | Ring |
| Rarity | Rare |
| Attunement | No |


## Source text

<!-- SRD page 237 -->

Ring, Rare

This ring has 3 charges, and it regains 1d3 expended charges daily at dawn. While wearing the ring, you can expend 1 charge to cast one of the following spells (save DC 13) from it:

•  Animal Friendship

•  Fear (affects Beasts only)

•  Speak with Animals

Original layout: [[../../srd-5.2.1/pages/237|page 237]].





## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:items/ring_of_animal_influence ` |
| ` implementation ` | ` dnd2024:item_ring_of_animal_influence ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` equipment_state ` | ` worn_on_finger_or_similar_digit ` |

#### Charges

<!-- splat:yaml path=splat_rule/parameters/charges -->

```yaml
maximum: 3
initial: 3
recovery:
  event: dawn
  amount:
    expression: 1d3
    scope: use
  cap: maximum
```

#### Spell grants

<!-- splat:rows path=splat_rule/parameters/spell_grants -->

| Row | charge_cost | casting_time | method | slot_cost | level | range | duration | concentration | save_dc | target_creature_types |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| ` 0 ` | ` 1 ` | ` spell_default ` | ` item ` | ` 0 ` | ` lowest_possible ` | ` spell_default ` | ` spell_default ` | ` spell_default ` | ` 13 ` |  |
| ` 1 ` | ` 1 ` | ` spell_default ` | ` item ` | ` 0 ` | ` lowest_possible ` | ` spell_default ` | ` spell_default ` | ` spell_default ` | ` 13 ` | ` ["beast"] ` |
| ` 2 ` | ` 1 ` | ` spell_default ` | ` item ` | ` 0 ` | ` lowest_possible ` | ` spell_default ` | ` spell_default ` | ` spell_default ` | ` 13 ` |  |

##### Spell

<!-- splat:values path=splat_rule/parameters/spell_grants/0/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/animal_friendship ` |

##### Components

<!-- splat:yaml path=splat_rule/parameters/spell_grants/0/components -->

```yaml
[]
```

##### Spell

<!-- splat:values path=splat_rule/parameters/spell_grants/1/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/fear ` |

##### Components

<!-- splat:yaml path=splat_rule/parameters/spell_grants/1/components -->

```yaml
[]
```

##### Spell

<!-- splat:values path=splat_rule/parameters/spell_grants/2/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/speak_with_animals ` |

##### Components

<!-- splat:yaml path=splat_rule/parameters/spell_grants/2/components -->

```yaml
[]
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
| ` 0 ` | ` dnd2024:rules/magic_item_categories_rings ` |
| ` 1 ` | ` dnd2024:rules/activating_a_magic_item_spells_cast_from_items ` |
