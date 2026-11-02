---
splat_schema: 1
splat_id: f823b2bb-c413-5344-9ed4-c22b3b4cc961
splat_kind: item
splat_key: dnd2024:items/wand_of_fear
splat_name: Wand of Fear
splat_origin:
  key: dnd2024:reference/magic_item/wand_of_fear
splat_sources:
- title: SRD 5.2.1
  locator: pp. 250
  url: https://www.dndbeyond.com/srd
---

# Wand of Fear

## Source text

<!-- SRD page 250 -->

Wand, Rare (Requires Attunement)

This wand has 7 charges.

Spells. While holding the wand, you can cast one of the spells (save DC 15) on the following table from it. The table indicates how many charges you must expend to cast the spell.

| Spell | Charge Cost |
| --- | --- |
| Command (“flee” or “grovel” only) | 1 |
| Fear (60-foot Cone) | 3 |

Regaining Charges. The wand regains 1d6 + 1 expended charges daily at dawn. If you expend the wand’s last charge, roll 1d20. On a 1, the wand crumbles into ashes and is destroyed.

Original layout: [[../../srd-5.2.1/pages/250|page 250]].

## Item

<!-- splat:values path=splat_item -->

| Field | Value |
| --- | --- |
| ` category ` | ` wand ` |
| ` rarity ` | ` rare ` |

### Equipment

<!-- splat:values path=splat_item/equipment -->

| Field | Value |
| --- | --- |
| ` attunement ` | ` required ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:items/wand_of_fear ` |
| ` implementation ` | ` dnd2024:item_wand_of_fear ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` equipment_state ` | ` held ` |

#### Charges

<!-- splat:yaml path=splat_rule/parameters/charges -->

```yaml
maximum: 7
initial: 7
recovery:
  event: dawn
  amount:
    expression: 1d6+1
    scope: use
  cap: maximum
```

#### Spell grants

<!-- splat:rows path=splat_rule/parameters/spell_grants -->

| Row | charge_cost | casting_time | method | slot_cost | level | range | duration | concentration | save_dc | allowed_commands |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| ` 0 ` | ` 1 ` | ` spell_default ` | ` item ` | ` 0 ` | ` lowest_possible ` | ` spell_default ` | ` spell_default ` | ` spell_default ` | ` 15 ` | ` ["flee", "grovel"] ` |
| ` 1 ` | ` 3 ` | ` spell_default ` | ` item ` | ` 0 ` | ` lowest_possible ` | ` spell_default ` | ` spell_default ` | ` spell_default ` | ` 15 ` |  |

##### Spell

<!-- splat:values path=splat_rule/parameters/spell_grants/0/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/command ` |

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

##### Area override

<!-- splat:values path=splat_rule/parameters/spell_grants/1/area_override -->

| Field | Value |
| --- | --- |
| ` shape ` | ` cone ` |
| ` length_ft ` | ` 60 ` |

#### Last charge

<!-- splat:yaml path=splat_rule/parameters/last_charge -->

```yaml
roll:
  expression: 1d20
  scope: use
'on': 1
effect: destroyed
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
| ` 0 ` | ` dnd2024:rules/magic_item_categories_wands ` |
| ` 1 ` | ` dnd2024:rules/activating_a_magic_item_spells_cast_from_items ` |
