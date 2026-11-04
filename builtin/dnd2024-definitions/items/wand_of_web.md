---
splat_schema: 2
splat_id: 4dcf10d2-e55b-560d-8d33-1a89b6f7fd52
splat_kind: item
splat_key: dnd2024:items/wand_of_web
splat_name: Wand of Web
splat_origin: dnd2024:reference/magic_item/wand_of_web
splat_sources:
- SRD 5.2.1, pp. 251, https://www.dndbeyond.com/srd
---

# Wand of Web


## Item

| Field | Value |
| --- | --- |
| Type | Wand |
| Rarity | Uncommon |
| Attunement | Yes |


## Source text

<!-- SRD page 251 -->

Wand, Uncommon (Requires Attunement by a Spellcaster)

This wand has 7 charges. While holding it, you can expend 1 charge to cast Web (save DC 13) from it.

Regaining Charges. The wand regains 1d6 + 1 expended charges daily at dawn. If you expend the wand’s last charge, roll 1d20. On a 1, the wand crumbles into ashes and is destroyed.

Original layout: [[../../srd-5.2.1/pages/251|page 251]].





## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:items/wand_of_web ` |
| ` implementation ` | ` dnd2024:item_wand_of_web ` |

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

| Row | charge_cost | casting_time | method | slot_cost | level | range | duration | concentration | save_dc |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| ` 0 ` | ` 1 ` | ` spell_default ` | ` item ` | ` 0 ` | ` lowest_possible ` | ` spell_default ` | ` spell_default ` | ` spell_default ` | ` 13 ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spell_grants/0/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/web ` |

##### Components

<!-- splat:yaml path=splat_rule/parameters/spell_grants/0/components -->

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
effect: destroyed
```

#### Attunement

<!-- splat:values path=splat_rule/parameters/attunement -->

| Field | Value |
| --- | --- |
| ` required ` | ` true ` |
| ` requires_spellcaster ` | ` true ` |

#### Common rules

<!-- splat:rows path=splat_rule/parameters/common_rules -->

| Row | key |
| --- | --- |
| ` 0 ` | ` dnd2024:rules/magic_item_categories_wands ` |
| ` 1 ` | ` dnd2024:rules/activating_a_magic_item_spells_cast_from_items ` |

> [!note]- Splat data (edit in Splat)
> <!-- splat:yaml path=splat_item_extras -->
> 
> ```yaml
> equipment:
>   attunement_requirement_text: a Spellcaster
> ```

