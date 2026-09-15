---
splat_schema: 1
splat_id: 7ab5c16e-4f9b-58c2-a437-4ea7fb3cbc2f
splat_kind: item
splat_key: dnd2024:items/wand_of_binding
splat_name: Wand of Binding
splat_origin:
  key: dnd2024:reference/magic_item/wand_of_binding
splat_sources:
- title: SRD 5.2.1
  locator: pp. 250
  url: https://www.dndbeyond.com/srd
---

# Wand of Binding

## Source text

<!-- SRD page 250 -->

Wand, Rare (Requires Attunement)\
This wand has 7 charges.\
Spells. While holding the wand, you can cast one\
of the spells (save DC 17) on the following table\
from it. The table indicates how many charges you\
must expend to cast the spell.\
Spell Charge\
Cost\
Hold Monster 5\
Hold Person 2\
Regaining Charges. The wand regains 1d6 +\
1 expended charges daily at dawn. If you expend\
the wand’s last charge, roll 1d20. On a 1, the wand\
crumbles into ashes and is destroyed.\

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
| ` mechanic ` | ` dnd2024:items/wand_of_binding ` |
| ` implementation ` | ` dnd2024:item_wand_of_binding ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` equipped ` | ` held ` |
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

<!-- splat:table
path: splat_rule/parameters/spells
key: Spell
order:
- hold_monster
- hold_person
columns:
  Spell:
    path: spell
    type: reference
  Charge cost:
    path: charge_cost
    type: integer
  Save dc:
    path: save_dc
    type: integer
-->

| Spell | Charge cost | Save dc |
| --- | --- | --- |
| [Hold Monster](<../spells/hold_monster.md> "dnd2024:spells/hold_monster") | ` 5 ` | ` 17 ` |
| [Hold Person](<../spells/hold_person.md> "dnd2024:spells/hold_person") | ` 2 ` | ` 17 ` |
