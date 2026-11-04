---
splat_schema: 2
splat_id: bf03bcc5-313b-5d37-93b7-5dfa68c7e915
splat_kind: item
splat_key: dnd2024:items/periapt_of_wound_closure
splat_name: Periapt of Wound Closure
splat_origin: dnd2024:reference/magic_item/periapt_of_wound_closure
splat_sources:
- SRD 5.2.1, pp. 234, https://www.dndbeyond.com/srd
---

# Periapt of Wound Closure


## Item

| Field | Value |
| --- | --- |
| Type | Wondrous Item |
| Rarity | Uncommon |
| Attunement | Yes |


## Source text

<!-- SRD page 234 -->

Wondrous Item, Uncommon (Requires Attunement)

While wearing this pendant, you gain the following benefits.

Life Preservation. Whenever you make a Death Saving Throw, you can change a roll of 9 or lower to a 10, turning a failed save into a successful one.

Natural Healing Boost. Whenever you roll a Hit Point Die to regain Hit Points, double the number of Hit Points it restores.

Original layout: [[../../srd-5.2.1/pages/234|page 234]].





## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:items/periapt_of_wound_closure ` |
| ` implementation ` | ` dnd2024:item_periapt_of_wound_closure ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` equipped ` | ` worn ` |
| ` attunement_required ` | ` true ` |

#### Death save

<!-- splat:values path=splat_rule/parameters/death_save -->

| Field | Value |
| --- | --- |
| ` optional ` | ` true ` |
| ` roll_max ` | ` 9 ` |
| ` replace_roll ` | ` 10 ` |
| ` result ` | ` success ` |

#### Hit point die healing

<!-- splat:values path=splat_rule/parameters/hit_point_die_healing -->

| Field | Value |
| --- | --- |
| ` multiplier ` | ` 2 ` |
| ` scope ` | ` hit_points_restored_by_die ` |
