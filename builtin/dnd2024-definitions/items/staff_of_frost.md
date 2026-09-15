---
splat_schema: 1
splat_id: e9833cc4-c7a6-5188-a882-956c6797a70a
splat_kind: item
splat_key: dnd2024:items/staff_of_frost
splat_name: Staff of Frost
splat_origin:
  key: dnd2024:reference/magic_item/staff_of_frost
splat_sources:
- title: SRD 5.2.1
  locator: pp. 245
  url: https://www.dndbeyond.com/srd
---

# Staff of Frost

## Source text

<!-- SRD page 245 -->

Staff, Very Rare (Requires Attunement by a Druid,\
Sorcerer, Warlock, or Wizard)\
You have Resistance to Cold damage while you hold\
this staff.\
Spells. The staff has 10 charges. While holding the\
staff, you can cast one of the spells on the following\
table from it, using your spell save DC. The table in-\
dicates how many charges you must expend to cast\
the spell.\
Charge Charge\
Spell Cost Spell Cost\
Cone of Cold 5 Ice Storm 4\
Fog Cloud 1 Wall of Ice 4\
Regaining Charges. The staff regains 1d6 + 4 ex-\
pended charges daily at dawn. If you expend the last\
charge, roll 1d20. On a 1, the staff turns to water\
and is destroyed.\

Original layout: [[../../srd-5.2.1/pages/245|page 245]].

## Item

<!-- splat:values path=splat_item -->

| Field | Value |
| --- | --- |
| ` category ` | ` staff ` |
| ` rarity ` | ` very_rare ` |

### Equipment

<!-- splat:values path=splat_item/equipment -->

| Field | Value |
| --- | --- |
| ` attunement ` | ` required ` |
| ` attunement_requirement_text ` | ` a Druid, Sorcerer, Warlock, or Wizard ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:items/staff_of_frost ` |
| ` implementation ` | ` dnd2024:item_staff_of_frost ` |

### Parameters

<!-- splat:values path=splat_rule/parameters -->

| Field | Value |
| --- | --- |
| ` equipped ` | ` held ` |
| ` attunement_classes ` | ` ["druid", "sorcerer", "warlock", "wizard"] ` |
| ` resistance ` | ` ["cold"] ` |
| ` attunement_required ` | ` true ` |

#### Charges

<!-- splat:yaml path=splat_rule/parameters/charges -->

```yaml
initial: 10
maximum: 10
recovery:
  event: time.dawn
  amount:
    roll:
      count: 1
      sides: 6
      add: 4
  cap_at_maximum: true
last_charge:
  roll:
    roll:
      count: 1
      sides: 20
  'on': 1
  result: turn_to_water_and_destroy
```

#### Spells

<!-- splat:rows path=splat_rule/parameters/spells -->

| Row | charge_cost | save_dc |
| --- | --- | --- |
| ` 0 ` | ` 5 ` | ` user.spell_save_dc ` |
| ` 1 ` | ` 4 ` | ` user.spell_save_dc ` |
| ` 2 ` | ` 1 ` | ` user.spell_save_dc ` |
| ` 3 ` | ` 4 ` | ` user.spell_save_dc ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/0/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/cone_of_cold ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/1/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/ice_storm ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/2/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/fog_cloud ` |

##### Spell

<!-- splat:values path=splat_rule/parameters/spells/3/spell -->

| Field | Value |
| --- | --- |
| ` key ` | ` dnd2024:spells/wall_of_ice ` |
