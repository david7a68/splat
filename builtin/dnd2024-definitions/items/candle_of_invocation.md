---
splat_schema: 2
splat_id: 6082eb9f-8f75-5b5e-9a57-c69b64e83afc
splat_kind: item
splat_key: dnd2024:items/candle_of_invocation
splat_name: Candle of Invocation
splat_origin: dnd2024:reference/magic_item/candle_of_invocation
splat_sources:
- SRD 5.2.1, pp. 214, 215, https://www.dndbeyond.com/srd
---

# Candle of Invocation


## Item

| Field | Value |
| --- | --- |
| Type | Wondrous Item |
| Rarity | Very Rare |
| Attunement | Yes |


## Source text

<!-- SRD page 214 -->

Wondrous Item, Very Rare (Requires Attunement)

This candle’s magic is activated when the candle is lit, which requires a Magic action. After burning for 4 hours, the candle is destroyed. You can snuff it out early for use at a later time. Deduct the time it burned in increments of 1 minute from its total burn time.

While lit, the candle sheds Dim Light in a 30-foot radius. While you are within that light, you have Advantage on D20 Tests. In addition, a Cleric or Druid in the light can cast level 1 spells they have prepared without expending spell slots.

Alternatively, when you light the candle for the first time, you can cast Gate with it. Doing so destroys the candle. The portal created by the spell links to a particular Outer Plane chosen by the GM or determined by rolling on the following table.

<!-- SRD page 215 -->

| 1d100 | Outer Plane | 1d100 | Outer Plane |
| --- | --- | --- | --- |
| 01–05 | Abyss | 55–59 | Gehenna |
| 06–10 | Acheron | 60–64 | Hades |
| 11–17 | Arborea | 65–69 | Limbo |
| 18–25 | Arcadia | 70–77 | Mechanus |
| 26–33 | Beastlands | 78–85 | Mount Celestia |
| 34–41 | Bytopia | 86–90 | Nine Hells |
| 42–46 | Carceri | 91–95 | Pandemonium |
| 47–54 | Elysium | 96–00 | Ysgard |

Original layout: [[../../srd-5.2.1/pages/214|page 214]], [[../../srd-5.2.1/pages/215|page 215]].





## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:items/candle_of_invocation ` |
| ` implementation ` | ` dnd2024:item_candle_of_invocation ` |

### Parameters

#### Activation

<!-- splat:values path=splat_rule/parameters/activation -->

| Field | Value |
| --- | --- |
| ` action ` | ` magic ` |
| ` operation ` | ` light ` |

#### Burn

<!-- splat:yaml path=splat_rule/parameters/burn -->

```yaml
total:
  value: 4
  unit: hour
accounting_increment:
  value: 1
  unit: minute
snuff_and_resume: true
on_exhaustion: destroy_item
```

#### Lit effects

<!-- splat:yaml path=splat_rule/parameters/lit_effects -->

```yaml
light:
  kind: dim
  radius_ft: 30
attuned_user_in_light:
  d20_tests: advantage
cleric_or_druid_in_light:
  prepared_spells_level: 1
  spell_slot_cost: 0
```

#### First lighting alternative

<!-- splat:values path=splat_rule/parameters/first_lighting_alternative -->

| Field | Value |
| --- | --- |
| ` destroys_item ` | ` true ` |

##### Cast

<!-- splat:yaml path=splat_rule/parameters/first_lighting_alternative/cast -->

```yaml
spell:
  key: dnd2024:spells/gate
```

##### Portal destination

<!-- splat:values path=splat_rule/parameters/first_lighting_alternative/portal_destination -->

| Field | Value |
| --- | --- |
| ` plane_selection ` | ` ["gm_choice", "d100"] ` |

###### Table

<!-- splat:table
path: splat_rule/parameters/first_lighting_alternative/portal_destination/table
key: Key
order:
- row_1
- row_2
- row_3
- row_4
- row_5
- row_6
- row_7
- row_8
- row_9
- row_10
- row_11
- row_12
- row_13
- row_14
- row_15
- row_16
columns:
  Key:
    type: key
  Roll:
    path: ''
    type: range
  Plane:
    path: plane
    type: symbol
-->

| Key | Roll | Plane |
| --- | --- | --- |
| ` row_1 ` | ` 1–5 ` | ` Abyss ` |
| ` row_2 ` | ` 6–10 ` | ` Acheron ` |
| ` row_3 ` | ` 11–17 ` | ` Arborea ` |
| ` row_4 ` | ` 18–25 ` | ` Arcadia ` |
| ` row_5 ` | ` 26–33 ` | ` Beastlands ` |
| ` row_6 ` | ` 34–41 ` | ` Bytopia ` |
| ` row_7 ` | ` 42–46 ` | ` Carceri ` |
| ` row_8 ` | ` 47–54 ` | ` Elysium ` |
| ` row_9 ` | ` 55–59 ` | ` Gehenna ` |
| ` row_10 ` | ` 60–64 ` | ` Hades ` |
| ` row_11 ` | ` 65–69 ` | ` Limbo ` |
| ` row_12 ` | ` 70–77 ` | ` Mechanus ` |
| ` row_13 ` | ` 78–85 ` | ` Mount celestia ` |
| ` row_14 ` | ` 86–90 ` | ` Nine hells ` |
| ` row_15 ` | ` 91–95 ` | ` Pandemonium ` |
| ` row_16 ` | ` 96–100 ` | ` Ysgard ` |

#### Attunement

<!-- splat:values path=splat_rule/parameters/attunement -->

| Field | Value |
| --- | --- |
| ` required ` | ` true ` |
