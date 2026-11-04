---
splat_schema: 2
splat_id: 7aaa5e38-3e00-595b-93f8-40c0e540003d
splat_kind: item
splat_key: dnd2024:items/dust_of_disappearance
splat_name: Dust of Disappearance
splat_origin: dnd2024:reference/magic_item/dust_of_disappearance
splat_sources:
- SRD 5.2.1, pp. 219, https://www.dndbeyond.com/srd
---

# Dust of Disappearance


## Item

| Field | Value |
| --- | --- |
| Type | Wondrous Item |
| Rarity | Uncommon |
| Attunement | No |


## Source text

<!-- SRD page 219 -->

Wondrous Item, Uncommon

This powder resembles fine sand. There is enough of it for one use. When you take a Utilize action to throw the dust into the air, you and each creature and object within a 10-foot Emanation originating from you have the Invisible condition for 2d4 minutes. The duration is the same for all subjects, and the dust is consumed when its magic takes effect. Immediately after an affected creature makes an attack roll, deals damage, or casts a spell, the Invisible condition ends for that creature.

Original layout: [[../../srd-5.2.1/pages/219|page 219]].





## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:items/dust_of_disappearance ` |
| ` implementation ` | ` dnd2024:item_dust_of_disappearance ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` action ` | ` utilize ` |
| ` consumed_on ` | ` effect_start ` |
| ` attunement_required ` | ` false ` |

#### Area

<!-- splat:values path=splat_rule/parameters/area -->

| Field | Value |
| --- | --- |
| ` shape ` | ` emanation ` |
| ` radius_ft ` | ` 10 ` |
| ` origin ` | ` actor ` |
| ` targets ` | ` ["actor", "creatures", "objects"] ` |

#### Effect

<!-- splat:yaml path=splat_rule/parameters/effect -->

```yaml
condition: invisible
duration:
  amount:
    roll:
      count: 2
      sides: 4
  unit: minutes
duration_roll_shared: true
ends_for_each_creature_immediately_after:
- attack_roll
- damage_dealt
- spell_cast
```
