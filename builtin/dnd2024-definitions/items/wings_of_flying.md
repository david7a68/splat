---
splat_schema: 2
splat_id: 0070177d-b32f-5469-a218-258baba75211
splat_kind: item
splat_key: dnd2024:items/wings_of_flying
splat_name: Wings of Flying
splat_origin: dnd2024:reference/magic_item/wings_of_flying
splat_sources:
- SRD 5.2.1, pp. 253, https://www.dndbeyond.com/srd
---

# Wings of Flying


## Item

| Field | Value |
| --- | --- |
| Type | Wondrous Item |
| Rarity | Rare |
| Attunement | Yes |


## Source text

<!-- SRD page 253 -->

Wondrous Item, Rare (Requires Attunement)

While wearing this cloak, you can take a Magic action to turn the cloak into a pair of wings on your back. The wings lasts for 1 hour or until you end the effect early as a Magic action. The wings give you a Fly Speed of 60 feet. If you are aloft when the wings disappear, you fall. When the wings disappear, you can’t use them again for 1d12 hours.

Original layout: [[../../srd-5.2.1/pages/253|page 253]].





## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:items/wings_of_flying ` |
| ` implementation ` | ` dnd2024:item_wings_of_flying ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` equipped ` | ` worn ` |
| ` attunement_required ` | ` true ` |

#### Activate

<!-- splat:values path=splat_rule/parameters/activate -->

| Field | Value |
| --- | --- |
| ` action ` | ` magic ` |
| ` transform ` | ` cloak_to_wings ` |

#### Effect

<!-- splat:yaml path=splat_rule/parameters/effect -->

```yaml
fly_speed_ft: 60
duration:
  amount: 1
  unit: hours
early_end_action: magic
aloft_on_end: fall
```

#### Cooldown

<!-- splat:yaml path=splat_rule/parameters/cooldown -->

```yaml
starts: wings_disappear
duration:
  amount:
    roll:
      count: 1
      sides: 12
  unit: hours
```
