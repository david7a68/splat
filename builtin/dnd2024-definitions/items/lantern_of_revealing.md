---
splat_schema: 1
splat_id: 92510b1f-60d7-5070-bbfd-bb845774eda2
splat_kind: item
splat_key: dnd2024:items/lantern_of_revealing
splat_name: Lantern of Revealing
splat_origin:
  key: dnd2024:reference/magic_item/lantern_of_revealing
splat_sources:
- title: SRD 5.2.1
  locator: pp. 228, 229
  url: https://www.dndbeyond.com/srd
---

# Lantern of Revealing

## Source text

<!-- SRD page 228 -->

Wondrous Item, Uncommon\
While lit, this hooded lantern burns for 6 hours on 1\
pint of oil, shedding Bright Light in a 30-foot radius\
and Dim Light for an additional 30 feet. Invisible\
creatures and objects are visible as long as they are\
in the lantern’s Bright Light. You can take a Utilize\

<!-- SRD page 229 -->

action to lower the hood, reducing the lantern’s light\
to Dim Light in a 5-foot radius.\

Original layout: [[../../srd-5.2.1/pages/228|page 228]], [[../../srd-5.2.1/pages/229|page 229]].

## Item

<!-- splat:values path=splat_item -->

| Field | Value |
| --- | --- |
| ` category ` | ` wondrous_item ` |
| ` rarity ` | ` uncommon ` |

### Equipment

<!-- splat:values path=splat_item/equipment -->

| Field | Value |
| --- | --- |
| ` attunement ` | ` none ` |

## Rule

<!-- splat:values path=splat_rule -->

| Field | Value |
| --- | --- |
| ` mechanic ` | ` dnd2024:items/lantern_of_revealing ` |
| ` implementation ` | ` dnd2024:item_lantern_of_revealing ` |

### Parameters

#### Fuel

<!-- splat:yaml path=splat_rule/parameters/fuel -->

```yaml
oil_pints: 1
burn_duration:
  value: 6
  unit: hour
```

#### Lit

<!-- splat:yaml path=splat_rule/parameters/lit -->

```yaml
bright_radius_ft: 30
additional_dim_ft: 30
reveal_invisible:
  targets:
  - creatures
  - objects
  within: bright_light
```

#### Lower hood

<!-- splat:yaml path=splat_rule/parameters/lower_hood -->

```yaml
action: utilize
replace_light:
  dim_radius_ft: 5
  bright_radius_ft: 0
```

#### Attunement

<!-- splat:values path=splat_rule/parameters/attunement -->

| Field | Value |
| --- | --- |
| ` required ` | ` false ` |
