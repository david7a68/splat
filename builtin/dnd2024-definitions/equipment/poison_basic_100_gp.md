---
splat_schema: 2
splat_id: 38f61356-3ddd-523a-8a52-548a6cdfe034
splat_kind: item
splat_key: dnd2024:equipment/poison_basic_100_gp
splat_name: Poison, Basic (100 GP)
splat_origin: dnd2024:reference/pages/099
splat_sources:
- SRD 5.2.1, pp. 99, https://www.dndbeyond.com/srd
---

# Poison, Basic (100 GP)


## Item

| Field | Value |
| --- | --- |
| Type | Adventuring Gear |
| Rarity |  |
| Attunement | No |


## Source text

<!-- SRD page 99 -->

As a Bonus Action, you can use a vial of Basic Poison to coat one weapon or up to three pieces of ammunition. A creature that takes Piercing or Slashing damage from the poisoned weapon or ammunition takes an extra 1d4 Poison damage. Once applied, the poison retains potency for 1 minute or until its damage is dealt, whichever comes first.









## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:equipment/poison_basic_100_gp ` |
| ` implementation ` | ` dnd2024:equipment_poison_basic_100_gp ` |

### Parameters

#### Apply

<!-- splat:values path=splat_rule/parameters/apply -->

| Field | Value |
| --- | --- |
| ` economy ` | ` bonus_action ` |
| ` consume ` | ` 1 ` |
| ` early_expiry ` | ` poison_damage_dealt ` |

##### Target choices

<!-- splat:table
path: splat_rule/parameters/apply/target_choices
key: Key
order:
- row_1
- row_2
columns:
  Key:
    type: key
  Kind:
    path: kind
    type: symbol
  Count:
    path: count
    type: integer
    optional: true
  Maximum count:
    path: maximum_count
    type: integer
    optional: true
-->

| Key | Kind | Count | Maximum count |
| --- | --- | --- | --- |
| ` row_1 ` | ` Weapon ` | ` 1 ` |  |
| ` row_2 ` | ` Ammunition ` |  | ` 3 ` |

##### Duration

<!-- splat:values path=splat_rule/parameters/apply/duration -->

| Field | Value |
| --- | --- |
| ` value ` | ` 1 ` |
| ` unit ` | ` minute ` |

##### Trigger

<!-- splat:values path=splat_rule/parameters/apply/trigger -->

| Field | Value |
| --- | --- |
| ` target_kind ` | ` creature ` |
| ` weapon_or_ammunition_damage_type_in ` | ` ["piercing", "slashing"] ` |

##### Extra damage

<!-- splat:yaml path=splat_rule/parameters/apply/extra_damage -->

```yaml
amount:
  dice:
    count: 1
    sides: 4
  bonus: 0
type: poison
```

#### Mass

<!-- splat:values path=splat_rule/parameters/mass -->

| Field | Value |
| --- | --- |
| ` value ` | ` 0.0 ` |
| ` unit ` | ` lb ` |
| ` negligible ` | ` true ` |

> [!note]- Splat data (edit in Splat)
> <!-- splat:yaml path=splat_item_extras -->
> 
> ```yaml
> price:
>   unit: gp
>   value: 100
> mass:
>   unit: lb
>   value: 0.0
> ```

