---
splat_schema: 2
splat_id: 07492c50-9b13-5c0e-914d-7987f83a9f17
splat_kind: item
splat_key: dnd2024:items/sword_of_wounding
splat_name: Sword of Wounding
splat_origin: dnd2024:reference/magic_item/sword_of_wounding
splat_sources:
- SRD 5.2.1, pp. 248, https://www.dndbeyond.com/srd
---

# Sword of Wounding


## Item

| Field | Value |
| --- | --- |
| Type | Weapon |
| Rarity | Rare |
| Attunement | Yes |


## Source text

<!-- SRD page 248 -->

Weapon (Glaive, Greatsword, Longsword, Rapier, Scimitar, or Shortsword), Rare (Requires Attunement)

When you hit a creature with an attack using this magic weapon, the target takes an extra 2d6 Necrotic damage and must succeed on a DC 15 Constitution saving throw or be unable to regain Hit Points for 1 hour. The target repeats the save at the end of each of its turns, ending the effect on itself on a success.

Original layout: [[../../srd-5.2.1/pages/248|page 248]].





## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:items/sword_of_wounding ` |
| ` implementation ` | ` dnd2024:item_sword_of_wounding ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` attunement_required ` | ` true ` |

#### On creature hit

<!-- splat:yaml path=splat_rule/parameters/on_creature_hit -->

```yaml
damage:
  amount:
    roll:
      count: 2
      sides: 6
  type: necrotic
save:
  ability: constitution
  dc: 15
  failure:
    can_regain_hp: false
    duration:
      amount: 1
      unit: hours
  repeat:
    event: target.turn_end
    success: end_effect
```
