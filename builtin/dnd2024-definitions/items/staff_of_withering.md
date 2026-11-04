---
splat_schema: 2
splat_id: 3aff3905-62ec-5e33-8fca-66e81225000a
splat_kind: item
splat_key: dnd2024:items/staff_of_withering
splat_name: Staff of Withering
splat_origin: dnd2024:reference/magic_item/staff_of_withering
splat_sources:
- SRD 5.2.1, pp. 248, https://www.dndbeyond.com/srd
---

# Staff of Withering


## Item

| Field | Value |
| --- | --- |
| Type | Staff |
| Rarity | Rare |
| Attunement | Yes |


## Source text

<!-- SRD page 248 -->

Staff, Rare (Requires Attunement)

This staff has 3 charges and regains 1d3 expended charges daily at dawn.

The staff can be wielded as a magic Quarterstaff. On a hit, it deals damage as a normal Quarterstaff, and you can expend 1 charge to deal an extra 2d10 Necrotic damage to the target and force it to make a DC 15 Constitution saving throw. On a failed save, the target has Disadvantage for 1 hour on any ability check or saving throw that uses Strength or Constitution.

Original layout: [[../../srd-5.2.1/pages/248|page 248]].





## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:items/staff_of_withering ` |
| ` implementation ` | ` dnd2024:item_staff_of_withering ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` magical_weapon ` | ` true ` |
| ` base_damage ` | ` normal_quarterstaff ` |
| ` attunement_required ` | ` true ` |

#### Weapon

<!-- splat:values path=splat_rule/parameters/weapon -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:equipment/quarterstaff ` |

#### Charges

<!-- splat:yaml path=splat_rule/parameters/charges -->

```yaml
initial: 3
maximum: 3
recovery:
  event: time.dawn
  amount:
    roll:
      count: 1
      sides: 3
  cap_at_maximum: true
```

#### On hit

<!-- splat:yaml path=splat_rule/parameters/on_hit -->

```yaml
optional_charge_cost: 1
damage:
  amount:
    roll:
      count: 2
      sides: 10
  type: necrotic
save:
  ability: constitution
  dc: 15
  failure:
    disadvantage:
      test_kinds:
      - ability_check
      - saving_throw
      abilities:
      - strength
      - constitution
    duration:
      amount: 1
      unit: hours
```
