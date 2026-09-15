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

Wondrous Item, Rare (Silver or Brass), Very Rare\
(Bronze), or Legendary (Iron)\
You can take a Magic action to blow this horn. In\
response, warrior spirits from the plane of Ysgard\
appear in unoccupied spaces within 60 feet of you.\
Each spirit uses the Berserker stat block and re-\
turns to Ysgard after 1 hour or when it drops to 0\
Hit Points. The spirits look like living, breathing\
warriors, and they have Immunity to the Charmed\
and Frightened conditions. Once you use the horn, it\
can’t be used again until 7 days have passed.\
Four types of Horn of Valhalla are known to ex-\
ist, each made of a different metal. The horn’s type\
determines how many spirits it summons, as well\
as the requirement for its use. The GM chooses the\
horn’s type or determines it randomly by rolling on\
the following table.\
If you blow the horn without meeting its require-\
ment, the summoned spirits attack you. If you meet\
the requirement, they are Friendly to you and your\
allies and follow your commands.\
1d100 Horn Type Spirits Requirement\
01–40 Silver 2 None\
41–75 Brass 3 Proficiency with all\
Simple weapons\
76–90 Bronze 4 Training with all\
Medium armor\
91–00 Iron 5 Proficiency with all\
Martial weapons\

Original layout: [[../../srd-5.2.1/pages/226|page 226]].

## Item

<!-- splat:values path=splat_item -->

| Field | Value |
| --- | --- |
| ` category ` | ` wondrous_item ` |
| ` rarity ` | ` rare ` |

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

<!-- splat:rows path=splat_rule/parameters/variants -->

| Row | min | max | metal | rarity | spirits | requirement |
| --- | --- | --- | --- | --- | --- | --- |
| ` 0 ` | ` 1 ` | ` 40 ` | ` silver ` | ` rare ` | ` 2 ` | ` null ` |
| ` 1 ` | ` 41 ` | ` 75 ` | ` brass ` | ` rare ` | ` 3 ` |  |
| ` 2 ` | ` 76 ` | ` 90 ` | ` bronze ` | ` very_rare ` | ` 4 ` |  |
| ` 3 ` | ` 91 ` | ` 100 ` | ` iron ` | ` legendary ` | ` 5 ` |  |

##### Requirement

<!-- splat:values path=splat_rule/parameters/variants/1/requirement -->

| Field | Value |
| --- | --- |
| ` proficiency ` | ` all_simple_weapons ` |

##### Requirement

<!-- splat:values path=splat_rule/parameters/variants/2/requirement -->

| Field | Value |
| --- | --- |
| ` training ` | ` all_medium_armor ` |

##### Requirement

<!-- splat:values path=splat_rule/parameters/variants/3/requirement -->

| Field | Value |
| --- | --- |
| ` proficiency ` | ` all_martial_weapons ` |

#### Attunement

<!-- splat:values path=splat_rule/parameters/attunement -->

| Field | Value |
| --- | --- |
| ` required ` | ` false ` |
