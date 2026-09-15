---
splat_schema: 1
splat_id: ad572d3b-16e7-5478-a402-ec28095e3c6e
splat_kind: item
splat_key: dnd2024:equipment/other_tools_musical_instrument_varies
splat_name: Musical Instrument (Varies)
splat_origin:
  key: dnd2024:reference/pages/094
splat_sources:
- title: SRD 5.2.1
  locator: pp. 94
  url: https://www.dndbeyond.com/srd
---

# Musical Instrument (Varies)

## Source text

<!-- SRD page 94 -->

Ability:  Charisma  Weight: Varies\
Utilize: Play a known tune (DC 10), or improvise a song\
(DC 15)\
Variants: Bagpipes (30 GP, 6 lb.), drum (6 GP, 3 lb.), dul-\
cimer (25 GP, 10 lb.), flute (2 GP, 1 lb.), horn (3 GP, 2\
lb.), lute (35 GP, 2 lb.), lyre (30 GP, 2 lb.), pan flute (12\
GP, 2 lb.), shawm (2 GP, 1 lb.), viol (30 GP, 1 lb.)\

## Item

<!-- splat:values path=splat_item -->

| Field | Value |
| --- | --- |
| ` category ` | ` adventuring_gear ` |

### Equipment

<!-- splat:values path=splat_item/equipment -->

| Field | Value |
| --- | --- |
| ` attunement ` | ` none ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:equipment/other_tools_musical_instrument_varies ` |
| ` implementation ` | ` dnd2024:equipment_other_tools_musical_instrument_varies ` |

### Parameters

#### Tool

<!-- splat:yaml path=splat_rule/parameters/tool -->

```yaml
ability: charisma
proficiency_bonus_if_proficient: true
skill_and_tool_proficiency_grants_advantage: true
utilize:
- action: utilize
  operation: play_known_tune
  check:
    ability: charisma
    dc: 10
- action: utilize
  operation: improvise_song
  check:
    ability: charisma
    dc: 15
craftable: []
monster_proficient_if_tool_in_stat_block: true
proficiency_rule:
  key: dnd2024:rules/tools_tool_proficiency
crafting_rule:
  key: dnd2024:rules/crafting_nonmagical_items_tools
```

#### Variants

<!-- splat:rows path=splat_rule/parameters/variants -->

| Row | id |
| --- | --- |
| ` 0 ` | ` bagpipes ` |
| ` 1 ` | ` drum ` |
| ` 2 ` | ` dulcimer ` |
| ` 3 ` | ` flute ` |
| ` 4 ` | ` horn ` |
| ` 5 ` | ` lute ` |
| ` 6 ` | ` lyre ` |
| ` 7 ` | ` pan_flute ` |
| ` 8 ` | ` shawm ` |
| ` 9 ` | ` viol ` |

##### Price

<!-- splat:values path=splat_rule/parameters/variants/0/price -->

| Field | Value |
| --- | --- |
| ` value ` | ` 30 ` |
| ` unit ` | ` gp ` |

##### Mass

<!-- splat:values path=splat_rule/parameters/variants/0/mass -->

| Field | Value |
| --- | --- |
| ` value ` | ` 6 ` |
| ` unit ` | ` lb ` |

##### Price

<!-- splat:values path=splat_rule/parameters/variants/1/price -->

| Field | Value |
| --- | --- |
| ` value ` | ` 6 ` |
| ` unit ` | ` gp ` |

##### Mass

<!-- splat:values path=splat_rule/parameters/variants/1/mass -->

| Field | Value |
| --- | --- |
| ` value ` | ` 3 ` |
| ` unit ` | ` lb ` |

##### Price

<!-- splat:values path=splat_rule/parameters/variants/2/price -->

| Field | Value |
| --- | --- |
| ` value ` | ` 25 ` |
| ` unit ` | ` gp ` |

##### Mass

<!-- splat:values path=splat_rule/parameters/variants/2/mass -->

| Field | Value |
| --- | --- |
| ` value ` | ` 10 ` |
| ` unit ` | ` lb ` |

##### Price

<!-- splat:values path=splat_rule/parameters/variants/3/price -->

| Field | Value |
| --- | --- |
| ` value ` | ` 2 ` |
| ` unit ` | ` gp ` |

##### Mass

<!-- splat:values path=splat_rule/parameters/variants/3/mass -->

| Field | Value |
| --- | --- |
| ` value ` | ` 1 ` |
| ` unit ` | ` lb ` |

##### Price

<!-- splat:values path=splat_rule/parameters/variants/4/price -->

| Field | Value |
| --- | --- |
| ` value ` | ` 3 ` |
| ` unit ` | ` gp ` |

##### Mass

<!-- splat:values path=splat_rule/parameters/variants/4/mass -->

| Field | Value |
| --- | --- |
| ` value ` | ` 2 ` |
| ` unit ` | ` lb ` |

##### Price

<!-- splat:values path=splat_rule/parameters/variants/5/price -->

| Field | Value |
| --- | --- |
| ` value ` | ` 35 ` |
| ` unit ` | ` gp ` |

##### Mass

<!-- splat:values path=splat_rule/parameters/variants/5/mass -->

| Field | Value |
| --- | --- |
| ` value ` | ` 2 ` |
| ` unit ` | ` lb ` |

##### Price

<!-- splat:values path=splat_rule/parameters/variants/6/price -->

| Field | Value |
| --- | --- |
| ` value ` | ` 30 ` |
| ` unit ` | ` gp ` |

##### Mass

<!-- splat:values path=splat_rule/parameters/variants/6/mass -->

| Field | Value |
| --- | --- |
| ` value ` | ` 2 ` |
| ` unit ` | ` lb ` |

##### Price

<!-- splat:values path=splat_rule/parameters/variants/7/price -->

| Field | Value |
| --- | --- |
| ` value ` | ` 12 ` |
| ` unit ` | ` gp ` |

##### Mass

<!-- splat:values path=splat_rule/parameters/variants/7/mass -->

| Field | Value |
| --- | --- |
| ` value ` | ` 2 ` |
| ` unit ` | ` lb ` |

##### Price

<!-- splat:values path=splat_rule/parameters/variants/8/price -->

| Field | Value |
| --- | --- |
| ` value ` | ` 2 ` |
| ` unit ` | ` gp ` |

##### Mass

<!-- splat:values path=splat_rule/parameters/variants/8/mass -->

| Field | Value |
| --- | --- |
| ` value ` | ` 1 ` |
| ` unit ` | ` lb ` |

##### Price

<!-- splat:values path=splat_rule/parameters/variants/9/price -->

| Field | Value |
| --- | --- |
| ` value ` | ` 30 ` |
| ` unit ` | ` gp ` |

##### Mass

<!-- splat:values path=splat_rule/parameters/variants/9/mass -->

| Field | Value |
| --- | --- |
| ` value ` | ` 1 ` |
| ` unit ` | ` lb ` |
