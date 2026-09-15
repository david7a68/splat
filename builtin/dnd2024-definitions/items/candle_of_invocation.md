---
splat_schema: 1
splat_id: 6082eb9f-8f75-5b5e-9a57-c69b64e83afc
splat_kind: item
splat_key: dnd2024:items/candle_of_invocation
splat_name: Candle of Invocation
splat_origin:
  key: dnd2024:reference/magic_item/candle_of_invocation
splat_sources:
- title: SRD 5.2.1
  locator: pp. 214, 215
  url: https://www.dndbeyond.com/srd
---

# Candle of Invocation

## Source text

<!-- SRD page 214 -->

Wondrous Item, Very Rare (Requires Attunement)\
This candle’s magic is activated when the candle\
is lit, which requires a Magic action. After burning\
for 4 hours, the candle is destroyed. You can snuff\
it out early for use at a later time. Deduct the time\
it burned in increments of 1 minute from its total\
burn time.\
While lit, the candle sheds Dim Light in a 30-foot\
radius. While you are within that light, you have\
Advantage on D20 Tests. In addition, a Cleric or\
Druid in the light can cast level 1 spells they have\
prepared without expending spell slots.\
Alternatively, when you light the candle for the\
first time, you can cast Gate with it. Doing so de-\
stroys the candle. The portal created by the spell\

<!-- SRD page 215 -->

links to a particular Outer Plane chosen by the GM\
or determined by rolling on the following table.\
1d100 Outer Plane 1d100 Outer Plane\
01–05 Abyss 55–59 Gehenna\
06–10 Acheron 60–64 Hades\
11–17 Arborea 65–69 Limbo\
18–25 Arcadia 70–77 Mechanus\
26–33 Beastlands 78–85 Mount Celestia\
34–41 Bytopia 86–90 Nine Hells\
42–46 Carceri 91–95 Pandemonium\
47–54 Elysium 96–00 Ysgard\

Original layout: [[../../srd-5.2.1/pages/214|page 214]], [[../../srd-5.2.1/pages/215|page 215]].

## Item

<!-- splat:values path=splat_item -->

| Field | Value |
| --- | --- |
| ` category ` | ` wondrous_item ` |
| ` rarity ` | ` very_rare ` |

### Equipment

<!-- splat:values path=splat_item/equipment -->

| Field | Value |
| --- | --- |
| ` attunement ` | ` required ` |

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

<!-- splat:yaml path=splat_rule/parameters/first_lighting_alternative -->

```yaml
cast:
  spell:
    key: dnd2024:spells/gate
destroys_item: true
portal_destination:
  plane_selection:
  - gm_choice
  - d100
  table:
  - min: 1
    max: 5
    plane: abyss
  - min: 6
    max: 10
    plane: acheron
  - min: 11
    max: 17
    plane: arborea
  - min: 18
    max: 25
    plane: arcadia
  - min: 26
    max: 33
    plane: beastlands
  - min: 34
    max: 41
    plane: bytopia
  - min: 42
    max: 46
    plane: carceri
  - min: 47
    max: 54
    plane: elysium
  - min: 55
    max: 59
    plane: gehenna
  - min: 60
    max: 64
    plane: hades
  - min: 65
    max: 69
    plane: limbo
  - min: 70
    max: 77
    plane: mechanus
  - min: 78
    max: 85
    plane: mount_celestia
  - min: 86
    max: 90
    plane: nine_hells
  - min: 91
    max: 95
    plane: pandemonium
  - min: 96
    max: 100
    plane: ysgard
```

#### Attunement

<!-- splat:values path=splat_rule/parameters/attunement -->

| Field | Value |
| --- | --- |
| ` required ` | ` true ` |
