---
splat_schema: 2
splat_id: e42200ee-5fd6-5f34-aa99-1f3b4d1a5516
splat_kind: item
splat_key: dnd2024:items/potion_of_invisibility
splat_name: Potion of Invisibility
splat_origin: dnd2024:reference/magic_item/potion_of_invisibility
splat_sources:
- SRD 5.2.1, pp. 236, https://www.dndbeyond.com/srd
---

# Potion of Invisibility


## Item

| Field | Value |
| --- | --- |
| Type | Potion |
| Rarity | Rare |
| Attunement | No |


## Source text

<!-- SRD page 236 -->

Potion, Rare

This potion’s container looks empty but feels as though it holds liquid. When you drink the potion, you have the Invisible condition for 1 hour. The effect ends early if you make an attack roll, deal damage, or cast a spell.

Original layout: [[../../srd-5.2.1/pages/236|page 236]].





## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:items/potion_of_invisibility ` |
| ` implementation ` | ` dnd2024:item_potion_of_invisibility ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` trigger ` | ` potion_drunk ` |
| ` consumed ` | ` true ` |
| ` attunement_required ` | ` false ` |

#### Effect

<!-- splat:yaml path=splat_rule/parameters/effect -->

```yaml
condition: invisible
duration:
  amount: 1
  unit: hours
ends_on:
- user.attack_roll
- user.damage_dealt
- user.spell_cast
```
