---
splat_schema: 1
splat_id: c40bfae4-5482-555c-949d-f937c57c1b35
splat_kind: item
splat_key: dnd2024:items/rod_of_resurrection
splat_name: Rod of Resurrection
splat_origin:
  key: dnd2024:reference/magic_item/rod_of_resurrection
splat_sources:
- title: SRD 5.2.1
  locator: pp. 242
  url: https://www.dndbeyond.com/srd
---

# Rod of Resurrection

## Source text

<!-- SRD page 242 -->

Rod, Legendary (Requires Attunement)\
The rod has 5 charges. While you hold it, you can\
cast one of the following spells from it: Heal (ex-\
pends 1 charge) or Resurrection (expends 5 charges).\
The rod regains 1 expended charge daily at dawn.\
If you expend the last charge, roll 1d20. On a 1, the\
rod disappears in a harmless burst of radiance.\

Original layout: [[../../srd-5.2.1/pages/242|page 242]].

## Item

<!-- splat:values path=splat_item -->

| Field | Value |
| --- | --- |
| ` category ` | ` rod ` |
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
| ` mechanic ` | ` dnd2024:items/rod_of_resurrection ` |
| ` implementation ` | ` dnd2024:item_rod_of_resurrection ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` equipment_state ` | ` held ` |

#### Charges

<!-- splat:yaml path=splat_rule/parameters/charges -->

```yaml
maximum: 5
initial: 5
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
| ` 0 ` | ` 1 ` | ` spell_default ` | ` item ` | ` 0 ` | ` lowest_possible ` | ` spell_default ` | ` spell_default ` | ` spell_default ` |
| ` 1 ` | ` 5 ` | ` spell_default ` | ` item ` | ` 0 ` | ` lowest_possible ` | ` spell_default ` | ` spell_default ` | ` spell_default ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spell_grants/0/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/heal ` |

##### Components

<!-- splat:yaml path=splat_rule/parameters/spell_grants/0/components -->

```yaml
[]
```

##### Spell

<!-- splat:values path=splat_rule/parameters/spell_grants/1/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/resurrection ` |

##### Components

<!-- splat:yaml path=splat_rule/parameters/spell_grants/1/components -->

```yaml
[]
```

#### Last charge

<!-- splat:yaml path=splat_rule/parameters/last_charge -->

```yaml
roll:
  expression: 1d20
  scope: use
'on': 1
effect: item_disappears_harmlessly
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
| ` 0 ` | ` dnd2024:rules/magic_item_categories_rods ` |
| ` 1 ` | ` dnd2024:rules/activating_a_magic_item_spells_cast_from_items ` |
