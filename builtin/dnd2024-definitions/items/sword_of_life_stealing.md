---
splat_schema: 2
splat_id: d96beda7-1874-5f6c-861d-3e4f2d84ed2b
splat_kind: item
splat_key: dnd2024:items/sword_of_life_stealing
splat_name: Sword of Life Stealing
splat_origin: dnd2024:reference/magic_item/sword_of_life_stealing
splat_sources:
- SRD 5.2.1, pp. 248, https://www.dndbeyond.com/srd
---

# Sword of Life Stealing


## Item

| Field | Value |
| --- | --- |
| Type | Weapon |
| Rarity | Rare |
| Attunement | Yes |


## Source text

<!-- SRD page 248 -->

Weapon (Glaive, Greatsword, Longsword, Rapier, Scimitar, or Shortsword), Rare (Requires Attunement)

When you attack a creature with this magic weapon and roll a 20 on the d20 for the attack roll, that target takes an extra 15 Necrotic damage if it isn’t a Construct or an Undead, and you gain Temporary Hit Points equal to the amount of Necrotic damage taken.

Original layout: [[../../srd-5.2.1/pages/248|page 248]].





## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:items/sword_of_life_stealing ` |
| ` implementation ` | ` dnd2024:item_sword_of_life_stealing ` |

### Parameters

#### Trigger

<!-- splat:yaml path=splat_rule/parameters/trigger -->

```yaml
event: attack_roll
natural_d20: 20
target:
  kind: creature
  excluded_types:
  - construct
  - undead
```

#### Extra damage

<!-- splat:values path=splat_rule/parameters/extra_damage -->

| Field | Value |
| --- | --- |
| ` amount ` | ` 15 ` |
| ` type ` | ` necrotic ` |

#### Temporary hit points

<!-- splat:values path=splat_rule/parameters/temporary_hit_points -->

| Field | Value |
| --- | --- |
| ` equal_to ` | ` necrotic_damage_taken_by_target ` |

#### Attunement

<!-- splat:values path=splat_rule/parameters/attunement -->

| Field | Value |
| --- | --- |
| ` required ` | ` true ` |
