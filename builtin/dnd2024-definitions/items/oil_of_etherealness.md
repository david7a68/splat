---
splat_schema: 2
splat_id: 27ea59e2-4ed9-586f-a6ce-133cf8776339
splat_kind: item
splat_key: dnd2024:items/oil_of_etherealness
splat_name: Oil of Etherealness
splat_origin: dnd2024:reference/magic_item/oil_of_etherealness
splat_sources:
- SRD 5.2.1, pp. 233, https://www.dndbeyond.com/srd
---

# Oil of Etherealness


## Item

| Field | Value |
| --- | --- |
| Type | Potion |
| Rarity | Rare |
| Attunement | No |


## Source text

<!-- SRD page 233 -->

Potion, Rare

One vial of this oil can cover one Medium or smaller creature, along with the equipment it’s wearing and carrying (one additional vial is required for each size category above Medium). Applying the oil takes 10 minutes. The affected creature then gains the effect of the Etherealness spell for 1 hour.

Beads of this cloudy, gray oil form on the outside of its container and quickly evaporate.

Original layout: [[../../srd-5.2.1/pages/233|page 233]].





## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:items/oil_of_etherealness ` |
| ` implementation ` | ` dnd2024:item_oil_of_etherealness ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` covers ` | ` ["creature", "worn_equipment", "carried_equipment"] ` |
| ` consume_applied_vials ` | ` true ` |
| ` attunement_required ` | ` false ` |

#### Application duration

<!-- splat:values path=splat_rule/parameters/application_duration -->

| Field | Value |
| --- | --- |
| ` amount ` | ` 10 ` |
| ` unit ` | ` minutes ` |

#### Vials by target size

<!-- splat:values path=splat_rule/parameters/vials_by_target_size -->

| Field | Value |
| --- | --- |
| ` tiny ` | ` 1 ` |
| ` small ` | ` 1 ` |
| ` medium ` | ` 1 ` |
| ` large ` | ` 2 ` |
| ` huge ` | ` 3 ` |
| ` gargantuan ` | ` 4 ` |

#### Effect

<!-- splat:yaml path=splat_rule/parameters/effect -->

```yaml
spell:
  key: dnd2024:spells/etherealness
duration:
  amount: 1
  unit: hours
```
