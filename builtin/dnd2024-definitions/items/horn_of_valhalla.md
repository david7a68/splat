---
splat_schema: 1
splat_id: 5f147a6f-13c7-5507-8da3-e355a113d025
splat_kind: item
splat_key: dnd2024:items/horn_of_valhalla
splat_name: Horn of Valhalla
splat_origin:
  key: dnd2024:reference/magic_item/horn_of_valhalla
splat_sources:
- title: SRD 5.2.1
  locator: pp. 226
  url: https://www.dndbeyond.com/srd
---

# Horn of Valhalla

## Source text

<!-- SRD page 226 -->

Wondrous Item, Rare (Silver or Brass), Very Rare (Bronze), or Legendary (Iron)

You can take a Magic action to blow this horn. In response, warrior spirits from the plane of Ysgard appear in unoccupied spaces within 60 feet of you. Each spirit uses the Berserker stat block and returns to Ysgard after 1 hour or when it drops to 0 Hit Points. The spirits look like living, breathing warriors, and they have Immunity to the Charmed and Frightened conditions. Once you use the horn, it can’t be used again until 7 days have passed.

Four types of Horn of Valhalla are known to exist, each made of a different metal. The horn’s type determines how many spirits it summons, as well as the requirement for its use. The GM chooses the horn’s type or determines it randomly by rolling on the following table.

If you blow the horn without meeting its requirement, the summoned spirits attack you. If you meet the requirement, they are Friendly to you and your allies and follow your commands.

| 1d100 | Horn Type | Spirits | Requirement |
| --- | --- | --- | --- |
| 01–40 | Silver | 2 | None |
| 41–75 | Brass | 3 | Proficiency with all Simple weapons |
| 76–90 | Bronze | 4 | Training with all Medium armor |
| 91–00 | Iron | 5 | Proficiency with all Martial weapons |

Original layout: [[../../srd-5.2.1/pages/226|page 226]].

## Item

<!-- splat:values path=splat_item -->

| Field | Value |
| --- | --- |
| ` category ` | ` wondrous_item ` |
| ` rarity ` | ` varies ` |

### Equipment

<!-- splat:values path=splat_item/equipment -->

| Field | Value |
| --- | --- |
| ` attunement ` | ` none ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:items/horn_of_valhalla ` |
| ` implementation ` | ` dnd2024:item_horn_of_valhalla ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` variant_selection ` | ` ["gm_choice", "d100"] ` |

#### Activation

<!-- splat:yaml path=splat_rule/parameters/activation -->

```yaml
action: magic
operation: blow_horn
recovery:
  value: 7
  unit: day
summon:
  creature:
    key: dnd2024:creatures/berserker
  count_from: variant.spirits
  placement:
    unoccupied: true
    range_ft: 60
  origin_plane: ysgard
  duration:
    value: 1
    unit: hour
  return_on:
  - duration
  - hp_zero
  condition_immunities:
  - charmed
  - frightened
  requirements_met:
    friendly_to:
    - user
    - allies
    commands: obey
  requirements_not_met:
    attack: user
```

#### Variants

<!-- splat:table
path: splat_rule/parameters/variants
key: Key
order:
- row_1
- row_2
- row_3
- row_4
columns:
  Key:
    type: key
  Roll:
    path: ''
    type: range
  Metal:
    path: metal
    type: symbol
  Rarity:
    path: rarity
    type: symbol
  Spirits:
    path: spirits
    type: integer
  Requirement proficiency:
    path: requirement/proficiency
    type: symbol
    optional: true
  Requirement training:
    path: requirement/training
    type: symbol
    optional: true
  Details:
    type: section
    optional: true
-->

| Key | Roll | Metal | Rarity | Spirits | Requirement proficiency | Requirement training | Details |
| --- | --- | --- | --- | --- | --- | --- | --- |
| ` row_1 ` | ` 1–40 ` | ` Silver ` | ` Rare ` | ` 2 ` |  |  | [Rules](#^splat-d2ccac60-row_1 "section:row_1") |
| ` row_2 ` | ` 41–75 ` | ` Brass ` | ` Rare ` | ` 3 ` | ` All simple weapons ` |  |  |
| ` row_3 ` | ` 76–90 ` | ` Bronze ` | ` Very rare ` | ` 4 ` |  | ` All medium armor ` |  |
| ` row_4 ` | ` 91–100 ` | ` Iron ` | ` Legendary ` | ` 5 ` | ` All martial weapons ` |  |  |

##### Row 1

^splat-d2ccac60-row_1

###### Requirement

<!-- splat:yaml path=splat_rule/parameters/variants/@row_1/requirement -->

```yaml
null
```

#### Attunement

<!-- splat:values path=splat_rule/parameters/attunement -->

| Field | Value |
| --- | --- |
| ` required ` | ` false ` |
