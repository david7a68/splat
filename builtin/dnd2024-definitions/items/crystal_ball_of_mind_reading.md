---
splat_schema: 2
splat_id: 423f448d-388d-5cc1-9cc0-cbce88f96c2e
splat_kind: item
splat_key: dnd2024:items/crystal_ball_of_mind_reading
splat_name: Crystal Ball of Mind Reading
splat_origin: dnd2024:reference/magic_item/crystal_ball_of_mind_reading
splat_sources:
- SRD 5.2.1, pp. 216, https://www.dndbeyond.com/srd
---

# Crystal Ball of Mind Reading


## Item

| Field | Value |
| --- | --- |
| Type | Wondrous Item |
| Rarity | Legendary |
| Attunement | Yes |


## Source text

<!-- SRD page 216 -->

Wondrous Item, Legendary (Requires Attunement)

While touching this crystal orb, you can cast Scrying (save DC 17) with it. In addition, you can cast Detect Thoughts (save DC 17) targeting creatures you can see within 30 feet of the spell’s sensor. You don’t need to concentrate on this Detect Thoughts spell to maintain it during its duration, but it ends if the Scrying spell ends.

Original layout: [[../../srd-5.2.1/pages/216|page 216]].





## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:items/crystal_ball_of_mind_reading ` |
| ` implementation ` | ` dnd2024:item_crystal_ball_of_mind_reading ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` equipped ` | ` touching ` |
| ` attunement_required ` | ` true ` |

#### Spells

<!-- splat:table
path: splat_rule/parameters/spells
key: Spell
order:
- scrying
- detect_thoughts
columns:
  Spell:
    path: spell
    type: reference
  Save dc:
    path: save_dc
    type: integer
  Usage:
    path: usage
    type: symbol
  Targets visible to actor:
    path: targets/visible_to_actor
    type: boolean
    optional: true
  Targets maximum distance from scrying sensor ft:
    path: targets/maximum_distance_from_scrying_sensor_ft
    type: integer
    optional: true
  Concentration:
    path: concentration
    type: boolean
    optional: true
  Ends on:
    path: ends_on
    type: symbol
    optional: true
-->

| Spell | Save dc | Usage | Targets visible to actor | Targets maximum distance from scrying sensor ft | Concentration | Ends on |
| --- | --- | --- | --- | --- | --- | --- |
| [Scrying](<../spells/scrying.md> "dnd2024:spells/scrying") | ` 17 ` | ` At will ` |  |  |  |  |
| [Detect Thoughts](<../spells/detect_thoughts.md> "dnd2024:spells/detect_thoughts") | ` 17 ` | ` At will ` | true | ` 30 ` | false | ` Linked scrying ends ` |
