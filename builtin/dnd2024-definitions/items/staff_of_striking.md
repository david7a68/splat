---
splat_schema: 2
splat_id: 6681752d-18a3-575e-9ad6-34b2d6c70531
splat_kind: item
splat_key: dnd2024:items/staff_of_striking
splat_name: Staff of Striking
splat_origin: dnd2024:reference/magic_item/staff_of_striking
splat_sources:
- SRD 5.2.1, pp. 246, https://www.dndbeyond.com/srd
---

# Staff of Striking


## Item

| Field | Value |
| --- | --- |
| Type | Staff |
| Rarity | Very Rare |
| Attunement | Yes |


## Source text

<!-- SRD page 246 -->

Staff, Very Rare (Requires Attunement)

This staff can be wielded as a magic Quarterstaff that grants a +3 bonus to attack rolls and damage rolls made with it.

The staff has 10 charges. When you hit with a melee attack using it, you can expend up to 3 charges. For each charge you expend, the target takes an extra 1d6 Force damage.

Regaining Charges. The staff regains 1d6 + 4 expended charges daily at dawn. If you expend the last charge, roll 1d20. On a 1, the staff becomes a nonmagical Quarterstaff.

Original layout: [[../../srd-5.2.1/pages/246|page 246]].





## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:items/staff_of_striking ` |
| ` implementation ` | ` dnd2024:item_staff_of_striking ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` magical_weapon ` | ` true ` |
| ` attack_bonus ` | ` 3 ` |
| ` damage_bonus ` | ` 3 ` |
| ` attunement_required ` | ` true ` |

#### Weapon

<!-- splat:values path=splat_rule/parameters/weapon -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:equipment/quarterstaff ` |

#### Charges

<!-- splat:yaml path=splat_rule/parameters/charges -->

```yaml
initial: 10
maximum: 10
recovery:
  event: time.dawn
  amount:
    roll:
      count: 1
      sides: 6
      add: 4
  cap_at_maximum: true
last_charge:
  roll:
    roll:
      count: 1
      sides: 20
  'on': 1
  result: nonmagical_quarterstaff
```

#### On melee hit

<!-- splat:yaml path=splat_rule/parameters/on_melee_hit -->

```yaml
optional_charge_spend:
  minimum: 1
  maximum: 3
extra_damage:
  type: force
  amount:
    roll:
      count: charges_spent
      sides: 6
```
