---
splat_schema: 1
splat_id: aee61ca4-d6e7-5870-9535-bfa3fdbf2dd2
splat_kind: item
splat_key: dnd2024:items/wand_of_fireballs
splat_name: Wand of Fireballs
splat_origin:
  key: dnd2024:reference/magic_item/wand_of_fireballs
splat_sources:
- title: SRD 5.2.1
  locator: pp. 250, 251
  url: https://www.dndbeyond.com/srd
---

# Wand of Fireballs

## Source text

<!-- SRD page 250 -->

Wand, Rare (Requires Attunement by a Spellcaster)

This wand has 7 charges. While holding it, you can expend no more than 3 charges to cast Fireball (save DC 15) from it. For 1 charge, you cast the level 3 version of the spell. You can increase the spell’s level by 1 for each additional charge you expend.

<!-- SRD page 251 -->

Regaining Charges. The wand regains 1d6 + 1 expended charges daily at dawn. If you expend the wand’s last charge, roll 1d20. On a 1, the wand crumbles into ashes and is destroyed.

Original layout: [[../../srd-5.2.1/pages/250|page 250]], [[../../srd-5.2.1/pages/251|page 251]].

## Item

<!-- splat:values path=splat_item -->

| Field | Value |
| --- | --- |
| ` category ` | ` wand ` |
| ` rarity ` | ` rare ` |
| ` stackable ` | ` false ` |

### Equipment

<!-- splat:values path=splat_item/equipment -->

| Field | Value |
| --- | --- |
| ` allowed ` | ` ["carried", "wielded"] ` |
| ` attunement ` | ` required ` |
| ` attunement_requirement ` | ` spellcaster ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:items/wand_of_fireballs ` |
| ` implementation ` | ` dnd2024:item_wand_of_fireballs ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` equipped ` | ` held ` |
| ` attunement_requires ` | ` spellcaster ` |
| ` attunement_required ` | ` true ` |

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

| Row | save_dc |
| --- | --- |
| ` 0 ` | ` 15 ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/0/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/fireball ` |

##### Charge choice

<!-- splat:values path=splat_rule/parameters/spells/0/charge_choice -->

| Field | Value |
| --- | --- |
| ` min ` | ` 1 ` |
| ` max ` | ` 3 ` |

##### Cast level

<!-- splat:values path=splat_rule/parameters/spells/0/cast_level -->

| Field | Value |
| --- | --- |
| ` add ` | ` [2, "charges_spent"] ` |
