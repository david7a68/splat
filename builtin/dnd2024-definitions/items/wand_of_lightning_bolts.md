---
splat_schema: 1
splat_id: 14d7eb55-c00b-5868-8a8b-f9d3df0830bd
splat_kind: item
splat_key: dnd2024:items/wand_of_lightning_bolts
splat_name: Wand of Lightning Bolts
splat_origin:
  key: dnd2024:reference/magic_item/wand_of_lightning_bolts
splat_sources:
- title: SRD 5.2.1
  locator: pp. 251
  url: https://www.dndbeyond.com/srd
---

# Wand of Lightning Bolts

## Source text

<!-- SRD page 251 -->

Wand, Rare (Requires Attunement by a Spellcaster)

This wand has 7 charges. While holding it, you can expend no more than 3 charges to cast Lightning Bolt (save DC 15) from it. For 1 charge, you cast the level 3 version of the spell. You can increase the spell’s level by 1 for each additional charge you expend.

Regaining Charges. The wand regains 1d6 + 1 expended charges daily at dawn. If you expend the wand’s last charge, roll 1d20. On a 1, the wand crumbles into ashes and is destroyed.

Original layout: [[../../srd-5.2.1/pages/251|page 251]].

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
| ` attunement_requirement_text ` | ` a Spellcaster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:items/wand_of_lightning_bolts ` |
| ` implementation ` | ` dnd2024:item_wand_of_lightning_bolts ` |

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
    dice: 1
    sides: 6
    bonus: 1
```

#### Spells

<!-- splat:rows path=splat_rule/parameters/spells -->

| Row | save_dc |
| --- | --- |
| ` 0 ` | ` 15 ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/0/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/lightning_bolt ` |

##### Charge choice

<!-- splat:values path=splat_rule/parameters/spells/0/charge_choice -->

| Field | Value |
| --- | --- |
| ` minimum ` | ` 1 ` |
| ` maximum ` | ` 3 ` |

##### Spell level

<!-- splat:yaml path=splat_rule/parameters/spells/0/spell_level -->

```yaml
sum:
- 2
- value_from: charges_spent
```

#### Last charge

<!-- splat:yaml path=splat_rule/parameters/last_charge -->

```yaml
roll:
  dice: 1
  sides: 20
  bonus: 0
on_value: 1
effect: destroy_item
```

#### Attunement

<!-- splat:yaml path=splat_rule/parameters/attunement -->

```yaml
required: true
eligibility:
  spellcaster: true
```
