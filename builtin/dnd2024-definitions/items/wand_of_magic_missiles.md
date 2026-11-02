---
splat_schema: 1
splat_id: 131ad305-433e-5c2b-b948-986c734f888c
splat_kind: item
splat_key: dnd2024:items/wand_of_magic_missiles
splat_name: Wand of Magic Missiles
splat_origin:
  key: dnd2024:reference/magic_item/wand_of_magic_missiles
splat_sources:
- title: SRD 5.2.1
  locator: pp. 251
  url: https://www.dndbeyond.com/srd
---

# Wand of Magic Missiles

## Source text

<!-- SRD page 251 -->

Wand, Uncommon

This wand has 7 charges. While holding it, you can expend no more than 3 charges to cast Magic Missile from it. For 1 charge, you cast the level 1 version of the spell. You can increase the spell’s level by 1 for each additional charge you expend.

Regaining Charges. The wand regains 1d6 + 1 expended charges daily at dawn. If you expend the wand’s last charge, roll 1d20. On a 1, the wand crumbles into ashes and is destroyed.

Original layout: [[../../srd-5.2.1/pages/251|page 251]].

## Item

<!-- splat:values path=splat_item -->

| Field | Value |
| --- | --- |
| ` category ` | ` wand ` |
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
| ` mechanic ` | ` dnd2024:items/wand_of_magic_missiles ` |
| ` implementation ` | ` dnd2024:item_wand_of_magic_missiles ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` equipped ` | ` held ` |
| ` attunement_required ` | ` false ` |

#### Charges

<!-- splat:yaml path=splat_rule/parameters/charges -->

```yaml
initial: 7
maximum: 7
recovery:
  event: time.dawn
  amount:
    roll:
      count: 1
      sides: 6
      add: 1
  cap_at_maximum: true
last_charge:
  roll:
    roll:
      count: 1
      sides: 20
  'on': 1
  result: destroy_to_ashes
```

#### Spells

<!-- splat:rows path=splat_rule/parameters/spells -->

| Row | cast_level |
| --- | --- |
| ` 0 ` | ` charges_spent ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/0/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/magic_missile ` |

##### Charge choice

<!-- splat:values path=splat_rule/parameters/spells/0/charge_choice -->

| Field | Value |
| --- | --- |
| ` min ` | ` 1 ` |
| ` max ` | ` 3 ` |
