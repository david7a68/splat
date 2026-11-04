---
splat_schema: 2
splat_id: 0099cb14-73ac-57e3-bd61-9ef53031d680
splat_kind: item
splat_key: dnd2024:equipment/lock_10_gp
splat_name: Lock (10 GP)
splat_origin: dnd2024:reference/pages/098
splat_sources:
- SRD 5.2.1, pp. 98, https://www.dndbeyond.com/srd
---

# Lock (10 GP)


## Item

| Field | Value |
| --- | --- |
| Type | Adventuring Gear |
| Rarity |  |
| Attunement | No |


## Source text

<!-- SRD page 98 -->

A Lock comes with a key. Without the key, a creature can use Thieves’ Tools to pick this Lock with a successful DC 15 Dexterity (Sleight of Hand) check.









## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:equipment/lock_10_gp ` |
| ` implementation ` | ` dnd2024:equipment_lock_10_gp ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` includes_key ` | ` true ` |

#### Pick

<!-- splat:yaml path=splat_rule/parameters/pick -->

```yaml
requires_item:
  key: dnd2024:equipment/other_tools_thieves_tools_25_gp
check:
  ability: dexterity
  skill: sleight_of_hand
  dc: 15
success: unlock
```

#### Key use

<!-- splat:values path=splat_rule/parameters/key_use -->

| Field | Value |
| --- | --- |
| ` matching_key_required ` | ` true ` |
| ` operation ` | ` unlock ` |

#### Mass

<!-- splat:values path=splat_rule/parameters/mass -->

| Field | Value |
| --- | --- |
| ` value ` | ` 1.0 ` |
| ` unit ` | ` lb ` |

> [!note]- Splat data (edit in Splat)
> <!-- splat:yaml path=splat_item_extras -->
> 
> ```yaml
> price:
>   unit: gp
>   value: 10
> mass:
>   unit: lb
>   value: 1.0
> ```

