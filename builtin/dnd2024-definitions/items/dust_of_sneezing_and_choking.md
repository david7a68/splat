---
splat_schema: 2
splat_id: e8dc8a9c-9430-5531-ae03-9cc6200909a3
splat_kind: item
splat_key: dnd2024:items/dust_of_sneezing_and_choking
splat_name: Dust of Sneezing and Choking
splat_origin: dnd2024:reference/magic_item/dust_of_sneezing_and_choking
splat_sources:
- SRD 5.2.1, pp. 219, https://www.dndbeyond.com/srd
---

# Dust of Sneezing and Choking


## Item

| Field | Value |
| --- | --- |
| Type | Wondrous Item |
| Rarity | Uncommon |
| Attunement | No |


## Source text

<!-- SRD page 219 -->

Wondrous Item, Uncommon

Found in a small container, this powder resembles Dust of Disappearance, and Identify reveals it to be such. There is enough of it for one use.

As a Utilize action, you can throw the dust into the air, forcing yourself and every creature in a 30-foot Emanation originating from you to make a DC 15 Constitution saving throw. Constructs, Elementals, Oozes, Plants, and Undead succeed on the save automatically.

On a failed save, a creature begins sneezing uncontrollably; it has the Incapacitated condition and is suffocating. The creature repeats the save at the end of each of its turns, ending the effect on itself on a success. The effect also ends on any creature targeted by a Lesser Restoration spell.

Original layout: [[../../srd-5.2.1/pages/219|page 219]].





## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:items/dust_of_sneezing_and_choking ` |
| ` implementation ` | ` dnd2024:item_dust_of_sneezing_and_choking ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` uses ` | ` 1 ` |

#### Identify false result

<!-- splat:values path=splat_rule/parameters/identify_false_result -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:items/dust_of_disappearance ` |

#### Appearance matches

<!-- splat:values path=splat_rule/parameters/appearance_matches -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:items/dust_of_disappearance ` |

#### Activation

<!-- splat:yaml path=splat_rule/parameters/activation -->

```yaml
action: utilize
consume: true
area:
  shape: emanation
  radius_ft: 30
  origin: user
  include_user: true
save:
  ability: constitution
  dc: 15
  failure:
    condition: incapacitated
    suffocating: true
  success:
    effect: none
  automatic_success_types:
  - construct
  - elemental
  - ooze
  - plant
  - undead
```

#### Repeat save

<!-- splat:values path=splat_rule/parameters/repeat_save -->

| Field | Value |
| --- | --- |
| ` event ` | ` target_turn_end ` |
| ` ability ` | ` constitution ` |
| ` dc ` | ` 15 ` |
| ` success ` | ` end_effect ` |

#### End on spell targeting

<!-- splat:rows path=splat_rule/parameters/end_on_spell_targeting -->

| Row | key |
| --- | --- |
| ` 0 ` | ` dnd2024:spells/lesser_restoration ` |

#### Attunement

<!-- splat:values path=splat_rule/parameters/attunement -->

| Field | Value |
| --- | --- |
| ` required ` | ` false ` |

#### Common rules

<!-- splat:rows path=splat_rule/parameters/common_rules -->

| Row | key |
| --- | --- |
| ` 0 ` | ` dnd2024:rules/magic_item_categories_wondrous_items ` |
| ` 1 ` | ` dnd2024:rules/activating_a_magic_item_spells_cast_from_items ` |
